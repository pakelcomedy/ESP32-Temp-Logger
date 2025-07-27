#![no_std]
#![no_main]

use esp32_hal::{
    clock::ClockControl,
    delay::Delay,
    peripherals::Peripherals,
    prelude::*,
    uart::Uart,
};
use panic_halt as _;
use core::fmt::Write;
use xtensa_lx::timer; // for get_cycle_count

#[entry]
fn main() -> ! {
    // Ambil handle ke semua peripheral
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.DPORT.split();

    // Konfigurasi clock & freeze
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Delay abstraction
    let mut delay = Delay::new(&clocks);

    // UART0 untuk serial output
    let mut uart = Uart::new(peripherals.UART0, &clocks);

    writeln!(uart, "\r\n[BOOT] ESP32 Temp Monitor").ok();

    loop {
        // Simulasi pembacaan suhu
        let temp = get_fake_temp();

        // Tulis ke serial
        writeln!(uart, "Suhu: {:.2} °C", temp).ok();

        // Tunggu 5 detik
        delay.delay_ms(5000u32);
    }
}

// Fungsi dummy untuk simulasi suhu (25.00–29.99 °C)
fn get_fake_temp() -> f32 {
    let ticks = unsafe { timer::get_cycle_count() };
    let frac = (ticks % 400) as f32 / 100.0;
    25.0 + frac
}
