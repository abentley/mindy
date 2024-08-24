use daemonize::Daemonize;
use nix::errno;
use nix::fcntl;
use nix::sys::select;
use nix::sys::stat::Mode;
use nix::unistd::mkfifo;
use std::fs::File;
use std::io::Read;
use std::os::fd::AsFd;
use std::os::fd::FromRawFd;
use std::process;

const TO_AGENT: &str = "/home/abentley/mindy/to-agent";
const FROM_AGENT: &str = "/home/abentley/mindy/from-agent";

fn become_agent() {
    let stdout = File::create("stdout-1").unwrap();
    let stderr = File::create("stderr-1").unwrap();
    eprintln!("Daemonizing");
    Daemonize::new()
        .stdout(stdout)
        .stderr(stderr)
        .start()
        .unwrap();
    eprintln!("Daemonized");
    let mut line = String::new();
    loop {
        eprintln!("Opening in_fifo");
        let in_fifo = match fcntl::open(TO_AGENT, fcntl::OFlag::O_NONBLOCK, Mode::all()) {
            Ok(x) => x,
            Err(error) => {
                panic!("Could not open myfife: {}", error)
            }
        };
        let mut fds = select::FdSet::new();
        let in_fifo = unsafe { File::from_raw_fd(in_fifo) };
        let mut in_fifo = match fcntl::Flock::lock(in_fifo, fcntl::FlockArg::LockExclusiveNonblock)
        {
            Ok(in_fifo) => in_fifo,
            Err((_, errno::Errno::EAGAIN)) => {
                eprintln!("Agent already running.  Exiting.");
                process::exit(0);
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        };
        fds.insert(in_fifo.as_fd());
        eprintln!("Waiting for in_fifo to be ready");
        select::select(None, &mut fds, None, None, None).unwrap();
        eprintln!("Reading from in_fifo");
        in_fifo.read_to_string(&mut line).unwrap();
        print!("{}", line);
        line.clear();
    }
}

fn get_value() -> String {
    format!("{}", process::id())
}

fn get_value_with_proxy() -> String {
    get_value()
}

fn main() {
    match mkfifo(TO_AGENT, Mode::S_IRWXU) {
        Ok(_) => {}
        Err(nix::Error::EEXIST) => {}
        Err(err) => {
            panic!("TO_AGENT: {}", err);
        }
    }
    match mkfifo(FROM_AGENT, Mode::S_IRWXU) {
        Ok(_) => {}
        Err(nix::Error::EEXIST) => {}
        Err(err) => {
            panic!("FROM_AGENT: {}", err);
        }
    }
    println!("{}", get_value_with_proxy());
    become_agent();
}
