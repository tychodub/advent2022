use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

pub fn day5_part1() {
    let mut input = File::open("inputday5").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");

    let mut storage = Storage::new();
    let mut bridge = 0;
    for line in text.split("\n") {
        if line.contains("[") {
            let mut line_pointer = 0;
            for char in line.chars() {
                line_pointer += 1;
                if !char.is_ascii_whitespace() {
                    if line_pointer == 2 {
                        storage.insert_crate(char, 1);
                    } else if ((line_pointer + 2) % 4) == 0 {
                        storage.insert_crate(char, (line_pointer + 2) / 4);
                    }
                }
            }
        } else if bridge < 2 {
            bridge += 1;
        } else {
            if !line.is_empty() {
                let (_, rest) = line.split_once("move ").unwrap();
                let (amount, rest) = rest.split_once(" from ").unwrap();
                let (from, into) = rest.split_once(" to ").unwrap();
                storage.move_crates(amount.parse().unwrap(), from.parse().unwrap(), into.parse().unwrap());
            }
        }
    }
    storage.print_contents();
}

struct Storage {
    s1: VecDeque<char>,
    s2: VecDeque<char>,
    s3: VecDeque<char>,
    s4: VecDeque<char>,
    s5: VecDeque<char>,
    s6: VecDeque<char>,
    s7: VecDeque<char>,
    s8: VecDeque<char>,
    s9: VecDeque<char>,
}

impl Storage {
    fn new() -> Storage {
        Storage {
            s1: VecDeque::new(),
            s2: VecDeque::new(),
            s3: VecDeque::new(),
            s4: VecDeque::new(),
            s5: VecDeque::new(),
            s6: VecDeque::new(),
            s7: VecDeque::new(),
            s8: VecDeque::new(),
            s9: VecDeque::new(),
        }
    }

    fn insert_crate(&mut self, goods: char, stack: u8) {
        match stack {
            1 => self.s1.push_front(goods),
            2 => self.s2.push_front(goods),
            3 => self.s3.push_front(goods),
            4 => self.s4.push_front(goods),
            5 => self.s5.push_front(goods),
            6 => self.s6.push_front(goods),
            7 => self.s7.push_front(goods),
            8 => self.s8.push_front(goods),
            9 => self.s9.push_front(goods),
            _ => panic!(),
        }
    }

    fn move_crates(&mut self,amount: u8, from: u8, into: u8) {
        let mut iteration = amount;
        while iteration > 0 {
            let element = match from {
                1 => self.s1.pop_back().unwrap(),
                2 => self.s2.pop_back().unwrap(),
                3 => self.s3.pop_back().unwrap(),
                4 => self.s4.pop_back().unwrap(),
                5 => self.s5.pop_back().unwrap(),
                6 => self.s6.pop_back().unwrap(),
                7 => self.s7.pop_back().unwrap(),
                8 => self.s8.pop_back().unwrap(),
                9 => self.s9.pop_back().unwrap(),
                _ => panic!(),
            };
            match into {
                1 => self.s1.push_back(element),
                2 => self.s2.push_back(element),
                3 => self.s3.push_back(element),
                4 => self.s4.push_back(element),
                5 => self.s5.push_back(element),
                6 => self.s6.push_back(element),
                7 => self.s7.push_back(element),
                8 => self.s8.push_back(element),
                9 => self.s9.push_back(element),
                _ => panic!(),
            }
            iteration -= 1;
        }
    }

    fn move_crates2(&mut self,amount: u8, from: u8, into: u8) {
        let mut queue = VecDeque::new();
        let mut iteration = amount;
        while iteration > 0 {
            let element = match from {
                1 => self.s1.pop_back().unwrap(),
                2 => self.s2.pop_back().unwrap(),
                3 => self.s3.pop_back().unwrap(),
                4 => self.s4.pop_back().unwrap(),
                5 => self.s5.pop_back().unwrap(),
                6 => self.s6.pop_back().unwrap(),
                7 => self.s7.pop_back().unwrap(),
                8 => self.s8.pop_back().unwrap(),
                9 => self.s9.pop_back().unwrap(),
                _ => panic!(),
            };
            queue.push_back(element);
            iteration -= 1;
        }
        while !queue.is_empty() {
            match into {
                1 => self.s1.push_back(queue.pop_back().unwrap()),
                2 => self.s2.push_back(queue.pop_back().unwrap()),
                3 => self.s3.push_back(queue.pop_back().unwrap()),
                4 => self.s4.push_back(queue.pop_back().unwrap()),
                5 => self.s5.push_back(queue.pop_back().unwrap()),
                6 => self.s6.push_back(queue.pop_back().unwrap()),
                7 => self.s7.push_back(queue.pop_back().unwrap()),
                8 => self.s8.push_back(queue.pop_back().unwrap()),
                9 => self.s9.push_back(queue.pop_back().unwrap()),
                _ => panic!(),
            }
        }
    }

    fn print_contents(&mut self) {
        println!("s1:");
        while !self.s1.is_empty() {
            println!("[{}]",self.s1.pop_back().unwrap());
        }
        println!("s2:");
        while !self.s2.is_empty() {
            println!("[{}]",self.s2.pop_back().unwrap());
        }
        println!("s3:");
        while !self.s3.is_empty() {
            println!("[{}]",self.s3.pop_back().unwrap());
        }
        println!("s4:");
        while !self.s4.is_empty() {
            println!("[{}]",self.s4.pop_back().unwrap());
        }
        println!("s5:");
        while !self.s5.is_empty() {
            println!("[{}]",self.s5.pop_back().unwrap());
        }
        println!("s6:");
        while !self.s6.is_empty() {
            println!("[{}]",self.s6.pop_back().unwrap());
        }
        println!("s7:");
        while !self.s7.is_empty() {
            println!("[{}]",self.s7.pop_back().unwrap());
        }
        println!("s8:");
        while !self.s8.is_empty() {
            println!("[{}]",self.s8.pop_back().unwrap());
        }
        println!("s9:");
        while !self.s9.is_empty() {
            println!("[{}]",self.s9.pop_back().unwrap());
        }
    }
}

pub fn day5_part2() {
    let mut input = File::open("inputday5").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");

    let mut storage = Storage::new();
    let mut bridge = 0;
    for line in text.split("\n") {
        if line.contains("[") {
            let mut line_pointer = 0;
            for char in line.chars() {
                line_pointer += 1;
                if !char.is_ascii_whitespace() {
                    if line_pointer == 2 {
                        storage.insert_crate(char, 1);
                    } else if ((line_pointer + 2) % 4) == 0 {
                        storage.insert_crate(char, (line_pointer + 2) / 4);
                    }
                }
            }
        } else if bridge < 2 {
            bridge += 1;
        } else {
            if !line.is_empty() {
                let (_, rest) = line.split_once("move ").unwrap();
                let (amount, rest) = rest.split_once(" from ").unwrap();
                let (from, into) = rest.split_once(" to ").unwrap();
                println!("amount: {} from: {} into: {}",amount,from,into);
                storage.move_crates2(amount.parse().unwrap(), from.parse().unwrap(), into.parse().unwrap());
            }
        }
    }
    storage.print_contents();
}