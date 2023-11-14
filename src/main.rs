use minifb::{Key, Window, WindowOptions};
use std::mem;

const WIDTH: usize = 480*3;
const HEIGHT: usize = 480*2;
const ALIVE_START: u32 = 0xFFFF00; // Yellow
const ALIVE_GENERATION_INC: u32 = 0;
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
fn to_rgb(r: usize, g: usize, b: usize, i: usize) -> u32 {
    ((r << 16) | (g << 8) | i % 255) as u32
}

#[inline(always)]
fn is_alive(buff: &[u32], x: usize, y: usize) -> u32  {
    const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
        (0, 1), (1, 0), (0, -1), (-1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)
    ];
    let mut neighbor = 0;

    for &(dx, dy) in &NEIGHBOR_OFFSETS {
        let nx = x.wrapping_add(dx as usize);
        let ny = y.wrapping_add(dy as usize );
        if nx < WIDTH && ny < HEIGHT && buff[ny * WIDTH + nx] > 0 {
            neighbor += 1;
        }
    }

    match buff[y * WIDTH + x] {
        // alive and < 2 neighbors == die from loneliness
        current if current > 0 && neighbor < 2 => DEAD,

        // alive and 2 or 3 neighbors == survive
        current if current > 0 && (neighbor == 2 || neighbor == 3) => current,

        // alive and more than 3 neighbors == die for overcrowding
        current if neighbor > 3 => DEAD,

        // dead and 3 neighbors == born
        0 if neighbor == 3 => ALIVE_START,

        _ => DEAD,
    }
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
        window
            .update_with_buffer(&game_buff, WIDTH, HEIGHT)
            .unwrap();

        next_generation(&game_buff,&mut swap_buff);
        mem::swap(&mut game_buff, &mut swap_buff);
        swap_buff.iter_mut().for_each(|element| *element = 0);
    }
}