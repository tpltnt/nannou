use std::process::Command;

#[cfg(target_os = "linux")]
fn install_gstreamer() {
    println!("Linux detected");
    // Fedora / Red Hat
    if Command::new("which")
        .args(&["dnf"])
        .status()
        .unwrap()
        .success()
    {
        println!("Fedora detected via dnf");
    }

    // Debian / Ubuntu
    if Command::new("which")
        .args(&["apt"])
        .status()
        .unwrap()
        .success()
    {
        println!("Debian detected via apt");
    }

    // Arch Linux
    if Command::new("which")
        .args(&["pacman"])
        .status()
        .unwrap()
        .success()
    {
        println!("Arch detected via pacman");
    }
}

#[cfg(target_os = "macos")]
fn install_gstreamer() {
    println!("no automatic gstreamer installation");
}

#[cfg(target_os = "windows")]
fn install_gstreamer() {
    println!("no automatic gstreamer installation");
}

fn main() {
    install_gstreamer();
}
