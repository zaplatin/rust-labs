#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    hal::{
        uarte::{Baudrate, Parity, Uarte},
        Delay,
    },
    Board,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let mut serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    let mut delay = Delay::new(board.SYST);

    loop {
        write!(serial, "Hello from microbit!\r\n").ok();
        delay.delay_ms(1000u32);
    }
}