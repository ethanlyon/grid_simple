use std;
use rand::{thread_rng, Rng};
use noise::{NoiseFn, Perlin};

const R: usize = 4;
const C: usize = 4;
const N_PLAYERS: usize = 3;
const DIRS: [[i32 ; 2] ; 5] = [[0,1], [1,0], [0, -1], [-1, 0], [0, 0]];

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

fn clamp(value: &mut i32, low: i32, high: i32) {
    if *value < low {*value = low;}
    else if *value > high {*value = high;}
}

struct MainState {
    players : [[i32; 3]; N_PLAYERS],
}

impl MainState {
    pub fn new() -> Self {
        MainState { 
            players: [[0; 3] ; N_PLAYERS],
        }
    }

    pub fn random_walk(&mut self, grid: &[[i32 ; C] ; R] ){
        let create_random_dir = |num: i32| {let rng:i32 = thread_rng().gen_range(-1..2); num + rng};
        for i in 0..N_PLAYERS {
            let val = self.players[i];
            let rand_dir = DIRS[thread_rng().gen_range(0..DIRS.len())];
            //let mut x1 = create_random_dir(val[0]);
            let mut x1 = val[0] + rand_dir[0];
            //let mut y1 = create_random_dir(val[1]);
            let mut y1 = val[1] + rand_dir[1];
            clamp(&mut x1, 0, (R - 1) as i32);
            clamp(&mut y1, 0, (C - 1) as i32);
            if grid[x1 as usize][y1 as usize] == 0 {
                self.players[i] = [x1, y1, self.players[i][2]];
            }
            //self.players[i] = [x1, y1, self.players[i][2]];
        }
    }

    pub fn update_grid(&self, grd: &mut [[i32 ; C] ; R]){
        *grd = [[0i32 ; C] ; R];
        for i in 0..N_PLAYERS {
            let x = self.players[i][0] as usize;
            let y = self.players[i][1] as usize;
            grd[x][y] = (i + 1) as i32;
            
        }
    }
}
fn main() {
    let mut grid:[[i32 ; C] ; R] = [[0i32; C] ; R];
    let mut state = MainState::new();
    let mut cnt: u8 = 0;
    loop{
        print!("{}[2J", 27 as char);
        println!("{:?} : {:?}", cnt, cnt as char);
        print_grid(&grid);
        print!("{}[2J", 27 as char);
        state.random_walk(&grid);
        state.update_grid(&mut grid);
        std::thread::sleep_ms(500);
        cnt += 1;
    }
}