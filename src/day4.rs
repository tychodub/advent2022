use std::fs::File;
use std::io::Read;

pub fn day4_part1() {
    let mut input = File::open("inputday4").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");
    let mut count = 0;
    for line in text.split("\n") {
        if !line.is_empty() {
            let (elf1, elf2) = line.split_once(",").unwrap();
            let range1 = elf1.split_once("-").unwrap();
            let range2 = elf2.split_once("-").unwrap();
            let range1 = (range1.0.parse::<u32>().unwrap(),range1.1.parse::<u32>().unwrap());
            let range2 = (range2.0.parse::<u32>().unwrap(),range2.1.parse::<u32>().unwrap());
            if containment(range1,range2) {
                count += 1;
                //println!("{}-{},{}-{}",range1.0,range1.1,range2.0,range2.1);
            }
        }
    }
    println!("{}",count);
}

fn containment(elf1: (u32,u32), elf2: (u32,u32)) -> bool {
    if elf1.0 <= elf2.0 && elf1.1 >= elf2.1 {
        true
    } else if elf1.0 >= elf2.0 && elf1.1 <= elf2.1 {
        true
    } else {
        false
    }
}

fn overlap(elf1: (u32,u32), elf2: (u32,u32)) -> bool {
    (elf1.1 >= elf2.0 && elf1.1 <= elf2.1) || (elf1.0 <= elf2.1 && elf1.0 >= elf2.0)
    || (elf2.1 >= elf1.0 && elf2.1 <= elf1.1) || (elf2.0 <= elf1.1 && elf2.0 >= elf1.0)
}

pub fn day4_part2() {
    let mut input = File::open("inputday4").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");
    let mut count = 0;
    for line in text.split("\n") {
        if !line.is_empty() {
            let (elf1, elf2) = line.split_once(",").unwrap();
            let range1 = elf1.split_once("-").unwrap();
            let range2 = elf2.split_once("-").unwrap();
            let range1 = (range1.0.parse::<u32>().unwrap(),range1.1.parse::<u32>().unwrap());
            let range2 = (range2.0.parse::<u32>().unwrap(),range2.1.parse::<u32>().unwrap());
            if overlap(range1,range2) {
                count += 1;
                println!("{}-{},{}-{}",range1.0,range1.1,range2.0,range2.1);
            }
        }
    }
    println!("{}",count);
}