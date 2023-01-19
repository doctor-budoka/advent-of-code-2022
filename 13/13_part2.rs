use std::env;
use std::fs;

mod packet_data;
use packet_data::Data;


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

    println!("Separator inds: {}, {}", ind_sep_1, ind_sep_2);
    println!("Product: {}", ind_sep_1 * ind_sep_2);
} 
