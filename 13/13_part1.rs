use std::env;
use std::fs;
use std::cmp::Ordering;


#[derive(PartialOrd, PartialEq, Eq, Debug)]
enum Data {
    Scalar(u32),
    SubPacket(String),
}

impl Data {
    fn from_string(data: String) -> Data {
        if data.starts_with("[") {
            return Data::SubPacket(data[1..data.len()-1].to_string());
        }
        else{
            return Data::Scalar(data.parse::<u32>().unwrap());
        }
    }


    fn compare_string_packets(&self, packet1: String, packet2: String) -> Ordering {
        let data1 = self.string_as_vec_data(packet1);
        let data2 = self.string_as_vec_data(packet2);

        return self.compare_data_vectors(data1, data2);
    }

    fn string_as_vec_data(&self, string: String) -> Vec<Data>  {
        let mut vec_string: Vec<String> = Vec::new();
        let mut square_bracket_nesting = 0;
        let mut inner_string = "".to_string();
        for string_char in string.chars() {
            match string_char {
                '[' => {
                    square_bracket_nesting += 1;
                    inner_string.push(string_char);
                },
                ']' => {
                    square_bracket_nesting -= 1;
                    inner_string.push(string_char);
                },
                ',' => {
                    if square_bracket_nesting == 0 {
                        vec_string.push(inner_string);
                        inner_string = "".to_string();
                    }
                    else {
                        inner_string.push(string_char);
                    }
                },
                other => inner_string.push(other),
            };
        }
        let mut data_vec: Vec<Data> = Vec::new();
        for string in &vec_string {
            data_vec.push(Data::from_string(string.to_string()));
        }
        return data_vec;
    }

    fn compare_data_vectors(&self, data1: Vec<Data>, data2: Vec<Data>) -> Ordering {
        let mut i = 0;
        let len1 = data1.len();
        let len2 = data2.len();
        loop {
            let i_less_than_len1 = i < len1;
            let i_less_than_len2 = i < len2;
            if i_less_than_len1 && i_less_than_len2 {
                if data1[i].cmp(&data2[i]) != Ordering::Equal {
                    return data1[i].cmp(&data2[i]);
                }
            }
            else {
                return i_less_than_len1.cmp(&i_less_than_len2);
            }
            i += 1;
        }
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        return match (&self, other) { 
            (Data::Scalar(num1), Data::Scalar(num2)) => num1.cmp(num2),
            (Data::SubPacket(sb1), Data::SubPacket(sb2)) => self.compare_string_packets(sb1.to_string(), sb2.to_string()),
            (Data::SubPacket(num), Data::Scalar(sb)) => self.compare_string_packets(num.to_string(), sb.to_string()),
            (Data::Scalar(sb), Data::SubPacket(num)) => self.compare_string_packets(sb.to_string(), num.to_string()),
        };
    }
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let mut correct_inds: Vec<u32> = Vec::new();
    let mut ind: u32 = 1;
    for pair in input.split("\n\n").collect::<Vec<&str>>() {
        let pair_vec: Vec<&str> = pair.trim().split("\n").collect();
        let packet_1: Data = Data::from_string(pair_vec[0].trim().to_string());
        let packet_2: Data = Data::from_string(pair_vec[1].trim().to_string());
        if packet_1.cmp(&packet_2) == Ordering::Less {
            println!("Pair {} is in the correct order", ind);
            correct_inds.push(ind);
        }
        ind += 1;
    }
    println!("Sum of correct inds: {}", correct_inds.iter().sum::<u32>());
} 
