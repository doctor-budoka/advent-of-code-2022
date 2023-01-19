use std::env;
use std::fs;
use std::cmp::Ordering;


#[derive(PartialOrd, PartialEq, Eq, Debug)]
enum Data {
    Scalar(u32),
    SubPacket(String),
    EmptyPacket,
}

impl Data {
    fn from_string(data: &String) -> Data {
        if data == "" {
            return Data::EmptyPacket;
        }
        else if data.starts_with("[") {
            return Data::SubPacket(data[1..data.len()-1].to_string());
        } 
        else{
            return Data::Scalar(data.parse::<u32>().unwrap());
        }
    }

    fn to_string(&self) -> String {
        return match &self {
            Data::Scalar(data) => data.to_string(),
            Data::SubPacket(data) => ("[".to_string() + data + &"]".to_string()).to_string(),
            Data::EmptyPacket => "".to_string(),
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
        vec_string.push(inner_string);
        let mut data_vec: Vec<Data> = Vec::new();
        for string in &vec_string {
            data_vec.push(Data::from_string(string));
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
            (Data::EmptyPacket, Data::EmptyPacket) => Ordering::Equal,
            (Data::SubPacket(sb1), Data::SubPacket(sb2)) => self.compare_string_packets(sb1.to_string(), sb2.to_string()),
            (Data::SubPacket(num), Data::Scalar(sb)) => self.compare_string_packets(num.to_string(), sb.to_string()),
            (Data::Scalar(sb), Data::SubPacket(num)) => self.compare_string_packets(sb.to_string(), num.to_string()),
            (Data::EmptyPacket, _) => Ordering::Less,
            (_, Data::EmptyPacket) => Ordering::Greater,
        };
    }
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let file_name = &env_args[1];
    println!("file name is '{}'", file_name);
    let input = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let string_separator_1 = "[[2]]".to_string();
    let string_separator_2 = "[[6]]".to_string();
    let mut packets: Vec<Data> = vec![Data::from_string(&string_separator_1.to_string()), Data::from_string(&string_separator_2.to_string())];
    for str_packet in input.lines().collect::<Vec<&str>>() {
        if str_packet != "".to_string() {
            let packet: Data = Data::from_string(&str_packet.trim().to_string());
            packets.push(packet);
        }
    }
    packets.sort();
    // We'll look for the separators as strings to avoid the recursion needed for Data comparisons
    let mut str_packets: Vec<String> = Vec::new();
    for packet in packets {
        str_packets.push(packet.to_string());
    }
    let ind_sep_1 = str_packets.iter().position(|r| r == &string_separator_1).unwrap() + 1;
    let ind_sep_2 = str_packets.iter().position(|r| r == &string_separator_2).unwrap() + 1;

    println!("{}", str_packets.join("\n"));

    println!("Separator inds: {}, {}", ind_sep_1, ind_sep_2);
    println!("Product: {}", ind_sep_1 * ind_sep_2);
} 
