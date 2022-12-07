use std::fs::File;
use std::io::Read;

pub fn day6_part1() {
    let mut input = File::open("inputday6").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");

    let mut count = 0;
    let mut next_slot = 0;
    let mut buffer: [char; 4] = [' ',' ',' ',' '];
    let (input, _) = text.split_once("\n").unwrap();
    for char in input.chars() {
        count += 1;
        buffer[next_slot] = char;
        next_slot = (next_slot+1) % 4;
        if check_start(&buffer) {
            println!("{}", count);
            break;
        }
    }
}

fn check_start(buffer: & [char; 4]) -> bool {
    let mut checklist = [' ',' ',' ',' '];
    let mut next_slot = 0;
    for character in buffer {
        if checklist.contains(&character) {
            return false;
        } else {
            checklist[next_slot] = *character;
            next_slot += 1
        }
    }
    true
}

pub fn day6_part2() {
    let mut input = File::open("inputday6").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");

    let mut count = 0;
    let mut next_slot = 0;
    let mut buffer: [char; 14] = [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '];
    let (input, _) = text.split_once("\n").unwrap();
    for char in input.chars() {
        count += 1;
        buffer[next_slot] = char;
        next_slot = (next_slot+1) % 14;
        if check_start2(&buffer) {
            println!("{}", count);
            break;
        }
    }
}

fn check_start2(buffer: & [char; 14]) -> bool {
    let mut checklist = [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' '];
    let mut next_slot = 0;
    for character in buffer {
        if checklist.contains(&character) {
            return false;
        } else {
            checklist[next_slot] = *character;
            next_slot += 1
        }
    }
    true
}