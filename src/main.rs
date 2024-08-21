use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::thread;
use std::time::Duration;
use daemonize::Daemonize;
use nix::unistd::mkfifo;
use nix::sys::stat::Mode;
use nix;

fn handle() -> (){
    let stdout = File::create("stdout-1").unwrap();
    let stderr = File::create("stderr-1").unwrap();
    let daemon = Daemonize::new().stdout(stdout).stderr(stderr).start().unwrap();
    print!("Daemonized");
    let mut line = String::new();
    loop {
        let mut fifo = match File::open("home/abentley/mindy/myfife") {
            Ok(x) => x,
            Err(error) => {panic!("Could not open myfife: {}", error)},
        };
        fifo.read_to_string(&mut line);
        print!("{}", line);
    }
}

fn main() {
    println!("Hello, world!");
    match mkfifo("myfife", Mode::S_IRWXU) {
        Ok(_) => {},
        Err(nix::Error::EEXIST) =>{},
        Err(err) => {panic!("{}", err);}
    }
    thread::spawn(|| handle());
    thread::sleep(Duration::from_secs(10));
    println!("Goodbye, world!");
}
