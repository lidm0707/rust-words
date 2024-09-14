use rand::Rng;
use std::io;

const SHIT: &str = "[💩]"; // ตัวแทนค่า 1
const PLACE: &str = "[  ]"; // ตัวแทนค่า 0
const TREE: &str = "[🌲]"; // ตัวแทนค่า 2
const WIN: &str = "[🧈]"; // ตัวแทนค่า 2

const MATRIX_SIZE: usize = 5;
const PLAYER_POS: (usize, usize) = (MATRIX_SIZE / 2, MATRIX_SIZE / 2);

fn gen_in_line(matrix: &Vec<Vec<u8>>) {
    for row in matrix {
        for val in row {
            match val {
                1 => print!("{}", SHIT),
                0 => print!("{}", PLACE),
                2 => print!("{}", TREE),
                3 => print!("{}", WIN),
                _ => print!("[??]"),
            }
        }
        println!();
    }
}

fn generate_new_row() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..MATRIX_SIZE)
        .map(|_| {
            if rng.gen_bool(0.01) {
                3
            } else if rng.gen_bool(0.3) {
                2
            } else {
                0
            }
        })
        .collect() // <- ไอนี้ return
}

fn move_matrix(matrix: &mut Vec<Vec<u8>>, direction: &str , state: &mut String)  {
    const MATRIX_SIZE: usize = 5; // Ensure this constant matches your matrix dimensions
    let (row, col) = PLAYER_POS;

    let should_move = match direction {
        "w" if row > 0 => matrix[row - 1][col] != 2,
        "s" if row < MATRIX_SIZE - 1 => matrix[row + 1][col] != 2,
        "a" if col > 0 => matrix[row][col - 1] != 2,
        "d" if col < MATRIX_SIZE - 1 => matrix[row][col + 1] != 2,
        _ => {
            println!("Invalid direction");
            return ;
        }

    };
    let check_win = match direction {
        "w" if row > 0 => matrix[row - 1][col] == 3,
        "s" if row < MATRIX_SIZE - 1 => matrix[row + 1][col] == 3,
        "a" if col > 0 => matrix[row][col - 1] == 3,
        "d" if col < MATRIX_SIZE - 1 => matrix[row][col + 1] == 3,
        _ => {
            println!("Invalid direction");
            return ;
        },
    };

    if check_win {
        *state = "win".to_string();
        println!("You win! Found a ' Gold' in direction {}",state);
        return ; // หยุดเมื่อชนกับ '3'
    }

    if should_move {
        // Update the matrix by shifting rows/columns
        match direction {
            "w" => {
                matrix.remove(MATRIX_SIZE - 1);
                matrix.insert(0, generate_new_row());
            }
            "s" => {
                matrix.remove(0);
                matrix.push(generate_new_row());
            }
            "a" => {
                for r in matrix.iter_mut() {
                    r.remove(MATRIX_SIZE - 1);
                    r.insert(0, if rand::random::<f32>() < 0.3 { 2 } else { 0 });
                }
            }
            "d" => {
                for r in matrix.iter_mut() {
                    r.remove(0);
                    r.push(if rand::random::<f32>() < 0.3 { 2 } else { 0 });
                }
            }
            _ => {
                println!("Invalid direction");
                return ;
            }
        }
        println!("Moved {}", direction);
    } else {
        println!("Can't move {}, there's a tree in the way!", direction);
    }

    // Ensure only one "💩" is in the center
    for row in matrix.iter_mut() {
        for cell in row.iter_mut() {
            if *cell == 1 {
                *cell = 0; // Clear any extra '1'
            }
        }
    }
    matrix[PLAYER_POS.0][PLAYER_POS.1] = 1; // Place player at new position
}

fn main() {
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; MATRIX_SIZE]; MATRIX_SIZE];
    matrix[PLAYER_POS.0][PLAYER_POS.1] = 1;

    // Initialize with some random trees
    let mut rng = rand::thread_rng();
    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            if (i, j) != PLAYER_POS && rng.gen_bool(0.3) {
                matrix[i][j] = 2;
            }
        }
    }

    loop {
        gen_in_line(&matrix);
        let mut input = String::new();
        println!("Enter direction (up, down, left, right) or 'exit' to quit:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let  direction: &str = input.trim();
        if direction == "exit" {
            break;
        }
        let mut state = "run".to_string();
         move_matrix(&mut matrix, direction,&mut state);
             // หยุดลูปเมื่อชนกับ '3'
        if state == "win" {
            println!("win and break {}" , state);
            break;
        }
    }
}
