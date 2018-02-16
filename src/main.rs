use std::env;
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

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

    let n_stars = contents.remove(0);
    let size = contents.len();

    let mut board = Vec::new();
    let mut star_board = Vec::new();

    let mut rng = rand::thread_rng();

    for i in 0..size {
        board.push(contents[i].chars().collect::<Vec<_>>());
        star_board.push(Vec::new());
        for _ in 0..size {
            star_board[i].push(false);
        }
    }
    
    println!("{:?}",star_count(&star_board));

    between = Range::new(0,size);
    for _ in 0..100 {
        let i = between.ind_sample(&mut rng);
        println!("{}",i);
    }

    //while star_count(&star_board) < n_stars {
    //    let i = rng.gen::<usize>
    //}
}

fn star_count(star_board : &Vec<Vec<bool>>) -> usize {
    star_board.iter().fold(0, |total, ref row| row.iter().fold(0, |tot, &cell| { if cell {total + 1} else {total}}))
}