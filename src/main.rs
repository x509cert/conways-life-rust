use minifb::{Key, Window, WindowOptions};
use std::mem;

const WIDTH: usize = 1280;
const HEIGHT: usize = 960;
const ALIVE_START: u32 = 0x777777FF; //0xFFFFFFFF;
const ALIVE_GENERATION_INC: u32 = 0x03010300;
const DEAD: u32 = 0x00000000;

fn populate_game(game_buff: &mut [u32]) {
    for _ in 0..(WIDTH * HEIGHT) / 5 { 
        let x = fastrand::usize(0..WIDTH);
        let y = fastrand::usize(0..HEIGHT);
        game_buff[y * WIDTH + x] = ALIVE_START;
    }
}

fn next_generation(game_buff: &[u32], swap_buff: &mut [u32]) {
    for x in 0..WIDTH-1 {
        for y in 0..HEIGHT-1 {
            swap_buff[y * WIDTH + x] = is_alive(game_buff, x, y);
        } 
    }
}

#[inline(always)]
fn is_alive(buff: &[u32], x: usize, y: usize) -> u32  {
    let mut neighbor: u32 = 0;

    if x > 0 && buff[y * WIDTH + x-1] > 0 { neighbor += 1};        // left
    if x < WIDTH && buff[y * WIDTH + x+1] > 0 { neighbor += 1};    // right
    if y > 0 && buff[(y-1) * WIDTH + x] > 0 { neighbor += 1};      // above
    if y < HEIGHT && buff[(y+1) * WIDTH + x] > 0 { neighbor += 1}; // below

    if y > 0 && x > 0 && buff[(y-1) * WIDTH + (x-1)] > 0 { neighbor += 1}; // above left
    if y > 0 && x < WIDTH && buff[(y-1) * WIDTH + (x+1)] > 0 { neighbor += 1}; // above right
    if y < HEIGHT && x > 0 && buff[(y+1) * WIDTH + (x-1)] > 0 { neighbor += 1}; // below left
    if y < HEIGHT && x < WIDTH && buff[(y+1) * WIDTH + (x+1)] > 0 { neighbor += 1}; // above left

    let current: u32 = buff[y * WIDTH + x];

    // THE RULES!
    // alive and < 2 neighbors == die
    if current > 0 && neighbor < 2 {
        return DEAD
    }

    // alive and 2 or 3 neighbors == survive
    if current > 0 && (neighbor == 2 || neighbor == 3) {    
        return 0xFF | (current + ALIVE_GENERATION_INC);// * (x as u32 * y as u32))  // this line is the color of the cell
    }

    // alive and more than 3 neighbors =- die
    if neighbor > 3 {
        return DEAD
    }

    // dead and 3 neighbors == born
    if current == 0 && neighbor == 3 {
        return ALIVE_START
    }

    DEAD
}

fn main() {
    let mut game_buff: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut swap_buff: Vec<u32> = vec![0; WIDTH * HEIGHT];

    populate_game(&mut game_buff);

    let mut window = Window::new(
        "Conway's Life",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails.
        window
            .update_with_buffer(&game_buff, WIDTH, HEIGHT)
            .unwrap();

        next_generation(&game_buff,&mut swap_buff);
        mem::swap(&mut game_buff, &mut swap_buff);
        swap_buff.iter_mut().for_each(|element| *element = 0);
    }
}