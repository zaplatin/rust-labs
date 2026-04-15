#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use libm::{atan2f, sqrtf};
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};
use microbit::{
    hal::{
        twim::{Frequency, Twim},
        uarte::{Baudrate, Parity, Uarte},
        Delay,
    },
    Board,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn heading_to_direction(angle: f32) -> &'static str {
    if angle >= 337.5 || angle < 22.5 {
        "N"
    } else if angle < 67.5 {
        "NE"
    } else if angle < 112.5 {
        "E"
    } else if angle < 157.5 {
        "SE"
    } else if angle < 202.5 {
        "S"
    } else if angle < 247.5 {
        "SW"
    } else if angle < 292.5 {
        "W"
    } else {
        "NW"
    }
}

fn tilt_to_text(roll_deg: f32) -> &'static str {
    if roll_deg > 10.0 {
        "RIGHT"
    } else if roll_deg < -10.0 {
        "LEFT"
    } else {
        "LEVEL"
    }
}

fn roll_from_accel_deg(ax_mg: i32, ay_mg: i32, az_mg: i32) -> f32 {
    let ax = ax_mg as f32;
    let ay = ay_mg as f32;
    let az = az_mg as f32;

    atan2f(ax, sqrtf(ay * ay + az * az)) * 180.0 / core::f32::consts::PI
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

    let i2c = Twim::new(
        board.TWIM0,
        board.i2c_internal.into(),
        Frequency::K100,
    );

    let mut imu = Lsm303agr::new_with_i2c(i2c);
    imu.init().ok();

    imu.set_accel_mode_and_odr(
        &mut delay,
        AccelMode::Normal,
        AccelOutputDataRate::Hz10,
    )
    .ok();

    let mut imu = imu.into_mag_continuous().ok().unwrap();

    // Простая автокалибровка по X/Y
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    loop {
        let mag_result = imu.magnetic_field();
        let acc_result = imu.acceleration();

        if let (Ok(mag), Ok(acc)) = (mag_result, acc_result) {
            let raw_x = mag.x_nt();
            let raw_y = mag.y_nt();
            let raw_z = mag.z_nt();

            // обновляем границы калибровки
            if raw_x < min_x {
                min_x = raw_x;
            }
            if raw_x > max_x {
                max_x = raw_x;
            }
            if raw_y < min_y {
                min_y = raw_y;
            }
            if raw_y > max_y {
                max_y = raw_y;
            }

            // смещение центра
            let offset_x = (max_x + min_x) as f32 / 2.0;
            let offset_y = (max_y + min_y) as f32 / 2.0;

            // полуоси
            let span_x = (max_x - min_x) as f32 / 2.0;
            let span_y = (max_y - min_y) as f32 / 2.0;

            // защита от деления на ноль
            let scale_x = if span_x > 1.0 { 1.0 / span_x } else { 1.0 };
            let scale_y = if span_y > 1.0 { 1.0 / span_y } else { 1.0 };

            // откалиброванные значения
            let cal_x = (raw_x as f32 - offset_x) * scale_x;
            let cal_y = (raw_y as f32 - offset_y) * scale_y;

            // heading
            let mut heading =
                atan2f(cal_y, cal_x) * 180.0 / core::f32::consts::PI;

            if heading < 0.0 {
                heading += 360.0;
            }

            let direction = heading_to_direction(heading);

            // наклон вправо/влево в градусах
            let roll_deg = roll_from_accel_deg(acc.x_mg(), acc.y_mg(), acc.z_mg());
            let tilt = tilt_to_text(roll_deg);

            // 1-я строка: X Y Z
            write!(
                serial,
                "X: {} Y: {} Z: {}\r\n",
                raw_x, raw_y, raw_z
            )
            .ok();

            // 2-я строка: всё остальное
            write!(
                serial,
                "Heading: {} ({:.1} deg) | Tilt: {} ({:.1} deg)\r\n\r\n",
                direction,
                heading,
                tilt,
                roll_deg
            )
            .ok();
        } else {
            write!(serial, "Sensor read error\r\n\r\n").ok();
        }

        delay.delay_ms(300u32);
    }
}