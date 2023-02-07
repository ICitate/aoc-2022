use std::{fs, str::FromStr};

#[derive(Copy, Clone)]
enum Instruction {
    Noop,
    Addx { x: isize },
}

impl Instruction {
    fn cycle_cost(self) -> isize {
        return match self {
            Instruction::Noop => 1,
            Instruction::Addx { x: _ } => 2,
        }
    }

    fn execute<F>(
        self,
        x: &mut isize,
        cycle: &mut isize,
        mut do_in_cycle: F)
        where F: FnMut(&mut isize, &mut isize) {
        for _ in 0..self.cycle_cost() {
            do_in_cycle(x, cycle);
            *cycle += 1;
        }

        match self {
            Instruction::Noop => (),
            Instruction::Addx { x: x_value } => {
                *x += x_value;
            },
        }
    }
}

impl FromStr for Instruction {
    type Err = (); 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }

        if s.starts_with("addx") {
            if let Some((_, number)) = s.split_once(' ') {
                let parsed_number = number.parse::<isize>()
                    .expect("couldn't parse number");

                return Ok(Instruction::Addx{ x: parsed_number });
            }
        }

        return Err(());
    }
}

fn main() {
    let input = fs::read_to_string("input.prod")
        .expect("couldn't read input file");

    println!("PT1 signal strength sum: {}", part_one(&input));
    println!("PT2 output \n{}", part_two(&input));
}


fn part_one(input: &str) -> isize {
    let mut x: isize = 1;
    let mut cycle: isize = 1;
    let mut signal_strengths: Vec<isize> = vec![];

    for line in input.lines() {
        let instruction = Instruction::from_str(line)
            .expect("couldn't parse instruction");
        
        let record_signal_strength = |x: &mut isize, cycle: &mut isize| { 
            signal_strengths.push(signal_strength(x, cycle)) 
        };

        instruction.execute(&mut x, &mut cycle, record_signal_strength)
    }

    return signal_strengths.iter()
        .skip(19)
        .step_by(40)
        .sum();
}

fn signal_strength(x: &isize, cycle: &isize) -> isize {
    return x*cycle;
}

fn part_two(input: &str) -> String {
    let mut x: isize = 1;
    let mut cycle: isize = 1;
    let mut screen = String::new();

    for line in input.lines() {
        let instruction = Instruction::from_str(line)
            .expect("couldn't parse instruction");

        let calculate_screen = |x: &mut isize, cycle: &mut isize| {
            let row_position = (*cycle-1) % 40;
            let char: char; 

            if *x >= row_position-1 && *x <= row_position+1 {
                char = '#';
            } else {
                char = '.';
            }

            screen.push(char);
            if row_position == 39 {
                screen.push('\n');
            }
        };

        instruction.execute(&mut x, &mut cycle, calculate_screen);
    }

    return screen;
}
