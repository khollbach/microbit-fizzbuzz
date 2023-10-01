#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{display::blocking, hal::Timer, Board};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut display = blocking::Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    loop {
        for idx in 0..25 {
            let i = idx / 5;
            let j = idx % 5;
            let mut grid = [[0; 5]; 5];
            grid[i][j] = 1;
            let delay_ms = 100;
            display.show(&mut timer, grid, delay_ms);
        }
    }
}
