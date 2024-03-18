use crate::{
    bcm::{self, UART, common, mailbox::MailboxTag},
    console::interface::Statistics,
    print, println,
};

const MAXCHAR: usize = 100;

fn reboot() {
    println!("Rebooting...");
    common::reset(100);
}

fn help() {
    println!("help    : print this help menu");
    println!("hello   : print Hello World!");
    println!("board   : print board rev");
    println!("status  : print UART status");
    println!("reboot  : reboot this device");
    println!("cancel  : cancel reboot");
}

pub fn interactiave_shell() -> ! {
    let mut array: [u8; MAXCHAR] = [0; MAXCHAR];
    let mut cnt = 0;

    loop {
        let c = bcm::UART.get_char();
        array[cnt] = c;
        let c = c as char;
        print!("{}", c);
        if c == '\r' {
            println!();
            match core::str::from_utf8(&array[0..cnt]).unwrap() {
                "help" => {
                    help();
                }
                "hello" => {
                    println!("Hello World!");
                }
                "reboot" => {
                    println!("Rebooting...");
                    reboot();
                }
                "cancel" => {
                    common::cancel_reset();
                }
                "board" => {
                    let (board, _) = bcm::MAILBOX.get(MailboxTag::BoardRevision);
                    let (mem0, mem1) = bcm::MAILBOX.get(MailboxTag::ArmMemory);
                    let (serial0, serial1) = bcm::MAILBOX.get(MailboxTag::BoardSerial);
                    let (vc0, vc1) = bcm::MAILBOX.get(MailboxTag::VcMemory);
                    println!("Board revision: {:x}", board);
                    println!("Board serial: {:x} {:x}", serial0, serial1);
                    println!("Arm memory(base, size): {:x}, {:x}", mem0, mem1);
                    println!("Vc memory(base, size):  {:x}, {:x}", vc0, vc1);
                }
                "status" => {
                    println!("Chars written: {}", UART.chars_written());
                }
                _ => {
                    if cnt > 0 {
                        println!(
                            "Unknown command: {:?}",
                            &core::str::from_utf8(&array[0..cnt]).unwrap()
                        );
                        help();
                    }
                }
            }

            print!("\r# ");
            cnt = 0;
        }
        cnt += if c == '\r' { 0 } else { 1 };
    }
}