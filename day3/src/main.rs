
fn main() {
    let input = include_str!("../input.prod");
    
    let sum: u32 = part_one(input);
    println!("sum (Part one): {:?}", sum);

    let sum = part_two(input);
    println!("sum (Part two): {:?}", sum);
}

// more declarative version
fn part_one(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| { 
            let bytes = line.as_bytes();
            let length = line.len();
            bytes 
                .iter()
                .find(|item| {
                    bytes[0..length/2].contains(item) &&
                    bytes[length/2..].contains(item)
                })
            .expect("Invalid format")
        })
        .map(to_priority)
        .map(u32::from)
        .sum();
}

fn to_priority(byte: &u8) -> u8 {
    if byte.is_ascii_lowercase() {
       return byte - 96; 
    } else if byte.is_ascii_uppercase() {
        return byte - 38;
    }
    panic!();
}

/* less declarative but works anyway
fn part_one(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| { 
            let bytes = line.as_bytes();
            let length = line.len();
            for i in 0..length/2 {
                let item = bytes[i];
                if bytes[length/2..].contains(&item) {
                    return item;
                }
            } 
            panic!();
        })
        .map(|byte| {
            if byte.is_ascii_lowercase() {
               return byte - 96; 
            } else if byte.is_ascii_uppercase() {
                return byte - 38;
            }
            panic!();
        })
        .map(u32::from)
        .sum();
}

*/

fn part_two(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();

    return lines
        .chunks(3)
        .map(|chunk| {
            let mut chunk_sum = 0;
            let mut occurrences: [u8; 53] = [0; 53];
            for (row_index, sack) in chunk.iter().enumerate() {
                for byte in sack.as_bytes() {
                    let item: usize = to_priority(byte).into();

                    occurrences[item] |= 1 << row_index;
                    if occurrences[item] == 0b111 {
                        chunk_sum += item;
                        break;
                    }
                } 
            }
            return chunk_sum;
        })
        .sum::<usize>();
}
