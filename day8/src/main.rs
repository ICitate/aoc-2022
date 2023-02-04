use std::fs;

fn main() {
    let input = fs::read_to_string("input.prod").unwrap();

    println!("PT1: visible trees = {}", part_one(&input));
    println!("PT2: max scenic score = {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let mut matrix: Vec<Vec<(bool, &u8)>> = input.lines()
        .map(|line| {
            return line.as_bytes()
                .iter()
                .map(|byte| (false, byte))
                .collect();
        })
        .collect();

    let rows = matrix.len();
    let cols = matrix[0].len();


    // visible from the left
    for row in &mut matrix {
        let mut max_seen = 0;
        for mut elem in row {
            if &max_seen < elem.1 {
                max_seen = *elem.1;
                elem.0 = true;
            }
        }
    }

    // visible from the right
    for i in 0..rows {
        let mut max_seen = 0;
        for j in (0..cols).rev() {
            let mut elem = &mut matrix[i][j];
            if &max_seen < elem.1 {
                max_seen = *elem.1;
                elem.0 = true;
            }
        }
    }

    // visible from the top 
    for i in 0..cols {
        let mut max_seen = 0;
        for j in 0..rows {
            let mut elem = &mut matrix[j][i];
            if &max_seen < elem.1 {
                max_seen = *elem.1;
                elem.0 = true;
            }
        }
    }

    let mut visible_count = 0;

    // visible from the bottom
    for i in 0..cols {
        let mut max_seen = 0;
        for j in (0..rows).rev() {
            let mut elem = &mut matrix[j][i];
            if &max_seen < elem.1 {
                max_seen = *elem.1;
                elem.0 = true;
            }

            // borders are always visible
            if i == 0 || j == 0 || i == cols-1 || j == rows-1 {
                elem.0 = true;
            }

            if elem.0 {
                visible_count += 1;
            }
        }
    }
    return visible_count;
}

fn part_two(input: &str) -> usize {
    let matrix: Vec<Vec<&u8>> = input.lines()
        .map(|line| {
            return line.as_bytes()
                .iter()
                .collect();
        })
        .collect();

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut max_scenic_score = 0;

    for i in 1..rows-1 {
        for j in 1..cols-1 {
            let mut see = (0,0,0,0);

            // right
            let mut pointer = j+1;
            while pointer < cols {
                see.0 += 1;
                if matrix[i][j] <= matrix[i][pointer] {
                    break;
                }
                pointer += 1;
            } 

            // left
            let mut pointer = j-1;

            loop {
                see.1 += 1;
                if matrix[i][j] <= matrix[i][pointer] {
                    break;
                }

                if pointer == 0 {
                    break;
                }
                pointer -= 1;
            } 

            // down 
            let mut pointer = i+1;
            while pointer < rows {
                see.2 += 1;
                if matrix[i][j] <= matrix[pointer][j] {
                    break;
                }
                pointer += 1;
            } 

            // up 
            let mut pointer = i-1;

            loop {
                see.3 += 1;
                if matrix[i][j] <= matrix[pointer][j] {
                    break;
                }

                if pointer == 0 {
                    break;
                }
                pointer -= 1;
            } 

            let scenic_score = see.0 * see.1 * see.2 * see.3;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    return max_scenic_score;
}
