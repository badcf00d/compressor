use std::{cmp, vec};

fn find_longest_match(haystack: &[char], needle: &[char]) -> (usize, usize) {
    let mut best_offset: usize = 0;
    let mut best_length: usize = 0;
    let mut test_offset: usize = 0;
    let mut test_length: usize = 0;
    let mut matching = false;

    for (i, test_char) in haystack.iter().enumerate() {
        if *test_char == needle[test_length] {
            if matching == false {
                test_offset = haystack.len() - i;
            }
            test_length += 1;
            matching = true;
        } else {
            if test_length > best_length {
                best_length = test_length;
                best_offset = test_offset;
            }
            test_length = 0;
            test_offset = 0;
            matching = false;
        }

        if (test_length >= needle.len() - 1) || (test_length == haystack.len()) {
            best_length = test_length;
            best_offset = test_offset;
            break;
        }
    }

    return (best_offset, best_length);
}

fn encode(data: &[char]) -> Vec<(u32, u32, char)> {
    let mut output: Vec<(u32, u32, char)> = vec![];
    let mut index = 0;

    while index < data.len() {
        for data in &data[0..index] {
            print!("{data}");
        }
        print!("[{}]", data[index]);
        for data in &data[index + 1..data.len()] {
            print!("{data}");
        }        
        
        let search_buffer_size = cmp::min(index, 5);
        let search_buffer = &data[index - search_buffer_size..index];
        let lookahead = &data[index..cmp::min(index + 5, data.len())];

        println!("\nIndex: {index}, search {:?}, lookahead {:?}", search_buffer, lookahead);
        let (offset, length) = find_longest_match(&search_buffer, lookahead);

        println!("\t{offset} {length} {:?}", &data[index - offset..(index - offset) + length + 1]);

        index += length + 1;
        output.push((offset as u32, length as u32, data[offset + length]))
    }

    return output;
}

fn decode(encoded_tuple: &Vec<(char, u32)>) -> Vec<char> {
    let mut output: Vec<char> = vec![];

    for tuple in encoded_tuple {
        for _ in 0..tuple.1 {
            output.push(tuple.0);
        }
    }

    return output;
}

fn main() {
    let data = ['a', 'a', 'a', 'a', 'a', 'b', 'a', 'a', 'a', 'a'];

    let encoded_data = encode(&data);
    println!("Encode: {:#?}", encoded_data);

    //let decoded_data = decode(&encoded_data);
    //println!("Decode: {:#?}", decoded_data);
}
