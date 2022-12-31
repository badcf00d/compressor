use std::vec;

fn encode(data: &[char]) -> Vec<(char, u32)> {
    let mut output: Vec<(char, u32)> = vec![];
    let mut match_byte = data[0];
    let mut match_len:u32 = 1;

    for byte in data {
        if *byte == match_byte {
            match_len += 1;
        } else {
            output.push((match_byte, match_len));
            match_byte = *byte;
            match_len = 1;
        }
    }
    output.push((match_byte, match_len));
    return output
}

fn decode(encoded_tuple: &Vec<(char,u32)>) -> Vec<char> {
    let mut output: Vec<char> = vec![];

    for tuple in encoded_tuple {
        for _ in 0..tuple.1 {
            output.push(tuple.0);
        }
    }

    return output
}

fn main() {
    let data = ['a', 'a', 'a', 'a', 'a', 'b', 'a', 'a', 'a', 'a'];

    let encoded_data = encode(&data);
    println!("Encode: {:#?}", encoded_data);

    let decoded_data = decode(&encoded_data);
    println!("Decode: {:#?}", decoded_data);
}
