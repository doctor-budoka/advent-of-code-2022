use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

const TOTAL_SIZE: u32 = 70000000;
const SIZE_NEEDED: u32 = 30000000;


struct Directory {
    _name: String ,
    files: HashMap<String, u32>,
    directories: HashMap<String, Link>,
    parent: Link,
    size: Option<u32>,
}

impl Directory {
    fn new(name: String, parent: Link) -> Directory {
        return Directory {
            _name: name.to_string(), 
            files: HashMap::new(), 
            directories: HashMap::new(), 
            parent: parent, 
            size: None
        };
    }

    fn get_size(&mut self) -> u32 {
        return match self.size {
            Some(bytes)=>bytes,
            None=>self._calculate_size(),
        };
    }

    fn _calculate_size(&mut self) -> u32 {
        let mut sum_of_own_files: u32 = 0;
        for (_name, size) in &self.files {
            sum_of_own_files += size;
        }

        let mut sum_of_directories: u32 = 0;
        for (_name, dir) in &self.directories {
            let this_dir_size: u32 = dir.as_ref().unwrap().borrow_mut().get_size();
            sum_of_directories += this_dir_size;
        }
        let full_size:u32 = sum_of_own_files + sum_of_directories;
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
            self.add_file_to_cwd(ls_item[1].to_string(), ls_item[0].parse::<u32>().unwrap());
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

    fn add_file_to_cwd(&mut self, file_name: String, file_size: u32) {
        self.cwd.as_ref().unwrap().borrow_mut().files.entry(file_name).or_insert(file_size);

    }

    fn get_cwd_size(&mut self) -> u32 {
        return self.cwd.as_ref().unwrap().borrow_mut().get_size();
    }

    fn _get_cwd_name(&mut self) -> String {
        return (&*self.cwd.as_ref().unwrap().borrow()._name).to_string();
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
                file_system.update_file_system(val.trim().to_string());
            }
        }
    }

    file_system.cd("/".to_string());
    let used: u32 = file_system.get_cwd_size();
    let free: u32 = TOTAL_SIZE - used;
    if free > SIZE_NEEDED {
        println!("No need to delete anything!");
    }
    else {
        let size_to_be_deleted: u32 = SIZE_NEEDED - free;
        println!("Amount to be deleted: {}", size_to_be_deleted);
        let dir_to_delete_size = find_smallest_large_enough_file(&mut file_system, size_to_be_deleted);
        println!("Size of required directory: {}", dir_to_delete_size.expect("We should have a file size!"));
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn find_smallest_large_enough_file(fs: &mut FileSystem, lower_bound: u32) -> Option<u32> {
    let mut dir_vec: Vec<String> = Vec::new();
    let current_size = fs.get_cwd_size();
    if current_size < lower_bound {
        return None;
    }

    let mut current_best: Option<u32> = Some(current_size); 
    for (dir_name, _) in &fs.cwd.as_ref().unwrap().borrow_mut().directories {
        dir_vec.push((&dir_name).to_string());
    }

    for dir_name in dir_vec {
        fs.cd(dir_name.to_string());

        let this_best = find_smallest_large_enough_file(fs, lower_bound);
        current_best = get_new_best_from_optionals(current_best, this_best);

        fs.cd("..".to_string());
    }
    return current_best;
}

fn get_new_best_from_optionals(current_best: Option<u32>, new_candidate: Option<u32>) -> Option<u32> {
    return match (current_best, new_candidate) {
        (Some(current_bytes), Some(candidate_bytes)) => if candidate_bytes < current_bytes {new_candidate} else {current_best},
        (None, Some(_)) => new_candidate,
        (_, None) => current_best,
    };
}

