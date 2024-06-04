use std::io::{stdout, stdin, Write};

pub const COLORS :  [&str;8] = [
"\u{001b}[31m",
"\u{001b}[34m",
"\u{001b}[32m",
"\u{001b}[33m",
"\u{001b}[35m",
"\u{001b}[36m",
"\u{001b}[37m",
"\u{001b}[30m",
];

#[derive(PartialEq)]
pub struct Player{
    pub name: String,
    color: String,
}

impl Player{
    pub fn new(n : String, c : String) -> Player{
        Player{
            name: n,
            color: (c.to_string() + "O\u{001b}[0m"),
        }
    }

    pub fn empty() -> Player{
        Player{
            name: "{empty}".to_string(),
            color: "\u{001b}[31mO\u{001b}[0m".to_string(),
        }
    }
}

pub enum GameEnd<'a>{
    Draw,
    None,
    Win(&'a Player),
}

pub struct GameState<'a>{
    pub winner: GameEnd<'a>,
    pub current_player: usize,
    move_count: u32,
    board_width: usize,
    board_height: usize,
    board: Vec<Vec<Option<&'a Player>>>,
}

impl<'a> GameState<'a>{
    
    pub fn new(w:usize,h:usize) -> GameState<'a>{
        GameState{
            move_count: 0,
            winner : GameEnd::None,
            current_player: 0,
            board_width: w,
            board_height: h,
            board: vec![vec![None;h];w],
        }
    }

    pub fn draw_board(&self){

        //O(nm) is treated the same as O(2nm) that's why i separeted it from the loop below
        print!(" ");
        for c in 1..=self.board_width {
            print!(" {}. ", c);
        }
        print!("\n");

        for r in 0..self.board_height{
            let mut row = "|".to_string();
            let mut divider = "+".to_string();
            for c in 0..self.board_width{
                divider = divider + "---+";
                row = row + &format!(" {} ", match self.board[c][r]{
                    None => " ",
                    Some(ref p) => &p.color,
                })[..];

                if c == self.board_width-1{
                    row = row + "|\n";   
                    divider = divider + "\n";
                }else{
                    row = row + "|"
                }
            }
            print!("{divider}");
            print!("{row}");
            if r == self.board_height-1{
                print!("{divider}");
            }
        }
    }

    pub fn put_dot_in(&mut self, c: usize, players : &'a Vec<Player>) -> Result<(), &'static str>{
        let c = c - 1;
        if c >= self.board_width { //does not require lower bound check because of usize type
            return Err("Column index out of bounds");
        }
        if self.board[c][0] != None {
            return Err("Column is full");
        }
        for r in 1..self.board_height{
            if self.board[c][r] != None {
                self.board[c][r-1] = Some(&players[self.current_player]);

                if self.current_player+1 >= players.len(){
                    self.current_player = 0;
                }else{
                    self.current_player +=1;
                }

                self.move_count += 1;
                break;
            }
            if r == self.board_height-1{
                self.board[c][r] = Some(&players[self.current_player]);
                
                if self.current_player+1 >= players.len(){
                    self.current_player = 0;
                }else{
                    self.current_player +=1;
                }

                self.move_count += 1;
                break;
            }
        }
        Ok(())
    }

    pub fn set_winner(&mut self){
        if self.move_count as usize >= self.board_height * self.board_width {
            self.winner = GameEnd::Draw;
        }

        for c in 0..self.board_width{
            for r in 0..self.board_height-3{
                if self.board[c][r] != None && self.board[c][r] == self.board[c][r+1] &&
                    self.board[c][r] == self.board[c][r+2] &&
                    self.board[c][r] == self.board[c][r+3] {
                    self.winner = GameEnd::Win(self.board[c][r].unwrap());
                } 
            }
        }

        for r in 0..self.board_height{
            for c in 0..self.board_width-3{
                if self.board[c][r] != None && self.board[c][r] == self.board[c+1][r] &&
                    self.board[c][r] == self.board[c+2][r] &&
                    self.board[c][r] == self.board[c+3][r] {
                    self.winner = GameEnd::Win(self.board[c][r].unwrap());

                } 
            }
        }

        //----------- diagonals -----------------------
        for row in 0..self.board_height-3{
            let mut c = 0;
            for r in row..self.board_height-3{
                //self.visualize_left(c, r);
                if self.board[c][r] != None &&
                    self.board[c][r] == self.board[c+1][r+1] &&
                    self.board[c][r] == self.board[c+2][r+2] &&
                    self.board[c][r] == self.board[c+3][r+3] {
                    self.winner = GameEnd::Win(self.board[c][r].unwrap());
                }
                c+=1;
                if c+3 >= self.board_width{
                    break;
                }
            }
        }

        for column in 1..self.board_width-3{
            let mut r = 0;
            for c in column..self.board_width-3{
                //self.visualize_left(c,r);
                if self.board[c][r] != None &&
                    self.board[c][r] == self.board[c+1][r+1] &&
                    self.board[c][r] == self.board[c+2][r+2] &&
                    self.board[c][r] == self.board[c+3][r+3] {
                    self.winner = GameEnd::Win(self.board[c][r].unwrap());
                }
                r+=1;
            }
        }

        for row in 0..self.board_height-3{
            let mut c = self.board_width-1;
            for r in row..self.board_height-3{
                //self.visualize_right(c,r);
                if self.board[c][r] != None &&
                    self.board[c][r] == self.board[c-1][r+1] &&
                    self.board[c][r] == self.board[c-2][r+2] &&
                    self.board[c][r] == self.board[c-3][r+3] {
                    self.winner = GameEnd::Win(self.board[c][r].unwrap());
                }
                c-=1;
                if c as i8 -3 <= 0{
                    break;
                }
            }
        }

        for column in (3..self.board_width-1).rev(){
            let mut r = 0;
            for c in (3..=column).rev(){
                //self.visualize_right(c,r);
                if self.board[c][r] != None &&
                    self.board[c][r] == self.board[c-1][r+1] &&
                    self.board[c][r] == self.board[c-2][r+2] &&
                    self.board[c][r] == self.board[c-3][r+3] {
                    self.winner = GameEnd::Win(self.board[c][r].unwrap());
                }
                r+=1;
            }
        }
    }

    //------------- helper functions -----------------

    fn  visualize_left(&self, c: usize, r: usize){
        let mut vis = GameState::new(self.board_width, self.board_height);
        let empty = Player::empty();
        vis.board[c][r] = Some(&empty);
        vis.board[c+1][r+1] = Some(&empty);
        vis.board[c+2][r+2] = Some(&empty);
        vis.board[c+3][r+3] = Some(&empty);
        vis.draw_board();
    }
    fn  visualize_right(&self, c: usize, r: usize){
        let mut vis = GameState::new(self.board_width, self.board_height);
        let empty = Player::empty();
        vis.board[c][r] = Some(&empty);
        vis.board[c-1][r+1] = Some(&empty);
        vis.board[c-2][r+2] = Some(&empty);
        vis.board[c-3][r+3] = Some(&empty);
        vis.draw_board();
    }
}


pub fn ask_for_input() -> String{
    let mut s = String::new();
    stdout().flush().expect("flush failed!");
    stdin().read_line(&mut s).expect("did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}
