#![no_std]
#![no_main]
#![feature(offset_of)]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::writeln;
use yansu::error;
use yansu::graphics::draw_test_pattern;
use yansu::graphics::fill_rect;
use yansu::graphics::Bitmap;
use yansu::info;
use yansu::init::init_basic_runtime;
use yansu::println;
use yansu::qemu::exit_qemu;
use yansu::qemu::QemuExitCode;
use yansu::uefi::init_vram;
use yansu::uefi::EfiHandle;
use yansu::uefi::EfiMemoryType;
use yansu::uefi::EfiSystemTable;
use yansu::uefi::VramTextWriter;
use yansu::warn;
use yansu::x86::hlt;



#[no_mangle]
fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    println!("Booting YansuOS...");
    println!("image_handle: {:#018X}", image_handle);
    println!("efi_system_table: {:#p}", efi_system_table);
    info!("info");
    warn!("warn");
    error!("error");
    let mut vram = init_vram(efi_system_table).expect("init_vram failed");
    let vw = vram.width();
    let vh = vram.height();
    fill_rect(&mut vram, 0x000000, 0, 0, vw, vh).expect("fill_rect failed");
    draw_test_pattern(&mut vram);
    let mut w = VramTextWriter::new(&mut vram);
    let memory_map = init_basic_runtime(image_handle, efi_system_table);
    let mut total_memory_pages = 0;
    for e in memory_map.iter() {
        if e.memory_type() != EfiMemoryType::CONVENTIONAL_MEMORY {
            continue;
        }
        total_memory_pages += e.number_of_pages();
        writeln!(w, "{e:?}").unwrap();
    }
    let total_memory_size_mib = total_memory_pages * 4096 / 1024 / 1024;
    writeln!(
        w,
        "Total: {total_memory_pages} pages = {total_memory_size_mib} MiB"
    )
    .unwrap();
    //println!("Hello, world!");
    writeln!(w, "Hello, Non-UEFI world!").unwrap();
    loop {
        hlt()
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("PANIC: {info:?}");
    exit_qemu(QemuExitCode::Fail);
}