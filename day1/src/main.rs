use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("FATAL: couldn't read file");


    let calories: Vec<u32> = calories_per_elf(contents);

    let max = calories.last()
        .expect("Couldn't get last element of vector");

    println!("Maximum amount of calories carried by a single elf is: {max}");

    println!("Top 3 max amount of calories carried by elves are: ");

    let length = calories.len();
    let mut sum: u32 = 0;
    for i in 1..=3 {
        let top = calories[length - i];
        sum += top;
        println!("#{i}: {top}");
    }
    println!("Total calories carried by top 3 elves: {sum}");
}

fn calories_per_elf(calorie_log: String) -> Vec<u32> {
    let mut collected_calories = Vec::new();
    let mut calories: u32 = 0;
    for line in calorie_log.lines() { 
        if line.is_empty() {
            insertion_sort(&mut collected_calories, calories);
            calories = 0;
            continue;
        }

        calories += line
            .trim()
            .parse::<u32>()
            .expect("FATAL: Reading invalid input (not a number)")
    }

    collected_calories
}

fn insertion_sort(v: &mut Vec<u32>, element: u32) {
    let index: usize = binary_search(v, element);
    let length = v.len();
    if length > 0 && index + 1 >= length && v[length - 1] < element {
        v.push(element);
    } else {
        v.insert(index, element);
    }
}

// returns index of target or index where target should be if not found
fn binary_search(v: &Vec<u32>, target: u32) -> usize {
    if v.is_empty() {
        return 0;
    }

    let mut high = v.len() - 1;
    let mut low = 0;
    while low < high {
        let mid = low + (high - low)/2;
        let val = v[mid];
        if val == target {
            return mid;
        }
        if target > val {
            low = mid + 1;
        }
        if target < val  {
            high = mid;
        }
    }

    low
}

