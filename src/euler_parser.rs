use std::fs;
use std::error::Error;


pub struct Sudokus{
    pub boards: Vec<[[u8;9]; 9]>,
}

impl Sudokus {
    fn push_sudo(&mut self, sudo: [[u8;9]; 9]) {
        self.boards.push(sudo);
    }
}

pub fn parse(name: &str) -> Result<Sudokus, Box<dyn Error>> {

    let contents = fs::read_to_string(name)?;
    let lines: Vec<&str> = contents.lines().collect();

    
    let mut sudos = Sudokus {
        boards: Vec::new(),
    };

    for chunk in lines.chunks(10) {
        let board = &chunk[1..10];
        let mut sudo: [[u8;9]; 9] = [[0; 9]; 9]; 

        for _v in board {
           for (i, row) in board.iter().enumerate() {
                for (j, ch) in row.chars().enumerate() {
                    sudo[i][j] = ch.to_digit(10).unwrap() as u8;
                }
            }
        }

        sudos.push_sudo(sudo);
    }

    Ok(sudos)
}