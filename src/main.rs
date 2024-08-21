use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;
use daemonize::Daemonize;
use nix::unistd::mkfifo;
use nix::sys::stat::Mode;
use nix;

fn handle() -> (){
    let mut line = String::new();
    loop {
        let mut myfile = BufReader::new(File::open("myfife").unwrap());
        myfile.read_line(&mut line);
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
    handle()
    /*let stdout = File::create("stdout-1").unwrap();
    let daemon = Daemonize::new().stdout(stdout).start().unwrap();
    loop {
        println!("Hello underworld!");
        sleep(Duration::from_secs(1));
    }*/
}
