use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn day7() {
    let mut input = File::open("inputday7").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");
    let mut root = FileStructure::new();
    for line in text.lines() {
        if line.starts_with("$") {
            //println!("cash money: {}", line);
            let (_, command) = line.split_once(" ").unwrap();
            if command.starts_with("cd") {
                let (_, input) = command.split_once(" ").unwrap();
                if input == "/" {
                    root.dir_root();
                } else if input == ".." {
                    root.dir_parent();
                } else {
                    root.dir_child(input);
                }
            }
        } else {
            //println!("less cash: {}", line);
            if line.starts_with("dir") {
                let (_, dir_name) = line.split_once(" ").unwrap();
                root.add_directory(dir_name);
            } else {
                let (size, _) = line.split_once(" ").unwrap();
                let size = size.parse::<u32>().unwrap();
                root.add_file(size);
            }
        }
    }
    let mut answer1: u32 = 0;
    let mut answer2: u32 = 70000000;
    let required = 30000000 - (70000000-root.calculate_size());
    println!("required: {}", required);
    root.part1_answer(&mut answer1);
    root.part2_answer(&mut answer2,required);
    println!("part1: {}, part2: {}", answer1, answer2);
}

struct FileStructure<'a> {
    files: Vec<u32>,
    directories: HashMap<&'a str, FileStructure<'a>>,
    current_dir: Vec<&'a str>,
}

impl<'a> FileStructure<'a> {
    fn new() -> FileStructure<'a> {
        FileStructure {
            files: Vec::new(),
            directories: HashMap::new(),
            current_dir: Vec::new(),
        }
    }

    fn dir_root(&mut self) {
        self.current_dir.clear();
    }

    fn dir_parent(&mut self) {
        self.current_dir.truncate(self.current_dir.len()-1);
    }

    fn dir_child(&mut self, dir: &'a str) {
        self.current_dir.push(dir);
    }

    fn add_directory(&mut self, new_dir: &'a str) {
        if self.current_dir.is_empty() {
            self.directories.insert(new_dir, FileStructure::new());
        } else {
            let mut current = &mut FileStructure::new();
            let mut iteration = 0;
            for dir in &self.current_dir {
                if iteration == 0 {
                    iteration += 1;
                    current = self.directories.get_mut(dir).unwrap();
                } else {
                    current = current.directories.get_mut(dir).unwrap();
                }
            }
            current.directories.insert(new_dir, FileStructure::new());
        }
    }

    fn calculate_size(&self) -> u32 {
        let mut sum = 0;
        for file in &self.files {
            sum += file;
        }
        for value in self.directories.values() {
            sum += value.calculate_size();
        }
        sum
    }

    fn add_file(&mut self, size: u32) {
        if self.current_dir.is_empty() {
            self.files.push(size);
        } else {
            let mut current = &mut FileStructure::new();
            let mut iteration = 0;
            for dir in &self.current_dir {
                if iteration == 0 {
                    iteration += 1;
                    current = self.directories.get_mut(dir).unwrap();
                } else {
                    current = current.directories.get_mut(dir).unwrap();
                }
            }
            current.files.push(size);
        }
    }

    fn part1_answer(&self, answer: &mut u32) {
        let own_size = self.calculate_size();
        if own_size <= 100000 {
            *answer += own_size;
        }
        for value in self.directories.values() {
            value.part1_answer(answer);
        }
    }

    fn part2_answer(&self, answer: &mut u32, required: u32) {
        let own_size = self.calculate_size();
        if own_size >= required {
            if *answer > own_size {
                *answer = own_size;
            }
            for value in self.directories.values() {
                value.part2_answer(answer, required);
            }
        }
    }
}