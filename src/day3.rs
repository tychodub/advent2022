use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn day3_part1() {
    let mut input = File::open("inputday3").expect("could not find file");
    let mut text = String::new();
    let mut value_map: HashMap<String,i32> = HashMap::with_capacity(52);
    let mut iteration = 0;
    for char in 'a'..='z' {
        iteration += 1;
        value_map.insert(char.to_string(), iteration);
    }
    for char in 'A'..='Z' {
        iteration += 1;
        value_map.insert(char.to_string(), iteration);
    }

    let mut final_score = 0;

    input.read_to_string(&mut text).expect("failure reading file");
    for line in text.split("\n") {
        if !line.is_empty() {
            let mut mutual_list = Vec::new();

            let (first, second) = line.split_at(line.len()/2);
            for char in first.chars() {
                if second.contains(char.to_string().as_str()) && !(mutual_list.contains(&char)) {
                    mutual_list.push(char);
                }
            }
            for mutual in mutual_list {
                final_score += value_map.get(mutual.to_string().as_str()).unwrap();
            }
        }
    }
    println!("{}",final_score);
}

pub fn day3_part2() {
    let mut input = File::open("inputday3").expect("could not find file");
    let mut text = String::new();
    let mut value_map: HashMap<char,i32> = HashMap::with_capacity(52);
    let mut iteration = 0;
    for char in 'a'..='z' {
        iteration += 1;
        value_map.insert(char, iteration);
    }
    for char in 'A'..='Z' {
        iteration += 1;
        value_map.insert(char, iteration);
    }

    let mut final_score = 0;
    let mut first_elf = Vec::new();
    let mut second_elf = Vec::new();

    let mut iteration = 0;
    input.read_to_string(&mut text).expect("failure reading file");
    for line in text.split("\n") {
        if iteration == 0 {
            first_elf = line.chars().collect();
        } else if iteration == 1 {
            second_elf = line.chars().collect();
        } else {
            for char in line.chars() {
                if first_elf.contains(&char) && second_elf.contains(&char) {
                    final_score += value_map.get(&char).expect("issue with hashmap");
                    break
                }
            }
        }
        iteration = (iteration + 1) % 3;
    }
    println!("{}",final_score);
}