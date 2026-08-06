use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("Building kernel (x86_64-unknown-none)...");
    let status = Command::new("cargo")
        .args(["+nightly", "build", "--target", "x86_64-unknown-none"])
        .status()
        .expect("Failed to execute cargo build for kernel");

    if !status.success() {
        eprintln!("Kernel build failed.");
        std::process::exit(1);
    }

    let kernel_path = PathBuf::from("target/x86_64-unknown-none/debug/rustix");
    let bios_path = PathBuf::from("target/x86_64-unknown-none/debug/boot-bios.img");
    let uefi_path = PathBuf::from("target/x86_64-unknown-none/debug/boot-uefi.img");

    println!("Generating boot images via bootloader...");
    bootloader::BiosBoot::new(&kernel_path).create_disk_image(&bios_path).unwrap();
    bootloader::UefiBoot::new(&kernel_path).create_disk_image(&uefi_path).unwrap();

    println!("\nSuccess! Boot images generated:");
    println!("  BIOS: {}", bios_path.display());
    println!("  UEFI: {}", uefi_path.display());
}
