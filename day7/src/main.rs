use std::fs;

fn main() {
    let input = fs::read_to_string("input.prod")
        .expect("couldn't read input file");

    let result =  part_one(&input);
    println!("result (part_one): {}", result);

    let result =  part_two(&input);
    println!("result (part_two): {}", result);
}

fn part_one(input: &str) -> usize {
    let max_size = 100_000;
    let mut total = 0;

    let mut stack: Vec<(usize, &str)> = vec![(0, "/")];
    for line in input.lines() {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            let dir = &line[5..];

            if dir == ".." {
                let (size, _) = stack.pop()
                    .expect("there must be some element on the stack");

                if size <= max_size {
                    total += size;
                }
                stack.last_mut().unwrap().0 += size;
            } else {
                stack.push((0, dir));
            }
            continue;
        }

        let (size, _) = line.split_once(' ')
            .expect("couldn't split line");

        let size: usize = match size.parse() {
            Ok(size) => size,
            Err(_) =>  continue,
        };

        stack.last_mut().unwrap().0 += size;
    }

    return total;
}

fn part_two(input: &str) -> usize {
    let disk_space = 70000000;
    let target_space = 30000000;

    let mut stack: Vec<(usize, &str)> = vec![(0, "/")];
    let mut directories: Vec<(usize, &str)> = vec![];

    for line in input.lines() {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            let dir = &line[5..];

            if dir == ".." {
                let (size, name) = stack.pop()
                    .expect("there must be some element on the stack");

                directories.push((size, name));
                stack.last_mut().unwrap().0 += size;
            } else {
                stack.push((0, dir));
            }
            continue;
        }

        let (size, _) = line.split_once(' ')
            .expect("couldn't split line");

        let size: usize = match size.parse() {
            Ok(size) => size,
            Err(_) =>  continue,
        };

        stack.last_mut().unwrap().0 += size;
    }

    while stack.len() > 0 {
        let (size, name) = stack.pop().unwrap();
        directories.push((size, name));
        
        if stack.len() > 0 {
            stack.last_mut().unwrap().0 += size;
        }
    }

    let used_space: usize = directories.last().unwrap().0; 
    let required_space_to_delete = target_space - (disk_space - used_space);

    return directories.iter()
        .filter(|(size, _)| size >= &required_space_to_delete)
        .map(|(size, _)| *size)
        .min()
        .unwrap();
}
