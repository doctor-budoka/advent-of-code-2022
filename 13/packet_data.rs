use std::cmp::Ordering;


#[derive(PartialOrd, PartialEq, Eq, Debug)]
pub enum Data {
    Scalar(u32),
    SubPacket(String),
    EmptyPacket,
}

impl Data {
    pub fn from_string(data: &String) -> Data {
        if data.starts_with("[") {
            return Data::SubPacket(data.to_string());
        } 
        else if data == "" {
            return Data::EmptyPacket;
        }
        else{
            return Data::Scalar(data.parse::<u32>().unwrap());
        }
    }

    pub fn to_string(&self) -> String {
        return match &self {
            Data::Scalar(data) => data.to_string(),
            Data::SubPacket(data) => data.to_string(),
            Data::EmptyPacket => "".to_string(),
        }
    }

    fn convert_scalar_data_to_packet_data(&self, data: u32) -> String {
        return ("[".to_string() + &data.to_string() + &"]".to_string()).to_string()
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

        for string_char in string[1..string.len()-1].to_string().chars() {
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
                let ordering = data1[i].cmp(&data2[i]);
                if ordering != Ordering::Equal {
                    return ordering;
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
            (Data::EmptyPacket, _) => Ordering::Less,
            (_, Data::EmptyPacket) => Ordering::Greater,
            (Data::SubPacket(sb1), Data::SubPacket(sb2)) => self.compare_string_packets(sb1.to_string(), sb2.to_string()),
            (Data::SubPacket(sb), Data::Scalar(num)) => self.compare_string_packets(sb.to_string(), self.convert_scalar_data_to_packet_data(*num)),
            (Data::Scalar(num), Data::SubPacket(sb)) => self.compare_string_packets(self.convert_scalar_data_to_packet_data(*num), sb.to_string()),
        };
    }
}
