use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.prod")
        .expect("Should have been able to read the file");

    let sum = part_one(&input);
    println!("Sum (Part one): {sum}");

    let sum = part_two(&input);
    println!("Sum (Part two): {sum}");
}

fn part_one(input: &str) -> usize {
    return input
        .lines()
        .map(|line| { 
            line.split(',')
                .flat_map(|x| x.split('-')) 
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|row| {
             (row[0] <= row[2] && row[1] >= row[3]) ||
                 (row[2] <= row[0] && row[3] >= row[1])
        })
        .count();
}

fn part_two(input: &str) -> usize {
    return input
        .lines()
        .map(|line| {
            line.split(',')
                .flat_map(|x| x.split('-'))
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|row| {
            (row[0] >= row[2] && row[0] <= row[3]) ||
            (row[1] >= row[2] && row[1] <= row[3]) ||
            (row[2] >= row[0] && row[2] <= row[1]) ||
            (row[3] >= row[0] && row[3] <= row[1])
        })
        .count();
}
