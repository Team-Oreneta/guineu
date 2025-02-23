// Disable linking to the rust standard library
// This is needed because the standard library relies on system functions.
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use fs::tar::Ramdisk;

mod tab_handler;
mod framebuffer;
mod fs;
mod gdt;
mod idt;
mod input;
mod irq;
mod isrs;
mod keyboard;
mod mb_utils;
mod oiff;
mod ports;
mod system;
mod text;
mod timer;
mod alloc;

// Define the panic handler function
#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

// Entry point of the kernel
#[no_mangle]
pub unsafe extern "C" fn kmain(multiboot_info_address: usize) -> ! {
    // Initialize the GDT, IDT, ISR, IRQ, timer, and keyboard
    gdt::init_gdt();
    idt::init_idt();
    isrs::init_isrs();
    irq::init_irqs();
    timer::init_timer();
    alloc::init_alloc();                                                                      
    keyboard::init_keyboard();

    // Use the multiboot information structure
    let multiboot_info = mb_utils::use_multiboot(multiboot_info_address);
    // Get the framebuffer from the multiboot structure
    let fb = mb_utils::get_framebuffer(&multiboot_info);
    // Set the default framebuffer for text output
    text::set_default_framebuffer(fb);
    // Find the address of the first module.
    let initrd_address = mb_utils::get_module(&multiboot_info);

    // Create the initial ramdisk
    let initrd = Ramdisk::new(initrd_address);

    // Load the logo from the ramdisk
    let logo = initrd.get_file("./guineu-logo.oiff").unwrap();

    // Set up bump alloc
    // let alloc = alloc::alloc(4) as *mut u32;
    // *alloc = 42;

    // Display boot messages
    text::WRITER.lock().boot_message(logo);
    text::WRITER.lock().boot_message_loaded();

    let test_file = initrd.get_file("./etc/hello.txt").unwrap();
    println!(
        "The file {}'s length is {}, and the contents are:\n",
        test_file.read_name(),
        test_file.read_size()
    );
    test_file.write_contents();

    // FIXME: This should be dynamically allocated.
    let mut buffer = [0u8; 128];
    print!(">");

    let n_chars = input::get_user_input(&mut buffer);
    let inputted_string = core::str::from_utf8(&buffer).unwrap();
    println!("You entered {} chars: {}", n_chars, inputted_string);

    let mut main_tab_handler = tab_handler::TabHandler::new();

    // Infinite loop to keep the kernel running
    loop {
        let n_chars = input::get_user_input(&mut buffer);
        let inputted_string = core::str::from_utf8(&buffer).unwrap();
        keyboard::map_key(0x01, main_tab_handler.switch_tab());
    }
}
