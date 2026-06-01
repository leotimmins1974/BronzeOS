use std::process::Command;

fn main() {
    let kernel_path = env!("CARGO_BIN_FILE_BRONZE_KERNEL_bronze_kernel");
    let out_dir = std::env::current_dir().unwrap();
    let bios_path = out_dir.join("target/bootimage.bios");

    let bootimage = bootloader::BiosBoot::new(std::path::Path::new(kernel_path));
    bootimage.create_disk_image(&bios_path).unwrap();

    println!(
        "Succesfully created the boot image at \"{}\"",
        bios_path.display()
    );

    // Temporary for debugging
    println!("Launching QEMU");

    let mut qemu = Command::new("qemu-system-x86_64");
    qemu.arg("-drive")
        .arg(format!("format=raw,file={}", bios_path.display()));
    qemu.arg("-m").arg("2G");

    let mut child = qemu.spawn().expect("QEMU FAILED");

    child.wait().unwrap();
}
