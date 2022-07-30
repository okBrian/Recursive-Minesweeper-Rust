use std::{io::{self, Write}, usize};
use rand::Rng;

#[derive(PartialEq)]
enum GameState
{
    Neutral,
    Quit,
    Win,
    Loss
}
#[derive(Clone)]
#[derive(Copy)]
struct Slot
{
    character: char,
    has_bomb: bool,
    has_flag: bool,
    is_revealed: bool
}

const NUM_ROWS: usize = 10;
const NUM_COLS: usize = 10;
const NUM_BOMBS: usize = 15;
const OFFSET: [(isize, isize); 8] = [(1, 0), (-1, 0), (0,1), (0,-1), (-1,-1), (1,-1), (-1, 1), (1, 1)];

fn main()
{
    let mut board = [[Slot {character: '-', has_bomb: false, has_flag: false, is_revealed: false}; NUM_COLS]; NUM_ROWS];
    let mut bomb_coords = [(0, 0); NUM_BOMBS];
    let mut i = 1;
    while i != NUM_BOMBS
    {
        let y = rand::thread_rng().gen_range(0..=9);
        let x = rand::thread_rng().gen_range(0..=9);
        
        if board[y][x].has_bomb
        {
            i-=1;
            continue;
        }
        bomb_coords[i] = (y,x);
        board[y][x].has_bomb = true;
        i+=1;
    }

    let mut game_state = GameState::Neutral;

    let mut turns = 0;
    while game_state == GameState::Neutral
    {
        turns+=1;
        print_board(&board);
        game_state = get_user_input(&mut board, turns);
        // Clears Screen
        print!("\x1B[2J");
    }

    match game_state
    {
        GameState::Win => println!("You Won in: {}!", turns),
        GameState::Loss => println!("Game Over :("),
        GameState::Quit => println!("Quitting Game...."),
        GameState::Neutral => println!("An Error has occured!")
    }
    
} // fn main

fn get_user_input(board: &mut [[Slot; NUM_COLS]; NUM_ROWS], turns: i32) -> GameState
{
    loop 
    {
        let mut guessed_coords: String = String::new();
        print!("\nEnter your command: ");
        io::stdout()
            .flush()
            .expect("Couldn't Flush the stdout stream");

        io::stdin()
            .read_line(&mut guessed_coords)
            .expect("Failed to read lines");
        
        guessed_coords.retain(|c| !c.is_whitespace());
        guessed_coords = guessed_coords.to_lowercase();

        if guessed_coords.len() == 0
        {
            println!("Invalid Command Try Again");
            continue;
        }
        let command = guessed_coords[..1].to_string();

        let mut y = 0;
        let mut x = 0;
        if command != "q"
        {
            if guessed_coords.len() != 3 
            {
                println!("Invalid Command Try Again");
                continue;
            }
            if guessed_coords[1..3].parse::<usize>().is_err()
            {
                println!("Invalid Command Try Again");
                continue;
            }
            y = guessed_coords[1..2].parse::<usize>().unwrap();
            x = guessed_coords[2..3].parse::<usize>().unwrap();
        }

        match command.as_str()
        {
            "g" => guess((y,x), board),
            "f" => return flag((y,x), board),
            "r" => return reveal((y,x), board, turns),
            "q" => return GameState::Quit,
            _ => 
            {
                println!("Invalid Command Try Again");
                continue;
            }
        }
        return GameState::Neutral;
    }
} // fn get_user_input

fn print_board(board : &[[Slot; NUM_COLS]; NUM_ROWS]) -> ()
{
    for (i, &slot_array) in board.iter().enumerate()
    {
        print!("\n{}| ", i);
        for slot in slot_array
        {
            print!("{} | ", slot.character);
        }
    }
    print!("\n   ");
    for i in 0..board[0].len()
    {
        print!("{}   ", i);
    }
    
} // fn print_board

fn guess(coord: (usize,usize), board: &mut [[Slot; NUM_COLS]; NUM_ROWS])
{
    board[coord.0][coord.1].character = 'g';
} // fn guess

fn flag(coord: (usize,usize), board: &mut [[Slot; NUM_COLS]; NUM_ROWS]) -> GameState
{
    board[coord.0][coord.1].character = 'f';
    board[coord.0][coord.1].has_flag = true;
    board[coord.0][coord.1].is_revealed = false;
    return check_win(board);
} // fn flag

fn reveal(coord: (usize,usize), board: &mut [[Slot; NUM_COLS]; NUM_ROWS], turns: i32) -> GameState
{
    board[coord.0][coord.1].is_revealed = true;
    
    if turns == 1 && board[coord.0][coord.1].has_bomb 
    {
        board[coord.0][coord.1].has_bomb = false;
        loop
        {
            let y = rand::thread_rng().gen_range(0..=9);
            let x = rand::thread_rng().gen_range(0..=9);
            if board[y][x].has_bomb || (y == coord.0 && x == coord.1) 
            {
                continue;
            }
            board[y][x].has_bomb = true;
            break;
        }

    }

    if board[coord.0][coord.1].has_bomb && board[coord.0][coord.1].is_revealed
    {
        return GameState::Loss;
    }

    recusive_reveal(coord, board);

    return check_win(board);
} // fn reveal

fn check_win(board: & [[Slot; NUM_COLS]; NUM_ROWS]) -> GameState
{
    let mut count = 0;
    for slot_array in board
    {
        for slot in slot_array
        {
            if slot.has_bomb && slot.has_flag
            {
                count+=1;
            }
        }
    }
    if count == 5
    {
        return GameState::Win;
    }
    return GameState::Neutral;
} // fn check_win

fn recusive_reveal(coord: (usize,usize), board: &mut [[Slot; NUM_COLS]; NUM_ROWS])
{
    let bomb_count = set_adjacent(coord, board);
    board[coord.0][coord.1].is_revealed = true;
    board[coord.0][coord.1].character = bomb_count.to_string().chars().nth(0).unwrap();
    if bomb_count == 0
    {
        for offset in OFFSET
        {
            let y = coord.0 as isize + offset.0;
            let x = coord.1 as isize + offset.1;
            if y.is_negative() || x.is_negative() || y > 9 || x > 9
            {
                continue;
            }
            if !board[y as usize][x as usize].is_revealed
            {  
                recusive_reveal((y as usize,x as usize), board);
            }
            print_board(board);
        }
    }
} // fn recusive_reveal

fn set_adjacent(coord: (usize,usize), board: & [[Slot; NUM_COLS]; NUM_ROWS]) -> i32
{
    let mut bomb_count: i32 = 0;

    for offset in OFFSET
    {
        let y = coord.0 as isize + offset.0;
        let x = coord.1 as isize + offset.1;
        if y.is_negative() || x.is_negative() || y > 9 || x > 9
        {
            continue;
        }
        if board[y as usize][x as usize].has_bomb
        {
            bomb_count+=1;
        }
        
    }
    bomb_count
}