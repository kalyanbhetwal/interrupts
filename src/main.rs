#![allow(unsafe_code, unused, non_upper_case_globals)]
#![no_main]
#![no_std]
#![no_mangle]
use core::mem;
use core::ptr;
use cortex_m::asm::{nop, self};
use panic_halt as _;


use cortex_m::peripheral::SCB;
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


#[no_mangle]
fn checkpoint(lr: u32){

    unsafe {
        asm!(
            "add sp, #280"
        );
    }
    unsafe {
        asm!(
            "pop {{r7}}"
        );
    }
    unsafe {
        asm!(
            "push {{r7}}"
        );
    }
    unsafe {
        asm!(
            "sub sp, #280"
        );
    }

    let r0_value: u32;
    let r1_value: u32;
    let r2_value: u32;
    let r3_value: u32;
    let r4_value: u32;
    let r5_value: u32;
    let r6_value: u32;
    let r7_value: u32;
    let r8_value: u32;
    let r9_value: u32;
    let r10_value: u32;
    let r11_value: u32;
    let r12_value: u32;
    let r13_sp: u32;
    let r14_lr: u32;
    let r15_pc: u32;

    unsafe {
        asm!(
            "MOV {0}, r0",
            out(reg) r0_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r1",
            out(reg) r1_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r2",
            out(reg) r2_value
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r3",
            out(reg) r3_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r4",
            out(reg) r4_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r5",
            out(reg) r5_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r6",
            out(reg) r6_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r7",
            out(reg) r7_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r8",
            out(reg) r8_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r9",
            out(reg) r9_value
        );
    }

    unsafe {
        asm!(
            "MOV {0}, r10",
            out(reg) r10_value
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r11",
            out(reg) r11_value
        );
    }


    unsafe {
        asm!(
            "MOV {0}, r12",
            out(reg) r12_value
        );
    }
 
    unsafe {
        asm!(
            "MOV {0}, r14",
            out(reg) r14_lr
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r15",
            out(reg) r15_pc
        );
    }

  //  r14_lr = lr;

    unsafe {
        asm!(
            "MOV r0, sp",
        );
    }
    // have to be extra careful for the sp value
    unsafe {
        asm!(
            "add r0, #288",
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r0",
            out(reg) r13_sp
        );
    }
    
    //let dp = Peripherals::take().unwrap();

    unsafe{
    let dp = Peripherals::steal();

    let mut flash= dp.FLASH;
    unlock(& mut flash);
    wait_ready(&flash);

   
        //let  start_address: u32 = 0x2000_fffc as u32;
        let mut start_address:u32;
        let  end_address = r13_sp;
        asm!("movw r0, 0xFFF8
             movt r0, 0x2000");

         asm!(
             "MOV {0}, r0",
             out(reg) start_address
         );

         let stack_size = (start_address - end_address) + 4;
        // leaving first xyz K for program i.e start at 0x0801_0000
         let mut flash_start_address = Volatile::new(0x0803_0000);
         let mut flash_end_address = Volatile::new(0x0807_FFFF);    

        let mut checkpoint_size= Volatile::new(0u32);
        asm::dmb();
        checkpoint_size.write(stack_size+4+16*4 +4);
        asm::dmb();

        loop{
            let mut offset = ptr::read_volatile(flash_start_address.read() as *const u32);
            if offset == 0xffff_ffff{
                break;
            }
            flash_start_address.write(flash_start_address.read() + offset); 
            if flash_start_address.read() + checkpoint_size.read() >= flash_end_address.read(){
                erase_all(&mut flash);
               flash_start_address = Volatile::new(0x0803_0000);
            }
        }
        asm::dmb();
        //write the size of packet at the begining of the packet
        write_to_flash(&mut flash,  (flash_start_address.read()) as u32, checkpoint_size.read() as u32); 
        flash_start_address.write(flash_start_address.read()+4);
        asm::dmb();
           // Code that involves Flash write
    //      if offset == 0xffff_ffff {
    //   // stack_size + 4(0xffff_ffff to signal end of stack) + 16*4(store registers) + 4 (size of a packet)
    //         write_to_flash(&mut flash,  flash_start_address as u32, checkpoint_size+1-1  as u32);
    //         flash_start_address = flash_start_address + 4;
    //      }
    //      else{
    //         flash_start_address = flash_start_address + offset; 
    //         let flash_end_address = 0x0807_FFFF-1+1;
    //         if flash_end_address - flash_start_address < checkpoint_size {
    //             //clear flash
    //             //set start address and offset
    //             erase_all(&mut flash);
    //             flash_start_address = 0x0801_0004;
    //             offset = 0;
    //         }
    //         write_to_flash(&mut flash,  0x0801_0000 as u32, offset+checkpoint_size+1-1  as u32);
    //      }
    asm::dmb(); 
         while start_address >= end_address{
            let mut data = Volatile::new(0u32);
            data.write(core::ptr::read_volatile(start_address as * const u32));
            write_to_flash(&mut flash,  flash_start_address.read() as u32, data.read() as u32);
            flash_start_address.write(flash_start_address.read() +1* 4);
            // Move to the next address based on the size of the type
            start_address = start_address-4;
            
        }
        asm::dmb();
    asm::dmb();
    //mark the end of the stack
    write_to_flash(&mut flash,  (flash_start_address.read()) as u32, 0xffff_ffff as u32);
    flash_start_address.write(flash_start_address.read() + 4);
    asm::dmb();

    // for i in 0..15{
    //     write_to_flash(&mut flash,  0x0800_9060 as u32, r0_value as u32);
    //       flash_start_address = flash_start_address + 4;
    // }

    write_to_flash(&mut flash,  flash_start_address.read() as u32, r0_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+4 as u32, r1_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+8 as u32, r2_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+12 as u32, r3_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+16 as u32, r4_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+20 as u32, r5_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+24 as u32, r6_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+28 as u32, r7_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+32 as u32, r8_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+36 as u32, r9_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+40 as u32, r10_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+44 as u32, r11_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+48 as u32, r12_value as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+52 as u32, r13_sp as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+56 as u32, r14_lr as u32);
    write_to_flash(&mut flash,  flash_start_address.read()+60 as u32, r15_pc as u32);
    drop(flash);
    }     
}

fn erase_all(flash: &mut FLASH){
    let start_address = 0x0801_0000;

    for i in 0..255{
        let page = start_address + i * 2*1024;
         erase_page(flash,  page);
    }

}
fn restore()->bool{
    unsafe {
        let mut flash_start_address = 0x0803_0000;
        let packet_size = ptr::read_volatile(0x0803_0000 as *const u32);
        //let r0_flash = ptr::read_volatile(0x0800_9060 as *const u32);
        if packet_size == 0xffff_ffff {
            return false
        }
        if  ptr::read_volatile((flash_start_address + packet_size) as *const u32)==0xffff_ffff{
            return  false;
        }

        let mut offset:u32 = 0;
        // think about multiple conditions where it could break
        //1. There could multiple failed checkpoints before a successful checkpoint.
        //2. The last checkpoint could be a failed(incomplete) checkpoint.
        loop{
            
            offset = ptr::read_volatile(flash_start_address  as *const u32);
             
            if  ptr::read_volatile((flash_start_address + offset) as *const u32) == 0xffff_ffff{
                break;
            }
    
            flash_start_address+=offset;
        }
        flash_start_address+=4;
       
        // let mut end_address = 0x0801_0004 + packet_size;
        // let recent_frame_size: u32 = ptr::read_volatile(end_address as *const u32);
        // let mut recent_frame_start_address = end_address - recent_frame_size;

        asm!(
            "mov r0, {0}",
            in(reg) flash_start_address
        );

        //set sp to 0x0200_fffc
        asm!("movw r1, 0xfff8
        movt r1, 0x02000");
        asm!("msr msp, r1");

        asm!("movw r3, 0xffff
        movt r3, 0xffff");
    
        asm!("1:
            ldr r1, [r0, #4]
            cmp r1, r3
            beq 2f
            push {{r1}}
            adds r0, r0, #4
            b 1b
            2:");     

        asm!("adds r0, r0, #4");
        asm!("adds r0, r0, #4");

        asm!( "LDR r1, [r0]");
        asm!("Push {{r1}}");

        asm!("adds r0, r0, #4");
        asm!( "LDR r1, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r2, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r3, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r4, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r5, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r6, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r7, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r8, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r9, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r10, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r11, [r0]");

        asm!("adds r0, r0, #4");
        asm!( "LDR r12, [r0]");

        asm!("adds r0, r0, #4");
       // asm!( "LDR r13, [r0]"); //no need to do this

        asm!("adds r0, r0, #4");
        asm!( "LDR r14, [r0]");

        asm!("POP {{r0}}");
        asm!("mov r15, r14");
    }
    return true;
}

fn delete_pg(page: u32){
    unsafe{
    let mut dp = Peripherals::steal();
    let mut flash= &mut dp.FLASH;
    unlock(& mut flash); 
    wait_ready(&flash);
    erase_page(&mut flash,  page);
    }
}
fn delete_all_pg(){
    let start_address = 0x0803_0000;
    unsafe{
        let mut dp = Peripherals::steal();
        let mut flash= &mut dp.FLASH;
        for i in 0..25{
            let page = start_address + i * 2*1024;
            unlock(& mut flash); 
            wait_ready(&flash);
            erase_page(&mut flash,  page);
        }
    }
}

#[entry]
fn main() -> ! {

    // delete_all_pg();
   //delete_pg(0x0803_0000 as u32); 
    // Get the peripheral access
    unsafe{
    let dp = Peripherals::steal(); //take().unwrap();

    // Enable the clock for GPIOA and SYSCFG
    dp.RCC.ahbenr.modify(|_, w| w.iopden().set_bit());
    dp.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());

    // Configure PA0 as input
    dp.GPIOA.moder.modify(|_, w| w.moder0().input());
    dp.SYSCFG.exticr1.modify(|_, w| w.exti0().pa0());

    // Configure EXTI0 for falling edge trigger and enable it
    dp.EXTI.imr1.modify(|_, w| w.mr0().set_bit());
    dp.EXTI.ftsr1.modify(|_, w| w.tr0().set_bit());
    }
    // Enable EXTI0 interrupt in the NVIC
    unsafe { NVIC::unmask(Interrupt::EXTI0) };

    // Enable interrupts globally
    unsafe { cortex_m::peripheral::NVIC::unmask(Interrupt::EXTI0) };

    restore();

    let a = Volatile::new(10);
    let b = Volatile::new(20);
    let mut d; 
    let mut e; 
    let mut f;
    let mut g ;
    let mut c;
    loop {
        // your code goes here
        c = a.read() + b.read();
        hprintln!("After c").unwrap();
        d = c + a.read();
        hprintln!("After d").unwrap();
        e = c + d;
        hprintln!("After e").unwrap();
        f = e + c;
        hprintln!("After f").unwrap();
        g = c + f;
        hprintln!("After g").unwrap();
    }

    
}

// Interrupt handler for EXTI0
#[interrupt]
fn EXTI0() {
    // Clear the interrupt pending bit
    let lr: u32;
    unsafe {
        asm!(
            "mov {}, lr",
            out(reg) lr
        );
    }
    hprintln!("LR value: {:#010x}", lr).unwrap();

    unsafe{
        let peripherals = Peripherals::steal();
        peripherals.EXTI.pr1.modify(|_, w| w.pr0().set_bit());
    }
    hprintln!("Interrupt happened").unwrap();
    checkpoint(lr);
    hprintln!("Checkpoint taken").unwrap();
    let a = 10 + 2;
    let b = a + 10;
    //reset_mcu();
    // Your interrupt handling code here
}

#[no_mangle]
fn reset_mcu() -> ! {
    // Perform a software reset
    hprintln!("reset mcu").unwrap();
    SCB::sys_reset();
}