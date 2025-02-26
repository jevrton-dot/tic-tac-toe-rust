use std::io::{self, Write};

fn print_chart(chart: [[i32; 3]; 3]) {
    println!("   0   1   2");
    println!("  ╔═══╤═══╤═══╗");
    for i in 0..3 {
        print!("{} ║", i);
        for j in 0..3 {
            if chart[i][j] == 0 {
                if j == 2 {
                    print!("   ║");
                    continue;
                }
                print!("   │");
            } else if chart[i][j] == 1 {
                if j == 2 {
                    print!(" X ║");
                    continue;
                }
                print!(" X │");
            } else {
                if j == 2 {
                    print!(" O ║");
                    continue;
                }
                print!(" O │");
            }
        }
        println!();
        if i < 2 {
            println!("  ╟───┼───┼───╢");
        }
    }
    println!("  ╚═══╧═══╧═══╝");
}

fn select_cell(chart: &mut [[i32; 3]; 3], player: i32) {
    let mut row = String::new();
    let mut col = String::new();

    println!("Player {} turn:", player);

    loop {
        print!("select row (0, 1 or 2): ");
        io::stdout().flush().unwrap();
        row.clear();
        std::io::stdin().read_line(&mut row).expect("read error");
        match row.trim().parse::<i32>() {
            Ok(r) if r >= 0 && r <= 2 => {
                row = r.to_string();
                break;
            }
            _ => {
                println!("Invalid row, try again");
            }
        }
    }

    loop {
        print!("select col (0, 1 or 2): ");
        io::stdout().flush().unwrap();
        col.clear();
        std::io::stdin().read_line(&mut col).expect("read error");
        match col.trim().parse::<i32>() {
            Ok(c) if c >= 0 && c <= 2 => {
                col = c.to_string();
                break;
            }
            _ => {
                println!("Invalid col, try again");
            }
        }
    }

    let row: i32 = row.trim().parse().expect("parse error");
    let col: i32 = col.trim().parse().expect("parse error");

    if chart[row as usize][col as usize] != 0 {
        println!("cell already taken, retry");
        select_cell(chart, player);
        return;
    }
    change_cell_value(chart, row, col, player);
}

fn change_cell_value(chart: &mut [[i32; 3]; 3], row: i32, col: i32, player: i32) {
    chart[row as usize][col as usize] = player;
}

fn change_player(player: &mut i32) {
    *player = if *player == 1 { 2 } else { 1 };
}

fn check_victory(chart: &[[i32; 3]; 3], finish: &mut bool) {
    for row in 0..3 {
        if chart[row][0] == chart[row][1] && chart[row][1] == chart[row][2] && chart[row][0] != 0 {
            *finish = true;
            return;
        }
    }
    for col in 0..3 {
        if chart[0][col] == chart[1][col] && chart[1][col] == chart[2][col] && chart[0][col] != 0 {
            *finish = true;
            return;
        }
    }
    if chart[0][0] == chart[1][1] && chart[1][1] == chart[2][2] && chart[0][0] != 0 {
        *finish = true;
        return;
    }
    if chart[0][2] == chart[1][1] && chart[1][1] == chart[2][0] && chart[0][2] != 0 {
        *finish = true;
        return;
    }
}

fn main() {
    let fir_line: [i32; 3] = [0, 0, 0];
    let sec_line: [i32; 3] = [0, 0, 0];
    let thi_line: [i32; 3] = [0, 0, 0];
    let mut chart: [[i32; 3]; 3] = [fir_line, sec_line, thi_line];

    let mut finish: bool = false;
    let mut turn: i32 = 1;
    let mut player: i32 = 1;

    while !finish {
        print_chart(chart);

        select_cell(&mut chart, player);

        change_player(&mut player);

        check_victory(&chart, &mut finish);
        if finish {
            break;
        }

        turn = turn + 1;
    }

    print_chart(chart);

    if turn % 2 == 0 {
        println!("Player 2 won!");
    } else {
        println!("Player 1 won!");
    }
}