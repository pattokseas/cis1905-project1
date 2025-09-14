use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameStatus {
    Win,
    Lose,
    Continue,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardError {
    InvalidCharacter(char),
    InvalidSize,
    NoMinotaur,
    NoTheseus,
    NoGoal,
    MultipleMinotaur,
    MultipleTheseus,
    MultipleGoal,
}
impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
            BoardError::InvalidSize => write!(f, "Invalid size"),
            BoardError::NoMinotaur => write!(f, "No minotaur"),
            BoardError::NoTheseus => write!(f, "No theseus"),
            BoardError::NoGoal => write!(f, "No goal"),
            BoardError::MultipleMinotaur => write!(f, "Multiple minotaur"),
            BoardError::MultipleTheseus => write!(f, "Multiple theseus"),
            BoardError::MultipleGoal => write!(f, "Multiple goal"),
        }
    }
}
impl Error for BoardError {}

#[derive(Clone)]
pub struct Grid {
    board: Vec<Vec<bool>>, // true = wall
}
impl Grid {}

#[derive(Clone)]
pub struct Game {
    grid: Grid,
    theseus: (usize, usize),
    minotaur: (usize, usize),
    goal: (usize, usize),
}

impl Game {
    pub fn from_board(board: &str) -> Result<Game, BoardError> {
        let mut board_vec: Vec<Vec<bool>> = Vec::new();
        let mut theseus: Option<(usize, usize)> = None;
        let mut minotaur: Option<(usize, usize)> = None;
        let mut goal: Option<(usize, usize)> = None;
        let mut line_number: usize = 0;
        // helper for setting a position, or erroring if it's already been set
        fn check_and_set(
            pos: &mut Option<(usize, usize)>,
            x: usize,
            y: usize,
            err: BoardError,
        ) -> Result<(), BoardError> {
            match *pos {
                None => {
                    *pos = Some((x, y));
                    Ok(())
                }
                _ => Err(err),
            }
        }
        // parse each line into a row
        for line in board.lines() {
            let mut row: Vec<bool> = Vec::new();
            let mut col: usize = 0;
            // parse each character into a column in the row
            for c in line.chars() {
                match c {
                    'X' => {
                        row.push(true);
                    }
                    'T' => {
                        check_and_set(&mut theseus, col, line_number, BoardError::MultipleTheseus)?;
                        row.push(false);
                    }
                    'M' => {
                        check_and_set(
                            &mut minotaur,
                            col,
                            line_number,
                            BoardError::MultipleMinotaur,
                        )?;
                        row.push(false);
                    }
                    'G' => {
                        check_and_set(&mut goal, col, line_number, BoardError::MultipleGoal)?;
                        row.push(false);
                    }
                    ' ' => {
                        row.push(false);
                    }
                    bad => {
                        return Err(BoardError::InvalidCharacter(bad));
                    }
                }
                col += 1;
            }
            board_vec.push(row);
            line_number += 1;
        }
        // check that all is well and return
        if theseus == None {
            return Err(BoardError::NoTheseus);
        }
        if minotaur == None {
            return Err(BoardError::NoMinotaur);
        }
        if goal == None {
            return Err(BoardError::NoGoal);
        }
        Ok(Game {
            grid: Grid { board: board_vec },
            theseus: theseus.unwrap(),
            minotaur: minotaur.unwrap(),
            goal: goal.unwrap(),
        })
    }

    pub fn show(&self) {
        // helper for choosing a wall character
        // each argument is true if there is another wall in that direction
        fn get_wall_char(left: bool, right: bool, up: bool, down: bool) -> char {
            match (left, right, up, down) {
                (false, false, false, false) => '\u{25FB}',
                (_, _, false, false) => '\u{2550}',
                (false, false, _, _) => '\u{2551}',
                (true, false, true, false) => '\u{255D}',
                (true, false, false, true) => '\u{2557}',
                (false, true, true, false) => '\u{255A}',
                (false, true, false, true) => '\u{2554}',
                (true, true, true, false) => '\u{2569}',
                (true, true, false, true) => '\u{2566}',
                (true, false, true, true) => '\u{2563}',
                (false, true, true, true) => '\u{2560}',
                (true, true, true, true) => '\u{256C}',
            }
        }
        let theseus_char = 'T';
        let minotaur_char = 'M';
        let goal_char = 'G';
        for y in 0..self.grid.board.len() {
            let mut line = String::from("");
            for x in 0..self.grid.board[y].len() {
                if (x, y) == self.minotaur {
                    line.push(minotaur_char);
                    continue;
                }
                if (x, y) == self.theseus {
                    line.push(theseus_char);
                    continue;
                }
                if (x, y) == self.goal {
                    line.push(goal_char);
                    continue;
                }
                if !self.grid.board[y][x] {
                    line.push(' ');
                    continue;
                }
                let left = x > 0 && self.grid.board[y][x - 1];
                let right = x + 1 < self.grid.board[y].len() && self.grid.board[y][x + 1];
                let up = y > 0 && x < self.grid.board[y - 1].len() && self.grid.board[y - 1][x];
                let down = y + 1 < self.grid.board.len()
                    && x < self.grid.board[y + 1].len()
                    && self.grid.board[y + 1][x];
                line.push(get_wall_char(left, right, up, down));
            }
            println!("{}", line);
        }
    }

