use rand::Rng;

const BOARD_DIMENSION : usize = 10;

const NUM_BIG_SHIPS : i8 = 1;
const BIG_SHIP_SIZE : i8 = 4;

const NUM_MEDIUM_SHIPS : i8 = 2;
const MEDIUM_SHIP_SIZE : i8 = 2;

const NUM_SMALL_SHIPS : i8 = 4;
const SMALL_SHIP_SIZE : i8 = 1;

enum ORIENTATION {
    Vertical,
    Horizontal
}

enum BATTLESHIP {
    Big,
    Medium,
    Small
}
impl BATTLESHIP {
    fn get_num_ships(&self) -> i8 {
        match self {
            BATTLESHIP::Big => {return NUM_BIG_SHIPS},
            BATTLESHIP::Medium => {return NUM_MEDIUM_SHIPS},
            BATTLESHIP::Small => {return NUM_SMALL_SHIPS}
        }
    }
    fn get_ship_size(&self) -> i8 {
        match self {
            BATTLESHIP::Big => {return BIG_SHIP_SIZE},
            BATTLESHIP::Medium => {return MEDIUM_SHIP_SIZE},
            BATTLESHIP::Small => {return SMALL_SHIP_SIZE}
        }
    }
}

struct Board {
    board : [[i8; BOARD_DIMENSION]; BOARD_DIMENSION],
    num_ships: i8,
    open_space_left: i8
}

fn main() {
    let mut board = init_board();
    show_board(&board);
}

fn show_board(board : &Board) {
    for row in board.board {
        println!("{:?}", row);
    }
    println!("{}", board.num_ships);
    println!("{}", board.open_space_left);
}

fn init_board() -> Board {
    let board : [[i8; BOARD_DIMENSION]; BOARD_DIMENSION] = [[0; BOARD_DIMENSION]; BOARD_DIMENSION];
    let num_ships = 0;
    let open_space_left = BOARD_DIMENSION * BOARD_DIMENSION;
    let mut board_struct = Board{ board: board, num_ships: num_ships, open_space_left: open_space_left as i8};
    initialize_battleships(&mut board_struct, BATTLESHIP::Big, 1);
    initialize_battleships(&mut board_struct, BATTLESHIP::Medium, 2);
    initialize_battleships(&mut board_struct, BATTLESHIP::Small, 4);
    return board_struct;
}



fn valid_generated_ship(board: &Board, ship_size: i8, start_index: i8, row_or_col: i8, ship_orientation: &ORIENTATION) -> bool {
    match ship_orientation {
        ORIENTATION::Vertical => {
            for i in start_index..start_index + ship_size {
                if board.board[i as usize][row_or_col as usize] == 1 {
                    return false;
                }
            }
        },
        ORIENTATION::Horizontal => {
            for i in start_index..start_index + ship_size {
                if board.board[row_or_col as usize][i as usize] == 1 {
                    return false
                }
            }
        },
    };
    return true;
}
fn initialize_battleships(board: &mut Board, ship_type: BATTLESHIP, ship_count: i8) {
    let ship_size: i8 = ship_type.get_ship_size();
    board.num_ships += ship_count;
    board.open_space_left -= ship_size * ship_count;
    if board.open_space_left < 0 {
        println!("bro");
    }
    
    for _i in 0..ship_count {
        let ship_orientation: ORIENTATION =
            if rand::thread_rng().gen_range(1..=2) == 1 {
                ORIENTATION::Vertical
            } else {
                ORIENTATION::Horizontal
            };
        
        let mut random_row_or_col: i8 = rand::thread_rng().gen_range(0..BOARD_DIMENSION as i8);
        let mut random_start_index: i8 = rand::thread_rng().gen_range(0..BOARD_DIMENSION as i8 - ship_size);

        while !valid_generated_ship(board, ship_size, random_start_index, random_row_or_col, &ship_orientation) {
            random_row_or_col = rand::thread_rng().gen_range(0..BOARD_DIMENSION as i8);
            random_start_index = rand::thread_rng().gen_range(0..BOARD_DIMENSION as i8 - ship_size);
        }
        
        for i in random_start_index..random_start_index + ship_size {
            match ship_orientation {
                ORIENTATION::Vertical => {board.board[i as usize][random_row_or_col as usize] = 1},
                ORIENTATION::Horizontal => board.board[random_row_or_col as usize][i as usize] = 1,
            };
        }
    }
}
