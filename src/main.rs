use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use daemonize::Daemonize;

fn main() {
    println!("Hello, world!");
    let stdout = File::create("stdout-1").unwrap();
    let Ok(daemon) = Daemonize::new().stdout(stdout).start().unwrap();
    loop {
        println!("Hello underworld!");
        sleep(Duration::from_secs(1));
    }
}
