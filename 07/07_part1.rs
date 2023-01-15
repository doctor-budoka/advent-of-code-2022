use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


struct Directory {
    name: String ,
    files: HashMap<String, i32>,
    directories: HashMap<String, Link>,
    parent: Link,
    size: Option<i32>,
}

impl Directory {
    fn new(name: String, parent: Link) -> Directory {
        return Directory {
            name: name.to_string(), 
            files: HashMap::new(), 
            directories: HashMap::new(), 
            parent: parent, 
            size: None
        };
    }

    fn get_size(&mut self) -> i32 {
        return match self.size {
            Some(bytes)=>bytes,
            None=>self._calculate_size(),
        };
    }

    fn _calculate_size(&mut self) -> i32 {
        let mut sum_of_own_files: i32 = 0;
        for (_name, size) in &self.files {
            sum_of_own_files += size;
        }

        let mut sum_of_directories: i32 = 0;
        for (_name, dir) in &self.directories {
            let this_dir_size: i32 = dir.as_ref().unwrap().borrow_mut().get_size();
            sum_of_directories += this_dir_size;
        }
        let full_size:i32 = sum_of_own_files + sum_of_directories;
        self.size = Some(full_size);
        return full_size;
    }
}

type Link = Option<Rc<RefCell<Directory>>>;

struct FileSystem {
    root: Link,
    cwd: Link,
}

impl FileSystem {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Directory::new("/".to_string(), None)));
        Self {root: Some(root), cwd: None}
    }

    fn update_file_system(&mut self, val: String) {
        if val.starts_with("$ cd") {
            let new_dir: String = val.split(" ").collect::<Vec<&str>>()[2].to_string();
            self.cd(new_dir);
        }
        else if val.starts_with("$ ls") {}
        else {
            let ls_item: Vec<&str> = val.split(" ").collect();
            self.process_ls_item(ls_item);
        }
    }

    fn cd(&mut self, dir: String) {
        if dir == "/" {
            self.move_to_root();
        }
        else if dir == ".." {
            self.move_to_parent();
        }
        else {
            self.move_to_child(dir);
        }
    }

    fn process_ls_item(&mut self, ls_item: Vec<&str>) {
        if ls_item[0] == "dir" {
            self.add_dir_to_cwd(ls_item[1].to_string());
        }
        else {
            self.add_file_to_cwd(ls_item[0].to_string(), ls_item[1].parse::<i32>().unwrap());
        }
    }

    fn move_to_root(&mut self) {
        self.cwd = match &self.root {
            Some(root_link) => Some(Rc::clone(&root_link)),
            None=>panic!("Root directory always exists!")
        };
    }

    fn get_cwd_parent(&self) -> Link {
        return Some(Rc::clone(self.cwd.as_ref().unwrap().borrow().parent.as_ref().unwrap()));
    }

    fn move_to_parent(&mut self) {
        self.cwd = match &self.get_cwd_parent() {
            Some(parent_link) => Some(Rc::clone(&parent_link)),
            None=>panic!("Parent directory doesn't exists!")
        };
    }

    fn get_cwd_child(&self, name: String) -> Link {
        return Some(Rc::clone(self.cwd.as_ref().unwrap().borrow().directories.get(&name).unwrap().as_ref().unwrap()))
    }

    fn move_to_child(&mut self, child_name: String) {
        self.cwd = match &self.get_cwd_child(child_name) {
            Some(child_link) => Some(Rc::clone(&child_link)),
            None=>panic!("Directory doesn't exist!")
        };
    }

    fn add_dir_to_cwd(&mut self, dir_name: String) {
        let cwd_to_pass = Some(Rc::clone(&self.cwd.as_ref().unwrap()));
        let new_dir: Link = Some(Rc::new(RefCell::new(Directory::new(dir_name.to_string(), cwd_to_pass))));
        self.cwd.as_ref().unwrap().borrow_mut().directories.entry(dir_name).or_insert(new_dir);
    }

    fn add_file_to_cwd(&mut self, file_name: String, file_size: i32) {
        self.cwd.as_ref().unwrap().borrow_mut().files.entry(file_name).or_insert(file_size);

    }

    fn get_cwd_size(&mut self) -> i32 {
        return self.cwd.as_ref().unwrap().borrow_mut().get_size();
    }
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    
    let mut file_system = FileSystem::new();

    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(val) = line {
                file_system.update_file_system(val);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