    pub fn minotaur_move(&mut self) {
        // if the minotaur is left of theseus and it can go right, go right
        if self.minotaur.0 < self.theseus.0
            && self.minotaur.0 + 1 < self.grid.board[self.minotaur.1].len()
            && !self.is_wall(self.minotaur.1, self.minotaur.0 + 1)
        {
            self.minotaur.0 += 1;
            return;
        }
        // if the minotaur is right of theseus and it can go left, go left
        if self.minotaur.0 > self.theseus.0 && !self.is_wall(self.minotaur.1, self.minotaur.0 - 1) {
            self.minotaur.0 -= 1;
            return;
        }
        // if the minotaur is above theseus and it can go down, go down
        if self.minotaur.1 < self.theseus.1
            && self.minotaur.0 < self.grid.board[self.minotaur.1 + 1].len()
            && !self.is_wall(self.minotaur.1 + 1, self.minotaur.0)
        {
            self.minotaur.1 += 1;
            return;
        }
        // if the minotaur is below theseus and it can go up, go up
        if self.minotaur.1 > self.theseus.1
            && self.minotaur.0 < self.grid.board[self.minotaur.1 - 1].len()
            && !self.is_wall(self.minotaur.1 - 1, self.minotaur.0)
        {
            self.minotaur.1 -= 1;
            return;
        }
        // if no moves which put it closer are possible, it does nothing
    }

    pub fn theseus_move(&mut self, command: Command) {
        if command == Command::Left
            && self.theseus.0 > 0
            && !self.is_wall(self.theseus.1, self.theseus.0 - 1)
        {
            self.theseus.0 -= 1;
        }
        if command == Command::Right
            && self.theseus.0 < self.grid.board[self.theseus.1].len()
            && !self.is_wall(self.theseus.1, self.theseus.0 + 1)
        {
            self.theseus.0 += 1;
        }
        if command == Command::Up
            && self.theseus.1 > 0
            && self.theseus.0 < self.grid.board[self.theseus.1 - 1].len()
            && !self.is_wall(self.theseus.1 - 1, self.theseus.0)
        {
            self.theseus.1 -= 1;
        }
        if command == Command::Down
            && self.theseus.1 < self.grid.board.len()
            && self.theseus.0 < self.grid.board[self.theseus.1 + 1].len()
            && !self.is_wall(self.theseus.1 + 1, self.theseus.0)
        {
            self.theseus.1 += 1;
        }
    }

    pub fn status(&self) -> GameStatus {
        if self.theseus == self.goal {
            GameStatus::Win
        } else if self.theseus == self.minotaur {
            GameStatus::Lose
        } else {
            GameStatus::Continue
        }
    }
}

impl Game {
    /// Returns true if the given position is Theseus
    pub fn is_theseus(&self, row: usize, col: usize) -> bool {
        (col, row) == self.theseus
    }
    /// Returns true if the given position is Minotaur
    pub fn is_minotaur(&self, row: usize, col: usize) -> bool {
        (col, row) == self.minotaur
    }
    /// Returns true if the given position is a wall
    pub fn is_wall(&self, row: usize, col: usize) -> bool {
        self.grid.board[row][col]
    }
    /// Returns true if the given position is the goal
    pub fn is_goal(&self, row: usize, col: usize) -> bool {
        (col, row) == self.goal
    }
    /// Returns true if the given position is empty
    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        !self.is_wall(row, col)
            && (col, row) != self.theseus
            && (col, row) != self.minotaur
            && (col, row) != self.goal
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    /// Move one tile up
    Up,
    /// Move one tile down
    Down,
    /// Move one tile left
    Left,
    /// Move one tile right
    Right,
    /// Don't move at all
    Skip,
}

//  To get a command from the user, you can use the following code:
//  ```
//  let line = stdin.lines().next().unwrap().unwrap();
//  ```
//  This will read a line from the user and store it in the `buffer` string.
//
//  Unfortunately, since stdin is line-buffered, everytime you enter a command while playing the
//  game you will have to press "enter" afterwards to send a new line.
//
//  While using the arrow keys to take inputs would be natural, it can be difficult to handle arrow
//  keys in a way that works on all devices. Therefore, it's recommended that you either use "w",
//  "a", "s", and "d" to take input, or else the words "up", "down", "left", "right". You can take
//  input however you like, so long as you document it here in a comment and it is reasonable to
//  use as a player.

// Either "WASD" or literal words are valid
pub fn input(stdin: impl io::Read + io::BufRead) -> Option<Command> {
    let line = stdin.lines().next().unwrap().unwrap();
    match line.to_lowercase().as_str() {
        "w" | "up" => Some(Command::Up),
        "a" | "left" => Some(Command::Left),
        "s" | "down" => Some(Command::Down),
        "d" | "right" => Some(Command::Right),
        "skip" => Some(Command::Skip),
        _ => None,
    }
}
