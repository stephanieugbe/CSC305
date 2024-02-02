#![no_std]
#![no_main]
#![feature(allow_internal_unstable)] //demanded by #[allow_internal_unstable(print_internals, format_args_nl)] in my std.rs
mod writer;
mod smart_pointer_example;
mod std;

// use bootloader_api::config::Mapping;
use x86_64::instructions::hlt;
use writer::FrameBufferWriter;
//let's get heap memory allocation going
extern crate alloc;
use good_memory_allocator::SpinLockedAllocator;
use lazy_static::lazy_static;
use spin::Mutex;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

use bootloader_api::{
    config::Mapping,
    info::{MemoryRegion,MemoryRegionKind},
    
};

//Use the entry_point macro to register the entry point function: bootloader_api::entry_point!(kernel_main)
//optionally pass a custom config
pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};
bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

//use lazy static to allow declaration of static without initializing with a constant value
//Mutex from spin is used for control of threads access.
lazy_static! {
    pub(crate) static ref FRAME_BUFFER_WRITER: Mutex<FrameBufferWriter> =
        Mutex::new(FrameBufferWriter::empty());
}

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();

    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    FRAME_BUFFER_WRITER.lock().init(buffer, frame_buffer_info);
 FRAME_BUFFER_WRITER.lock().set_x_y_pos(None, Some(100));
   
 
    println!("Testing testing {} and {}. It prints", 1, 4.0 / 2.0);


//let's initialize our global memory allocator
    let last_memory_region = boot_info.memory_regions.last().unwrap();
    //get the first bootload memory
    let mut boot_loader_memory_region = MemoryRegion::empty();

    for memory_region in boot_info.memory_regions.iter() {
        match memory_region.kind {
            MemoryRegionKind::Bootloader => {
                boot_loader_memory_region = *memory_region;
                break;
            }
            _ => continue,
}
    }
    let physical_memory_offset = boot_info.physical_memory_offset.into_option().unwrap();
    
    let heap_start = boot_loader_memory_region.end + 0x1 + physical_memory_offset;
    let heap_size = last_memory_region.end - (boot_loader_memory_region.end + 0x1);

    unsafe {
        ALLOCATOR.init(heap_start as usize, heap_size as usize);
}
use alloc::boxed::Box;
let x = Box::new(33);
println!("\nValue in heap is {}", &x);

let y =33;
println!("\nValue in stack is {}", &y);
let root = create_tree();
    add_child(&root);
    print_tree(root);
    
    loop {
        hlt();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}