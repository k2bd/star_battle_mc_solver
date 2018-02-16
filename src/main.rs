extern crate rand;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

const MAX_ITER: usize = 1000;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let mut filename;
    if args.len() == 1 {
        filename = "example.txt".to_string();
    } else {
        filename = args[1].clone() + ".txt";
    }

    let mut f = File::open(&filename).expect(format!("Could not open file {}",filename).as_ref());

    let mut contents_string = String::new();

    f.read_to_string(&mut contents_string).expect(format!("Couldn't read contents of file {}",filename).as_ref());

    let mut contents = contents_string.split('\n').collect::<Vec<_>>();

    let n_stars = contents.remove(0).parse::<usize>().unwrap();
    let size = contents.len();

    let total_stars = n_stars * size;

    let mut regions = Vec::new();
    let mut star_board = Vec::new();

    let mut rng = rand::thread_rng();

    for i in 0..size {
        regions.push(contents[i].chars().collect::<Vec<_>>());
        star_board.push(Vec::new());
        for _ in 0..size {
            star_board[i].push(false);
        }
    }

    let between = Range::new(0,size);
    while star_count(&star_board) < total_stars{
        let i = between.ind_sample(&mut rng);
        let j = between.ind_sample(&mut rng);

        star_board[i][j] = true;
    }

    println!("{:?}",star_board[..][0]);

    let mut i = 0;
    let mut board_score = score(&star_board,&regions,n_stars);

    let prob_range = Range::new(0.0f64, 1.0f64);

    while board_score > 0  && i < MAX_ITER {
        let new_board = mutate(&star_board);

        let new_score = score(&new_board,&regions,n_stars);
        if new_score < board_score {
            star_board = new_board;
            board_score = new_score;
        } else {
            // Accept with probability old_score / new_score;
            let target = board_score as f64 / new_score as f64;
            let u = prob_range.ind_sample(&mut rng);
            if u <= target {
                star_board = new_board;
                board_score = new_score;
            }
        }

        print_board(&star_board);
        println!("{}",score(&star_board,&regions,n_stars));
        i += 1;
    }

    print_board(&star_board);
}

fn star_count(star_board: &Vec<Vec<bool>>) -> usize {
    star_board.iter().fold(0, |total, ref row| total + row.iter().fold(0, |tot, &cell| { if cell {tot + 1} else {tot}}))
}

fn score(star_board: &Vec<Vec<bool>>, regions: &Vec<Vec<char>>, n_stars: usize) -> usize {
    let mut total = 0;

    for i in 0..star_board.len() {
        let row_total = star_board[i].iter().fold(0, |row_tot, &cell| {if cell {row_tot + 1} else {row_tot}});
        total += (row_total as isize - n_stars as isize).abs() as usize;
    }

    for i in 0..star_board.len() {
        let col_total = star_board[..][i].iter().fold(0, |col_tot, &cell| {if cell {col_tot + 1} else {col_tot}});
        total += (col_total as isize - n_stars as isize).abs() as usize;
    }

    total
}

fn mutate(star_board: &Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    let mut board = star_board.clone();

    let mut rng = rand::thread_rng();
    let between = Range::new(0,star_board.len());

    let i1 = between.ind_sample(&mut rng);
    let j1 = between.ind_sample(&mut rng);
    let i2 = between.ind_sample(&mut rng);
    let j2 = between.ind_sample(&mut rng);

    let state1 = board[i1][j1];
    let state2 = board[i2][j2];

    board[i1][j1] = state2;
    board[i2][j2] = state1;

    board
}

fn print_board(board: &Vec<Vec<bool>>) {
    let mut print_board = Vec::new();

    for row in board {
        print_board.push(String::new());
    }

    for i in 0..board.len() {
        for &cell in &board[i] {
            if cell {
                print_board[i] += "*";
            } else {
                print_board[i] += ".";
            }
        }
    }

    for line in print_board {
        println!("{}",line);
    }
}