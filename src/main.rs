use std;
use std::ops;

const R: usize = 4;
const C: usize = 4;

fn print_grid(gr: &[[i32 ; C] ; R]){
    for i in 0..R {
        println!("{:?}", gr[i]);
    }
}

fn search(i:usize, j:usize, gr: &mut [[i32 ; C] ; R]){
    if gr[i][j] % 2 == 0 {gr[i][j] += (i + j) as i32;}
    else{return}

    if j > 0{search(i, j - 1, gr);}
    if j < C - 1{search(i, j + 1, gr);}
    if i > 0 {search(i - 1, j, gr);}
    if i < R - 1 {search(i + 1, j, gr);}
}

fn clamp<T>(value: &mut T, low: T, high: T) {
    if *value < low {*value = low;}
    else if *value > high {*value = high;}
}
struct MainState{
    p1: (i32, i32, i32, char),
    p2: (i32, i32, i32, char),
}

impl MainState {
    pub fn new(s1: char, s2: char) -> Self {
        MainState 
        { 
            p1: (0,0,0, s1), p2: ((R - 1) as i32, (C - 1) as i32, 0, s2) 
        }
    }

    pub fn random_walk() {
        p1.1 += 
        p2.2 +=
    }
}
fn main() {
    let mut grid:[[i32 ; C] ; R] = [[0i32; C] ; R];
    print_grid(&grid);
        //print!("{}[2J", 27 as char);
    print!("{}[2J", 27 as char);
    search(0 ,0, &mut grid);
    print_grid(&grid);
    std::thread::sleep_ms(400);
}