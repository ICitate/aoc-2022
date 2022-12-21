use std::{fs, collections::HashSet};

const START_OF_PACKET_CHARS: usize = 4;
const START_OF_MESSAGE_CHARS: usize = 14;

fn main() {
    let input = fs::read_to_string("input.prod")
        .expect("should be able to read input file");

    let start_index = find_start_of_packet(&input, START_OF_PACKET_CHARS)
        .expect("should have been able to find start of packet");
    println!("start index (Part one): {}", start_index);

    let start_index = find_start_of_packet(&input, START_OF_MESSAGE_CHARS)
        .expect("should have been able to find start of packet");
    println!("start index (Part two): {}", start_index);
}

fn find_start_of_packet(input: &str, start_chars: usize) -> Result<usize, String> {
    let bytes = input.as_bytes();

    for i in 0..bytes.len() {
        let mut seen: HashSet<u8> = HashSet::new();
        for byte in bytes.iter().skip(i).take(start_chars) {
            seen.insert(*byte);
        }
        if seen.len() == start_chars {
            return Ok(i + start_chars);
        }
    }

    Err("should have been able to find start of packet".to_string())
}

