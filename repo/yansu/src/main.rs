#![no_std]
#![no_main]
#![feature(offset_of)]

use core::panic::PanicInfo;
use core::time::Duration;
use yansu::error;
use yansu::executor::Executor;
use yansu::executor::Task;
use yansu::executor::TimeoutFuture;
use yansu::hpet::global_timestamp;
use yansu::info;
use yansu::init::init_allocator;
use yansu::init::init_basic_runtime;
use yansu::init::init_display;
use yansu::init::init_hpet;
use yansu::init::init_paging;
use yansu::print::hexdump;
use yansu::print::set_global_vram;
use yansu::println;
use yansu::qemu::exit_qemu;
use yansu::qemu::QemuExitCode;
use yansu::serial::SerialPort;
use yansu::uefi::init_vram;
use yansu::uefi::locate_loaded_image_protocol;
use yansu::uefi::EfiHandle;
use yansu::uefi::EfiSystemTable;
use yansu::warn;
use yansu::x86::init_exceptions;
use yansu::x86::trigger_debug_interrupt;

#[no_mangle]
fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    println!("Booting YansuOS...");
    println!("image_handle: {:#018X}", image_handle);
    println!("efi_system_table: {:#p}", efi_system_table);

    let loaded_image_protocol = locate_loaded_image_protocol(image_handle, efi_system_table)
        .expect("Failed to get LoadedImageProtocol");
    println!("image_base: {:#018X}", loaded_image_protocol.image_base);
    println!("image_size: {:#018X}", loaded_image_protocol.image_size);

    info!("info");
    warn!("warn");
    error!("error");

    hexdump(efi_system_table);

    let mut vram = init_vram(efi_system_table).expect("init_vram failed");

    init_display(&mut vram);
    set_global_vram(vram);

    let acpi = efi_system_table.acpi_table().expect("ACPI table not found");

    let memory_map = init_basic_runtime(image_handle, efi_system_table);

    info!("Hello, Non-UEFI world!");
    init_allocator(&memory_map);

    // let cr3 = yansu::x86::read_cr3();
    // println!("cr3 = {cr3:#p}");

    let (_gdt, _idt) = init_exceptions();

    trigger_debug_interrupt();
    info!("Execution continued.");

    let (_gdt, _idt) = init_exceptions();
    init_paging(&memory_map);

    init_hpet(acpi);
    let t0 = global_timestamp();

    let task1 = Task::new(async move {
        for i in 100..=103 {
            info!("{i} hpet.main_counter = {:?}", global_timestamp() - t0);
            TimeoutFuture::new(Duration::from_secs(1)).await;
        }
        Ok(())
    });

    let task2 = Task::new(async move {
        for i in 200..=203 {
            info!("{i} hpet.main_counter = {:?}", global_timestamp() - t0);
            TimeoutFuture::new(Duration::from_secs(2)).await;
        }
        Ok(())
    });

    let serial_task = Task::new(async {
        let sp = SerialPort::default();
        if let Err(e) = sp.loopback_test() {
            error!("{e:?}");
            return Err("serial: loopback test failed");
        }
        info!("Started to monitor serial port");
        loop {
            if let Some(v) = sp.try_read() {
                let c = char::from_u32(v as u32);
                info!("serial input: {v:#04X} = {c:?}");
            }
            TimeoutFuture::new(Duration::from_millis(20)).await;
        }
    });
    let mut executor = Executor::new();
    executor.enqueue(task1);
    executor.enqueue(task2);
    executor.enqueue(serial_task);
    Executor::run(executor)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("PANIC: {info:?}");
    exit_qemu(QemuExitCode::Fail);
}
