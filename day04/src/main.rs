use std::env;
use std::fs;

fn read_board(file_name: &String) -> Vec<Vec<char>> {
    return fs::read_to_string(file_name.clone())
        .unwrap()
        .lines()
        .map(String::from)
        .map(|s| s.chars().collect())
        .collect();
}

fn check_left(board: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if j < 3 {
        return 0;
    }
    if board[i][j-1] == 'M' &&
        board[i][j-2] == 'A' &&
        board[i][j-3] == 'S' {
        return 1;
    }
    return 0;
}
fn check_down_left(board: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if (j < 3) || ((i+3) >= board.len()) {
        return 0;
    }
    if board[i+1][j-1] == 'M' &&
        board[i+2][j-2] == 'A' &&
        board[i+3][j-3] == 'S' {
        return 1;
    }
    return 0;
}
fn check_down(board: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if (i+3) >= board.len() {
        return 0;
    }
    if board[i+1][j] == 'M' &&
        board[i+2][j] == 'A' &&
        board[i+3][j] == 'S' {
        return 1;
    }
    return 0;
}
fn check_down_right(board: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if ((j+3) >= board[0].len()) || ((i+3) >= board.len()) {
        return 0;
    }
    if board[i+1][j+1] == 'M' &&
        board[i+2][j+2] == 'A' &&
        board[i+3][j+3] == 'S' {
        return 1;
    }
    return 0;
}
fn check_right(board: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if (j+3) >= board[0].len() {
        return 0;
    }
    if board[i][j+1] == 'M' &&
        board[i][j+2] == 'A' &&
        board[i][j+3] == 'S' {
        return 1;
    }
    return 0;
}
fn check_up_right(board: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if ((j+3) >= board[0].len()) || (i < 3) {
        return 0;
    }
    if board[i-1][j+1] == 'M' &&
        board[i-2][j+2] == 'A' &&
        board[i-3][j+3] == 'S' {
        return 1;
    }
    return 0;
}
fn check_up(board: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if i < 3 {
        return 0;
    }
    if board[i-1][j] == 'M' &&
        board[i-2][j] == 'A' &&
        board[i-3][j] == 'S' {
        return 1;
    }
    return 0;
}
fn check_up_left(board: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if (j < 3) || (i < 3) {
        return 0;
    }
    if board[i-1][j-1] == 'M' &&
        board[i-2][j-2] == 'A' &&
        board[i-3][j-3] == 'S' {
        return 1;
    }
    return 0;
}
fn part1(board: &Vec<Vec<char>>) {
    let mut word_count: i32 = 0;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 'X' {
                word_count += check_up(board, i, j);
                word_count += check_up_right(board, i, j);
                word_count += check_right(board, i, j);
                word_count += check_down_right(board, i, j);
                word_count += check_down(board, i, j);
                word_count += check_down_left(board, i, j);
                word_count += check_left(board, i, j);
                word_count += check_up_left(board, i, j);
            }
        }
    }
    println!("Part 1 word count: {}", word_count);
}

fn check_xmas_vert_right(board: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if ((j+2) >= board[0].len()) || ((i+2) >= board.len()) {
        return 0;
    }
    if board[i+1][j+1] == 'A' &&
        board[i][j+2] == 'S' &&
        board[i+2][j+2] == 'S' {
        return 1;
    }
    return 0;
}
fn check_xmas_vert_left(board: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if (j<2) || ((i+2) >= board.len()) {
        return 0;
    }
    if board[i+1][j-1] == 'A' &&
        board[i][j-2] == 'S' &&
        board[i+2][j-2] == 'S' {
        return 1;
    }
    return 0;
}
fn check_xmas_horiz_down(board: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if ((i+2) >= board.len()) || ((j+2) >= board[0].len()) {
        return 0;
    }
    if board[i+1][j+1] == 'A' &&
        board[i+2][j] == 'S' &&
        board[i+2][j+2] == 'S' {
        return 1;
    }
    return 0;
}
fn check_xmas_horiz_up(board: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if (i<2) || ((j+2) >= board[0].len()) {
        return 0;
    }
    if board[i-1][j+1] == 'A' &&
        board[i-2][j] == 'S' &&
        board[i-2][j+2] == 'S' {
        return 1;
    }
    return 0;
}
fn part2(board: &Vec<Vec<char>>) {
    let mut word_count: i32 = 0;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] != 'M' {
                continue;
            }
            if ((i+2) < board.len()) && (board[i+2][j] == 'M') {
                word_count += check_xmas_vert_left(board, i, j);
                word_count += check_xmas_vert_right(board, i, j);
            }
            if ((j+2) < board[i].len()) && (board[i][j+2] == 'M') {
                word_count += check_xmas_horiz_up(board, i, j);
                word_count += check_xmas_horiz_down(board, i, j);
            }
        }
    }
    println!("Part2 word count: {}", word_count);
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    
    let board: Vec<Vec<char>> = read_board(&args[1]);
    
    part1(&board);
    part2(&board);
}
