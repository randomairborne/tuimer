use std::{io::Write, path::PathBuf};

fn main() {
    #[cfg(target_os = "macos")]
    let place = std::env::var("HOME").expect("No HOME env var") + "/Library/tuimer/time.txt";
    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    let place = std::env::var("HOME").expect("No HOME env var") + "/.local/tuimer/time.txt";
    #[cfg(target_os = "windows")]
    let place = std::env::var("LocalAppData").expect("No LocalAppData env var") + "\\tuimer\\time.txt";
    println!("{}", &place);
    let path = PathBuf::from(place).canonicalize().expect("Failed to resolve path");
    if !path.exists() {
        std::fs::create_dir_all(&path.parent().unwrap()).expect("Failed to create directory");
        std::fs::write(&path, "1").expect("Failed to create new file");
    }
    let mut time: u64 = std::fs::read_to_string(&path)
        .expect("Failed to read file")
        .trim()
        .parse()
        .expect("Failed to parse time as int");
    loop {
        let seconds = time % 60;
        let minutes = (time / 60) % 60;
        let hours = time / 3600;
        print!("\r{:02}:{:02}:{:02}", hours, minutes, seconds,);
        std::io::stdout().flush().unwrap();
        time += 1;
        std::fs::write(&path, time.to_string()).ok();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
