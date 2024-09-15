#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{Temp, Timer},
};
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);

        let mut temp = Temp::new(board.TEMP);
        loop {
            rprintln!("Reading temperature");
            let deg_c: f32 = temp.measure().to_num();
            rprintln!("Temperature: {}°C", deg_c);

            let led_display = get_display(deg_c);

            display.show(&mut timer, led_display, 10000);

            display.clear();
        }
    }

    panic!("End");
}

fn get_display(temp: f32) -> [[u8; 5]; 5] {
    let digit = (temp % 10.0) as i32;
    let mut display = match digit {
        0 => [
            [0, 0, 1, 1, 1],
            [0, 0, 1, 0, 1],
            [0, 0, 1, 0, 1],
            [0, 0, 1, 0, 1],
            [0, 0, 1, 1, 1],
        ],
        1 => [
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
        ],
        2 => [
            [0, 0, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 1, 1],
        ],
        3 => [
            [0, 0, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 1, 1, 1],
        ],
        4 => [
            [0, 0, 1, 0, 1],
            [0, 0, 1, 0, 1],
            [0, 0, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
        ],
        5 => [
            [0, 0, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 1, 1, 1],
        ],
        6 => [
            [0, 0, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 1, 1],
            [0, 0, 1, 0, 1],
            [0, 0, 1, 1, 1],
        ],
        7 => [
            [0, 0, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
        ],
        8 => [
            [0, 0, 1, 1, 1],
            [0, 0, 1, 0, 1],
            [0, 0, 1, 1, 1],
            [0, 0, 1, 0, 1],
            [0, 0, 1, 1, 1],
        ],
        9 => [
            [0, 0, 1, 1, 1],
            [0, 0, 1, 0, 1],
            [0, 0, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 1, 1, 1],
        ],
        _ => [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
    };

    let tens = (temp / 10.0) as i32;
    match tens {
        1 => {
            display[0][0] = 1;
        }
        2 => {
            display[0][0] = 1;
            display[0][1] = 1;
        }
        3 => {
            display[0][0] = 1;
            display[0][1] = 1;
            display[1][0] = 1;
        }
        4 => {
            display[0][0] = 1;
            display[0][1] = 1;
            display[1][0] = 1;
            display[1][0] = 1;
        }
        _ => {}
    };

    let decimal = ((temp * 10.0) % 10.0) as i32;
    match decimal {
        1 => {
            display[4][1] = 1;
        }
        2 => {
            display[4][0] = 1;
        }
        3 => {
            display[4][0] = 1;
            display[4][1] = 1;
        }
        4 => {
            display[3][1] = 1;
        }
        5 => {
            display[3][1] = 1;
            display[4][1] = 1;
        }
        6 => {
            display[3][1] = 1;
            display[4][0] = 1;
        }
        7 => {
            display[3][1] = 1;
            display[4][0] = 1;
            display[4][1] = 1;
        }
        8 => {
            display[3][0] = 1;
        }
        9 => {
            display[3][0] = 1;
            display[4][1] = 1;
        }
        _ => {}
    }
    display
}
