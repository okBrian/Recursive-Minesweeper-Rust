pub mod minesweeper_game
{
    use std::io::{self, Write};
    use rand::Rng;


    const NUM_ROWS: usize = 10;
    const NUM_COLS: usize = 10;
    const NUM_BOMBS: usize = 15;
    const OFFSET: [(isize, isize); 8] = [(1, 0), (-1, 0), (0,1), (0,-1), (-1,-1), (1,-1), (-1, 1), (1, 1)];

    pub struct Game 
    {
        board: [[Slot; NUM_COLS]; NUM_ROWS],
        bomb_coords: [(i32,i32); NUM_BOMBS],
        game_state: GameState,
        turns: i32,
    }

    impl Game
    {
        pub fn new() -> Self
        {
            Game
            {
                board: [[Slot {character: '-', has_bomb: false, has_flag: false, is_revealed: false}; NUM_COLS]; NUM_ROWS],
                bomb_coords: [(0, 0); NUM_BOMBS],
                game_state: GameState::Neutral,
                turns: 0,
            }
        }

        pub fn game_loop(&mut self)
        {
            self.make_bombs();
            while self.game_state == GameState::Neutral
            {
                self.turns+=1;
                self.print_board();
                self.game_state = self.get_user_input();
                // Clears Screen
                print!("\x1B[2J");
            }
        
            match self.game_state
            {
                GameState::Win => println!("You Won in: {}!", self.turns),
                GameState::Loss => println!("Game Over :("),
                GameState::Quit => println!("Quitting Game...."),
                GameState::Neutral => println!("Error: GameState On Neutral")
            }
        }

        fn make_bombs(&mut self)
        {
            let mut i = 1;
            while i != NUM_BOMBS
            {
                let y = rand::thread_rng().gen_range(0..=9);
                let x = rand::thread_rng().gen_range(0..=9);
                
                if self.board[y][x].has_bomb
                {
                    i-=1;
                    continue;
                }
                self.bomb_coords[i] = (y as i32,x as i32);
                self.board[y][x].has_bomb = true;
                i+=1;
            }
        }

        fn get_user_input(&mut self) -> GameState
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
                
                if guessed_coords.len() == 0
                {
                    println!("Invalid Command Try Again");
                    continue;
                }

                guessed_coords.retain(|c| !c.is_whitespace());
                guessed_coords = guessed_coords.to_lowercase();
        
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
                    "g" => self.guess((y,x)),
                    "f" => return self.flag((y,x)),
                    "r" => return self.reveal((y,x)),
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
        
        fn print_board(&self) -> ()
        {
            for (i, &slot_array) in self.board.iter().enumerate()
            {
                print!("\n{}| ", i);
                for slot in slot_array
                {
                    print!("{} | ", slot.character);
                }
            }
            print!("\n   ");
            for i in 0..self.board[0].len()
            {
                print!("{}   ", i);
            }
            
        } // fn print_board
        
        fn guess(&mut self, coord: (usize,usize))
        {
            self.board[coord.0][coord.1].character = 'g';
        } // fn guess
        
        fn flag(&mut self, coord: (usize,usize)) -> GameState
        {
            self.board[coord.0][coord.1].character = 'f';
            self.board[coord.0][coord.1].has_flag = true;
            self.board[coord.0][coord.1].is_revealed = false;
            return self.check_win();
        } // fn flag
        
        fn reveal(&mut self, coord: (usize,usize)) -> GameState
        {
            self.board[coord.0][coord.1].is_revealed = true;
            
            if self.turns == 1 && self.board[coord.0][coord.1].has_bomb 
            {
                self.board[coord.0][coord.1].has_bomb = false;
                loop
                {
                    let y = rand::thread_rng().gen_range(0..=9);
                    let x = rand::thread_rng().gen_range(0..=9);
                    if self.board[y][x].has_bomb || (y == coord.0 && x == coord.1) 
                    {
                        continue;
                    }
                    self.board[y][x].has_bomb = true;
                    break;
                }
        
            }
        
            if self.board[coord.0][coord.1].has_bomb && self.board[coord.0][coord.1].is_revealed
            {
                return GameState::Loss;
            }
        
            self.recusive_reveal(coord);
        
            return self.check_win();
        } // fn reveal
        
        fn check_win(&self) -> GameState
        {
            let mut count = 0;
            for (y,x) in self.bomb_coords 
            {
                if self.board[y as usize][x as usize].has_flag
                {
                    count+=1;
                }
            }
            if count == NUM_BOMBS
            {
                return GameState::Win;
            }
            GameState::Neutral
        } // fn check_win 
        
        fn recusive_reveal(&mut self, coord: (usize,usize)) 
        {
            let bomb_count = self.set_adjacent(coord);
            self.board[coord.0][coord.1].is_revealed = true;
            self.board[coord.0][coord.1].character = bomb_count.to_string().chars().nth(0).unwrap();
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
                    if !self.board[y as usize][x as usize].is_revealed
                    {  
                        self.recusive_reveal((y as usize,x as usize));
                    }
                    self.print_board();
                }
            }
        } // fn recusive_reveal
        
        fn set_adjacent(&self, coord: (usize,usize)) -> i32
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
                if self.board[y as usize][x as usize].has_bomb
                {
                    bomb_count+=1;
                }
                
            }
            return bomb_count;
        } // fn set_adjacent
    }

    #[derive(PartialEq)]
    enum GameState
    {
        Neutral,
        Quit,
        Win,
        Loss
    }
    #[derive(Copy,Clone)]
    struct Slot
    {
        character: char,
        has_bomb: bool,
        has_flag: bool,
        is_revealed: bool
    }
}