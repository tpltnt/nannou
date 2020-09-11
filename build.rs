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
        Command::new("dnf")
            .args(&[
                "install",
                "gstreamer1-devel",
                "gstreamer1-plugins-base-tools",
                "gstreamer1-devel-docs",
                "gstreamer1-plugins-base-devel",
                "gstreamer1-plugins-base-devel-docs",
                "gstreamer1-plugins-good",
                "gstreamer1-plugins-good-extras",
                "gstreamer1-plugins-ugly",
                "gstreamer1-plugins-ugly-devel-docs",
                "gstreamer1-plugins-bad-free",
                "gstreamer1-plugins-bad-free-devel",
                "gstreamer1-plugins-bad-free-extras",
            ])
            .status()
            .expect("failed to install gstreamer via dnf");
    }

    // Debian / Ubuntu
    if Command::new("which")
        .args(&["apt"])
        .status()
        .unwrap()
        .success()
    {
        println!("Debian detected via apt");
        Command::new("apt-get")
            .args(&[
                "install",
                "libgstreamer1.0-0",
                "gstreamer1.0-plugins-base",
                "gstreamer1.0-plugins-good",
                "gstreamer1.0-plugins-bad",
                "gstreamer1.0-plugins-ugly",
                "gstreamer1.0-libav",
                "gstreamer1.0-doc",
                "gstreamer1.0-tools",
                "gstreamer1.0-x",
                "gstreamer1.0-alsa",
                "gstreamer1.0-gl",
                "gstreamer1.0-gtk3",
                "gstreamer1.0-qt5",
                "gstreamer1.0-pulseaudio",
            ])
            .status()
            .expect("failed to install gstreamer via apt");
        println!("kdgje");
    }

    // Arch Linux
    if Command::new("which")
        .args(&["pacman"])
        .status()
        .unwrap()
        .success()
    {
        println!("Arch detected via pacman");
        Command::new("pacman")
            .args(&[
                "-S",
                "gstreamer",
                "gst-libav",
                "gst-plugins-bad",
                "gst-plugins-base",
                "gst-plugins-good",
                "gst-plugins-ugly",
                "gst-plugin-libde265",
            ])
            .status()
            .expect("failed to install gstreamer via pacman");
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
