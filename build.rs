#[cfg(target_os = "linux")]
fn install_gstreamer() {
    println!("linux");
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
