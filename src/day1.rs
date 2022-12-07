use std::fs::File;
use std::io::Read;

pub fn day1() {
    let mut input = File::open("inputday1").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");
    let mut max_calories = 0;
    let mut current_calories = 0;
    for line in text.split("\n") {
        if line.is_empty() && current_calories > max_calories{
            max_calories = current_calories;
            current_calories = 0;
        } else if line.is_empty() {
            current_calories = 0;
        } else if !line.is_empty() {
            current_calories += line.parse::<i32>().expect("error parsing line");
        }
    }
    println!("{}", max_calories);
}

pub fn day1_part2() {
    let mut input = File::open("inputday1").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");
    let mut max_calories = 0;
    let mut second_calories = 0;
    let mut third_calories = 0;
    let mut current_calories = 0;
    for line in text.split("\n") {
        if line.is_empty() {
            if current_calories > max_calories {
                third_calories = second_calories;
                second_calories = max_calories;
                max_calories = current_calories;
            } else if current_calories > second_calories {
                third_calories = second_calories;
                second_calories = current_calories;
            } else if current_calories > third_calories {
                third_calories = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories += line.parse::<u32>().expect("error parsing line");
        }
    }
    let top_three = max_calories+second_calories+third_calories;
    println!("{},{},{}",max_calories,second_calories,third_calories);
    println!("{}", top_three);
}