#![allow(unsafe_code, unused, non_upper_case_globals)]
#![no_main]
#![no_std]
#![no_mangle]
use core::mem;
use core::ptr;
use cortex_m::asm::{nop, self};
use panic_halt as _;

use cortex_m_rt::entry;
use::core::arch::asm;
use cortex_m_semihosting::{debug, hprintln};
use stm32f3xx_hal_v2::{self as hal, pac, prelude::*,flash::ACR, pac::Peripherals, pac::FLASH, pac::Interrupt};

use volatile::Volatile;
use stm32f3xx_hal_v2::hal::blocking::rng::Read;

mod my_flash;
use my_flash::{unlock, wait_ready, clear_error_flags, erase_page, write_to_flash};

use cortex_m::peripheral::NVIC;
use stm32f3xx_hal_v2::interrupt;

const UNLOCK_KEY1: u32 = 0x4567_0123;
const UNLOCK_KEY2: u32 = 0xCDEF_89AB;

#[entry]
fn main() -> ! {

    // Get the peripheral access
    let dp = Peripherals::take().unwrap();

    // Enable the clock for GPIOA and SYSCFG

    dp.RCC.ahbenr.modify(|_, w| w.iopden().set_bit());
    dp.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());

    // Configure PA0 as input
    dp.GPIOA.moder.modify(|_, w| w.moder0().input());
    dp.SYSCFG.exticr1.modify(|_, w| w.exti0().pa0());

    // Configure EXTI0 for falling edge trigger and enable it
    dp.EXTI.imr1.modify(|_, w| w.mr0().set_bit());
    dp.EXTI.ftsr1.modify(|_, w| w.tr0().set_bit());

    // Enable EXTI0 interrupt in the NVIC
    unsafe { NVIC::unmask(Interrupt::EXTI0) };

    // Enable interrupts globally
    unsafe { cortex_m::peripheral::NVIC::unmask(Interrupt::EXTI0) };


    loop {
        // your code goes here
    }

    
}

// Interrupt handler for EXTI0
#[interrupt]
fn EXTI0() {
    // Clear the interrupt pending bit
    unsafe{
        let peripherals = Peripherals::steal();
        peripherals.EXTI.pr1.modify(|_, w| w.pr0().set_bit());
    }
    hprintln!("Interrupt happened").unwrap();
    // Your interrupt handling code here
}