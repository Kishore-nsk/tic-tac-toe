use std::io::{self, Write};

fn main() {
    println!("Welcome to Tic Tac Toe, but this time I create it myself");
    println!("Choose mode, 1 = vs CPU, 2 = vs P2");

    let mode = read_choice(&["1","2"]);
    
    let mut board = [' ';9];

    let mut current = 'X';

    loop {
        print_board(&board);
        if mode == "1" && current == 'O' {
            println!("CPU is thinking...");
            let mv = cpu_move(&board);
            board[mv] = current;
            println!("CPU chose {} ", mv + 1)
        } else {
            println!("Player {}: enter move (1-9)", current);
            let idx = loop {
                let s = read_line_trimmed();
                if let Ok(n) = s.parse::<usize>() {
                    if (1..=9).contains(&n) && board[n-1] == ' ' {
                        break n - 1;
                    }
                }
                print!("Invalid move. Enter a free cell 1-9");
                let _ = io::stdout().flush();
            };
            board[idx] = current;
        }

        if let Some(winner) = check_winner(&board) {
            print_board(&board);
            println!("Winner is {} ", if winner == 'X' {"Player X"} else {"Player O"});
            break;
        }
        if board.iter().all(|&c| c != ' ' ) {
            print_board(&board);
            println!("It is a draw!");
            break;
        }
        current = if current == 'X' {'O'} else {'X'};
    }

    println!("Game over! Thanks for playing!");
}

fn check_winner(b: &[char;9]) -> Option<char> {
    let wins = [
        (0,1,2),
        (3,4,5),
        (6,7,8),
        (0,3,6),
        (1,4,7),
        (2,5,8),
        (0,4,8),
        (8,4,0),
    ];

    for &(a,c,d) in &wins {
        if b[a] != ' ' && b[a] == b[c] && b[c] == b[d] {
            return Some(b[a]);
        } 
    }
    None
}


fn cpu_move(b: &[char; 9]) -> usize {
    if b[4] == ' ' {
        return 4
    } else {
        for i in 0..9 {
            if b[i] == ' ' {
                return i
            }
        }
    }
    0
}

fn print_board(b: &[char;9]) {
    println!();
    for r in 0..3 {
        let row: Vec<String> = (0..3).map(|c| {
            let idx = r * 3 + c;
            if b[idx] == ' ' {
                (idx + 1).to_string()
            } else {
                b[idx].to_string()
            }
        }).collect();
        println!("{} | {} | {}", row[0], row[1],row[2]);
        if r != 2 {
            println!("---+---+---");
        }
    }
    println!();
}

fn read_line_trimmed() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin read failed");
    input.trim().to_string()
}


fn read_choice(opts: &[&str]) -> String {
    loop {
        println!("> ");
        let _ = io::stdout().flush();
        let s = read_line_trimmed();
        if opts.contains(&s.as_str()) {
            return s;
        }
        println!("Please enter one of {} ", opts.join(", "));
    }
}
