#![no_main]
#![no_std]

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use hal::{
    gpio,
    gpio::{
        p0::{Parts, P0_04},
        Output, Pin, PushPull,
    },
    pac::{self, Peripherals},
    prelude::*,
    pwm,
    pwm::Pwm,
    Timer,
};
use nrf52833_hal as hal;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let perphs = Peripherals::take().unwrap();
    let gp_port0 = gpio::p0::Parts::new(perphs.P0);
    let gp_port1 = gpio::p1::Parts::new(perphs.P1);
    let pwm_pin = gp_port0
        .p0_02
        .into_push_pull_output(gpio::Level::Low)
        .degrade();

    let btn_a = gp_port0.p0_14.into_pullup_input();
    let btn_b = gp_port0.p0_23.into_pullup_input();

    let nrf_pwm = Pwm::new(perphs.PWM0);
    let mut timer = Timer::new(perphs.TIMER1);

    nrf_pwm.set_output_pin(pwm::Channel::C0, pwm_pin);
    nrf_pwm.set_prescaler(pwm::Prescaler::Div32);
    nrf_pwm.set_max_duty(10_000_u16);

    let mut cur_duty = 570_u16;

    nrf_pwm.set_duty_off(pwm::Channel::C0, cur_duty);
    rprintln!("Wait while the servo resets...");
    timer.delay_ms(2_000_u16);
    rprintln!("Ready!");

    loop {
        while btn_a.is_low().unwrap() && cur_duty <= 1100_u16 {
            nrf_pwm.set_duty_off(pwm::Channel::C0, cur_duty);
            cur_duty += 20_u16;
            timer.delay_ms(25_u16)
        }
        while btn_b.is_low().unwrap() && cur_duty >= 270_u16 {
            nrf_pwm.set_duty_off(pwm::Channel::C0, cur_duty);
            cur_duty -= 20_u16;
            timer.delay_ms(25_u16)
        }

        while btn_a.is_low().unwrap() && btn_b.is_low().unwrap() {
            cur_duty = 570_u16;
            nrf_pwm.set_duty_off(pwm::Channel::C0, cur_duty);
            timer.delay_ms(200_u16)
        }
    }
}
