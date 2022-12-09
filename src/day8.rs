use std::fs::File;
use std::io::Read;

pub fn day8_part1() {
    let mut input = File::open("inputday8").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");
    let mut matrix = Trix::new();
    for line in text.split("\n") {
        if !line.is_empty() {
            matrix.add_row();
            for char in line.chars() {
                matrix.append_at_latest(char.to_digit(10).unwrap() as u8);
            }
        }
    }

    matrix.answer_part1();
    matrix.answer_part2();
}

struct Trix {
    matrix: Vec<Vec<u8>>,
}

impl Trix {
    fn new() -> Trix {
        Trix {
            matrix: Vec::new()
        }
    }

    fn add_row(&mut self) {
        self.matrix.push(Vec::new());
    }

    fn append_at_latest(&mut self, tree: u8) {
        let latest = self.matrix.len() - 1;
        self.matrix.get_mut(latest).unwrap().push(tree);
    }

    fn get(&self ,row: usize, column: usize) -> u8 {
        self.matrix.get(row).unwrap().get(column).unwrap().clone()
    }

    fn is_visible(&self, row: usize, column: usize) -> bool {
        let height = self.get(row, column);
        let mut row_iter = 0;
        let mut column_iter = 0;
        let mut truth_tuple = (true,true,true,true);
        while row_iter < row {
            if self.get(row_iter, column) >= height {
                truth_tuple.0 = false;
            }
            row_iter += 1;
        }

        row_iter = self.matrix.len()-1;
        while row_iter > row {
            if self.get(row_iter, column) >= height {
                truth_tuple.1 = false;
            }
            row_iter -= 1;
        }

        while column_iter < column {
            if self.get(row, column_iter) >= height {
                truth_tuple.2 = false;
            }
            column_iter += 1;
        }

        column_iter = self.matrix.get(0).unwrap().len()-1;
        while column_iter > column {
            if self.get(row, column_iter) >= height {
                truth_tuple.3 = false;
            }
            column_iter -= 1;
        }

        truth_tuple.0 || truth_tuple.1 || truth_tuple.2 || truth_tuple.3
    }

    fn answer_part1(&self) {
        let mut answer = 0;
        let mut row = 0;
        let mut column;
        while row < self.matrix.len() {
            column = 0;
            while column < self.matrix.get(0).unwrap().len() {
                if self.is_visible(row,column) {
                    answer += 1;
                }
                column += 1;
            }
            row += 1;
        }
        println!("{}", answer);
    }

    fn view(&self, row: usize, column: usize) -> usize {
        let height = self.get(row, column);
        let mut row_iter = row;
        let mut column_iter = column;

        let mut top = 1;
        while row_iter < self.matrix.len() - 1 {
            if self.get(row_iter+1, column) >= height {
                top += 1;
                break;
            }
            row_iter += 1;
            top += 1;
        }
        top -= 1;

        row_iter = row;
        let mut bottom = 1;
        while row_iter > 0 {
            if self.get(row_iter-1, column) >= height {
                bottom += 1;
                break;
            }
            row_iter -= 1;
            bottom += 1;
        }
        bottom -= 1;

        let mut left = 1;
        while column_iter > 0 {
            if self.get(row, column_iter-1) >= height {
                left += 1;
                break;
            }
            column_iter -= 1;
            left += 1;
        }
        left -= 1;

        column_iter = column;
        let mut right = 1;
        while column_iter < self.matrix.get(0).unwrap().len() - 1 {
            if self.get(row, column_iter+1) >= height {
                if row == 55 && column == 29 {
                    println!("row: {}, column: {}", row, column_iter);
                }
                right += 1;
                break;
            }
            column_iter += 1;
            right += 1;
        }
        right -= 1;

        if row == 55 && column == 29 {
            println!("bottom: {}, top: {}, left: {}, right: {}", top, bottom, left, right);
        }
        top * bottom * left * right
    }

    fn answer_part2(&self) {
        let mut answer = 0;
        let mut row = 0;
        let mut column;
        while row < self.matrix.len() {
            column = 0;
            while column < self.matrix.get(0).unwrap().len() {
                if self.view(row,column) > answer {
                    answer = self.view(row,column);
                }
                column += 1;
            }
            row += 1;
        }
        println!("{}", answer);
    }
}