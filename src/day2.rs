use std::fs::File;
use std::io::Read;

enum PlayerAction {
    X,
    Y,
    Z,
    None,
}

enum OpponentAction {
    A,
    B,
    C,
    None,
}

struct RoundActions {
    player: PlayerAction,
    opponent: OpponentAction,
}

impl RoundActions {
    fn new() -> RoundActions {
        RoundActions {
            player: PlayerAction::None,
            opponent: OpponentAction::None,
        }
    }

    fn update(&mut self, action: &str) {
        match action {
            "A" => self.opponent = OpponentAction::A,
            "B" => self.opponent = OpponentAction::B,
            "C" => self.opponent = OpponentAction::C,
            "X" => self.player = PlayerAction::X,
            "Y" => self.player = PlayerAction::Y,
            "Z" => self.player = PlayerAction::Z,
            _ => ()
        }
    }

    fn calculate_score(& self) -> i32 {
        let score = match self.player {
            PlayerAction::X => 1,
            PlayerAction::Y => 2,
            PlayerAction::Z => 3,
            _ => 0,
        };
        match (&self.player,&self.opponent) {
            (PlayerAction::X,OpponentAction::C) => score + 6,
            (PlayerAction::Y,OpponentAction::A) => score + 6,
            (PlayerAction::Z,OpponentAction::B) => score + 6,
            (PlayerAction::X,OpponentAction::A) => score + 3,
            (PlayerAction::Y,OpponentAction::B) => score + 3,
            (PlayerAction::Z,OpponentAction::C) => score + 3,
            _ => score,
        }
    }

    fn calculate_score2(&self) -> i32 {
        let score = match self.player {
            PlayerAction::X => 0,
            PlayerAction::Y => 3,
            PlayerAction::Z => 6,
            _ => 0,
        };
        match (&self.player,&self.opponent) {
            (PlayerAction::X,OpponentAction::C) => score + 2,
            (PlayerAction::Y,OpponentAction::A) => score + 1,
            (PlayerAction::Z,OpponentAction::B) => score + 3,
            (PlayerAction::X,OpponentAction::A) => score + 3,
            (PlayerAction::Y,OpponentAction::B) => score + 2,
            (PlayerAction::Z,OpponentAction::C) => score + 1,
            (PlayerAction::X,OpponentAction::B) => score + 1,
            (PlayerAction::Y,OpponentAction::C) => score + 3,
            (PlayerAction::Z,OpponentAction::A) => score + 2,
            _ => 0,
        }
    }
}

pub fn day2_part1() {
    let mut input = File::open("inputday2").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");
    let mut total_score = 0;
    for line in text.split("\n") {
        let mut actions = RoundActions::new();
        if !line.is_empty() {
            for action in line.split(" ") {
                actions.update(action);
            }
        }
        total_score += actions.calculate_score();
    }
    println!("{}",total_score);
}

pub fn day2_part2() {
    let mut input = File::open("inputday2").expect("could not find file");
    let mut text = String::new();
    input.read_to_string(&mut text).expect("failure reading file");
    let mut total_score = 0;
    for line in text.split("\n") {
        let mut actions = RoundActions::new();
        if !line.is_empty() {
            for action in line.split(" ") {
                actions.update(action);
            }
        }
        total_score += actions.calculate_score2();
    }
    println!("total: {}",total_score);
}