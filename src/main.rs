use connect_four::{GameState, ask_for_input, GameEnd, Player, COLORS};

fn main() {
    
    let mut players = Vec::new();

    print!("Player Count: ");
    let player_count = match ask_for_input().parse::<u8>(){
            Err(e) => {println!("parsing error: {e}");return;}
            Ok(n) if n <= 8 && n >=2 => n,
            Ok(n) => {println!("player count {n} is too big or too small");return;}
    };

    for i in 1..=player_count{
        print!("Player {i}'s name : ");
        let p_name = ask_for_input();
        players.push(Player::new(p_name.to_owned(), COLORS[i as usize -1].to_string()));
    }
    
    print!("Width: ");
    let width = match ask_for_input().parse::<u8>(){
            Err(e) => {println!("parsing error: {e}");return;}
            Ok(n) if n >= 4 => n,
            Ok(n) => {println!("width {n} is too small");return;}
    };

    print!("Height: ");
    let height = match ask_for_input().parse::<u8>(){
            Err(e) => {println!("parsing error: {e}");return;}
            Ok(n) if n >= 4 => n,
            Ok(n) => {println!("height {n} is too small");return;}
    };

    let mut gm = GameState::new(width.into(), height.into());

    loop{

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); //clear screen

        gm.draw_board();
        print!("{}'s turn : ", players[gm.current_player].name);
        let column = match ask_for_input().parse::<u8>(){
            Err(e) => {println!("parsing error: {e}");continue;}
            Ok(n) => n,
        };
        match gm.put_dot_in(column.into(), &players){
            Err(e) => {println!("{e}");continue;}
            Ok(_) => (),
        }

        gm.set_winner();
        match gm.winner{
            GameEnd::Win(ref player) => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char); //clear screen
                gm.draw_board();
                println!("And the winner is {}", player.name);
                break;
            }
            GameEnd::Draw => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char); //clear screen
                gm.draw_board();
                println!("Draw");
                break;
            }
            GameEnd::None => continue,
        }

    }
}

