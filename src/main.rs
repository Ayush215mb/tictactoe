use std::io;

const PLAYER_X:char='X';
const PLAYER_O:char='O';

const BOARD_SIZE:usize=3;

//2D array- 3X3
type Board= [[char; BOARD_SIZE]; BOARD_SIZE];

fn initialize_board()-> Board{
    return [[' ';BOARD_SIZE];BOARD_SIZE];
}

fn print_Boards(board: &Board){
    for row in board{
        for cell in row{
            println!("{}",cell);
        }
        println!();
    }
}

fn get_player_move(current_player: char, board: &Board)->(usize,usize){
    println!("Player {} move you chance (row, col) ",current_player);
    let input= String::new();

    io::stdin().read_line(&mut input);// 0 1

    println!("input {}",input);

    let coordinates: Vec<usize>= input.split_whitespace().flat_map(str::parse::usize).collect()

    
}


fn play_game(){
    let mut board= initialize_board();
    let mut current_player= PLAYER_X;

    loop{
        println!("Current Board: ");
        print_Boards(&board);

        let(row,col)= get_player_move(current_player, &board);


        board[row][col] = current_player;

        if current_player== PLAYER_X{
        current_player= PLAYER_O;
    }else{
        current_player= PLAYER_X;
    }

    }
    
}
fn main() {
    println!("Welcome to the Tic Tac Toe");

    play_game();
    
}
