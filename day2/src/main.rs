
fn parse(input: &str) -> Vec<(u32, u32)> {
    return input
        .lines()
        .map(|line| { 
            let round = line.as_bytes();
            ((round[0] - 65) as u32, (round[2] - 88) as u32)
        })
        .collect();
}

fn part_one(input: &[(u32, u32)]) -> u32 {
    input
        .iter()
        .map(|round| {
            let mut score: u32 = 0;
            let theirs = round.0 + 1;
            let ours = round.1 + 1;
            score += ours;
            // draw 
            if theirs == ours {
                score += 3;
            }
            // win
            if ((3 + theirs) - ours) % 3 == 2 {
                score += 6;
            }
            score
        })
        .map(u32::from)
        .sum()
}

fn part_two(input: &[(u32, u32)]) -> u32 {
    input
        .iter()
        .map(|round| {
            // theirs: 0 ROCK, expected: 1 DRAW -> mine: 0
            // theirs: 2 SCISSORS, expected: 2 WIN -> mine: 0
            // theirs: 1 PAPER, expected: 2 WIN -> mine: 2 
            // theirs: 2 SCISSORS, expected: 0 LOSE -> mine: 1 
            let theirs = round.0;
            let expected = round.1;
            let mine = match expected {
                0 => (theirs + 2) % 3,
                1 => theirs,
                2 => (theirs + 1) % 3,
                _ => unreachable!(),
            };

            (mine + 1) + 3 * expected
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.prod");
    let parsed = parse(input);
    let mut score = part_one(&parsed);
    println!("Score (Part 1): {:?}", score);
    score = part_two(&parsed);
    println!("Score (Part 2): {:?}", score);
}

