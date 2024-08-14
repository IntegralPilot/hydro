#![no_std]
#![no_main]

extern crate alloc;

use alloc::{format, vec};
use alloc::vec::Vec;
use breadcrumbs::LogListener;
use hydro_os::{println, LOGS_ENABLED};
use hydro_os::task::{executor::Executor, keyboard, Task};
use hydro_os::wasm::run_from_bytes;
use bootloader::{entry_point, BootInfo};
use hydro_os::vga_buffer::{Color, _println_with_color};
use core::panic::PanicInfo;

static mut COLOR_INDEX: usize = 0;

struct HydroLogListener(Vec<Color>);

impl LogListener for HydroLogListener {
    fn on_log(&mut self, log: breadcrumbs::Log) {
        if !*LOGS_ENABLED.lock() {
            return;
        }
        // choose a color
        let color = unsafe {
            let color = self.0[COLOR_INDEX];
            COLOR_INDEX = (COLOR_INDEX + 1) % self.0.len();
            color
        };

        // print the log with the chosen color
        _println_with_color(format!("{}", log).as_str(), color);

        log.remove();
    }
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use hydro_os::allocator;
    use hydro_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    println!("Made heap!");

    hydro_os::init();

    breadcrumbs::init!(HydroLogListener( vec![Color::Blue, Color::Green, Color::Cyan, Color::Red, Color::Magenta, Color::Brown, Color::LightGray, Color::DarkGray, Color::LightBlue, Color::LightGreen, Color::LightCyan, Color::LightRed, Color::Pink, Color::Yellow, Color::White]));

    run_from_bytes(include_bytes!("../apps/hello.wasm")).unwrap();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hydro_os::hlt_loop();
}