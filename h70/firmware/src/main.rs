#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    time::{Instant, Duration},
    gpio::{Level, Output, OutputConfig},
    main,
};

#[main]
fn main() -> ! {
    let esp_config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(esp_config);

    let mut led = Output::new(peripherals.GPIO23, Level::High, OutputConfig::default());

    loop {
        led.toggle();

        // wait 500ms
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}

