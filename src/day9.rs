use std::fs::File;
use std::io::Read;
use crate::day9::Direction::{Down, Left, Right, Up};

pub fn day9_part1() {
    let mut input = File::open("inputday9").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");
    let mut rope1 = Rope::new();
    let mut rope = ExtendedRope::new(9);

    for line in text.split("\n") {
        if !line.is_empty() {
            let (command, amount) = line.split_once(" ").unwrap();
            let mut amount = amount.parse::<i32>().unwrap();
            while amount > 0 {
                match command {
                    "D" => {
                        rope1.down();
                        rope.change(Down);
                    },
                    "U" => {
                        rope1.up();
                        rope.change(Up);
                    },
                    "L" => {
                        rope1.left();
                        rope.change(Left);
                    },
                    "R" => {
                        rope1.right();
                        rope.change(Right);
                    },
                    _ => unreachable!(),
                }
                amount -= 1;
            }
        }
    }
    for visited in &rope.visited {
        println!("({},{})", visited.0,visited.1);
    }

    println!("part2: {}, part1: {}", rope.visited.len(), rope1.visited.len());
}

struct Rope {
    head : Position,
    tail: Position,
    visited: Vec<(i32,i32)>
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Position::new(),
            tail: Position::new(),
            visited: vec![(0, 0)],
        }
    }

    fn down(&mut self) {
        self.head.y -= 1;
        if (self.head.y - self.tail.y).abs() > 1 {
            self.tail.y = self.head.y + 1;
            self.tail.x = self.head.x;
            if !self.visited.contains(&self.tail.pos()) {
                self.visited.push(self.tail.pos());
            }
        }
    }

    fn up(&mut self) {
        self.head.y += 1;
        if (self.head.y - self.tail.y).abs() > 1 {
            self.tail.y = self.head.y - 1;
            self.tail.x = self.head.x;
            if !self.visited.contains(&self.tail.pos()) {
                self.visited.push(self.tail.pos());
            }
        }
    }

    fn left(&mut self) {
        self.head.x -= 1;
        if (self.head.x - self.tail.x).abs() > 1 {
            self.tail.x = self.head.x + 1;
            self.tail.y = self.head.y;
            if !self.visited.contains(&self.tail.pos()) {
                self.visited.push(self.tail.pos());
            }
        }
    }

    fn right(&mut self) {
        self.head.x += 1;
        if (self.head.x - self.tail.x).abs() > 1 {
            self.tail.x = self.head.x - 1;
            self.tail.y = self.head.y;
            if !self.visited.contains(&self.tail.pos()) {
                self.visited.push(self.tail.pos());
            }
        }
    }
}

struct ExtendedRope {
    head : Position,
    tail: Option<Box<ExtendedRope>>,
    visited: Vec<(i32,i32)>
}

impl ExtendedRope {
    fn new(tails: u8) -> ExtendedRope {
        if tails > 0 {
            ExtendedRope {
                head: Position::new(),
                tail: Some(Box::from(ExtendedRope::new(tails - 1))),
                visited: vec![(0, 0)],
            }
        } else {
            ExtendedRope {
                head: Position::new(),
                tail: None,
                visited: vec![(0, 0)],
            }
        }
    }

    fn get_tail(&self) -> &ExtendedRope {
        match &self.tail {
            Some(x) => x,
            _ => panic!(),
        }
    }

    fn change(&mut self, direction: Direction) {
        let mut tail = self.tail.as_mut().unwrap();
        let mut visited = self.visited.as_mut();
        match direction {
            Up => {
                self.head.y += 1;
                if self.head.out_of_range(&tail.head) {
                    tail.head.x = self.head.x;
                    tail.head.y = self.head.y - 1;
                    tail.chain(visited);
                }
            }
            Down => {
                self.head.y -= 1;
                if self.head.out_of_range(&tail.head) {
                    tail.head.x = self.head.x;
                    tail.head.y = self.head.y + 1;
                    tail.chain(visited);
                }
            }
            Left => {
                self.head.x -= 1;
                if self.head.out_of_range(&tail.head) {
                    tail.head.x = self.head.x + 1;
                    tail.head.y = self.head.y;
                    tail.chain(visited);
                }
            }
            Right => {
                self.head.x += 1;
                if self.head.out_of_range(&tail.head) {
                    tail.head.x = self.head.x - 1;
                    tail.head.y = self.head.y;
                    tail.chain(visited);
                }
            }
        }
    }

    fn chain(&mut self, passed_root: &mut Vec<(i32,i32)>) {
        match &self.tail {
            None => {
                if !passed_root.contains(&self.head.pos()) {
                    passed_root.push(self.head.pos());
                }
                return;
            },
            _ => {},
        }
        let mut tail = self.tail.as_mut().unwrap();
        if self.head.out_of_range(&tail.head) {
            if self.head.x > tail.head.x + 1 && self.head.y > tail.head.y + 1 {
                tail.head.x = self.head.x - 1;
                tail.head.y = self.head.y - 1;
            } else if self.head.x < tail.head.x + 1 && self.head.y < tail.head.y + 1 {
                tail.head.x = self.head.x + 1;
                tail.head.y = self.head.y + 1;
            } else {
                if self.head.x > tail.head.x + 1 {
                    tail.head.x = self.head.x - 1;
                    tail.head.y = self.head.y;
                } else if self.head.x < tail.head.x - 1 {
                    tail.head.x = self.head.x + 1;
                    tail.head.y = self.head.y;
                }
                if self.head.y > tail.head.y + 1 {
                    tail.head.x += self.head.x;
                    tail.head.y = self.head.y - 1;
                } else if self.head.y < tail.head.y - 1 {
                    tail.head.x += self.head.x;
                    tail.head.y = self.head.y + 1;
                }
            }
        }
        tail.chain(passed_root);
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Position {
        Position {
            x: 0,
            y: 0,
        }
    }

    fn pos(&self) -> (i32,i32) {
        (self.x,self.y)
    }

    fn out_of_range(&self, head: &Position) -> bool {
        (self.x - head.x).abs() > 1 || (self.y - head.y).abs() > 1
    }
}