extern crate termios;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;

use termios::*;

fn setup_fd(fd: RawFd) -> io::Result<()> {
    let mut termios = try!(Termios::from_fd(fd));

    termios.c_iflag = IGNPAR | IGNBRK;
    termios.c_oflag = 0;
    termios.c_cflag = CS8 | CREAD | CLOCAL;
    termios.c_lflag = 0;

    try!(cfsetspeed(&mut termios, B9600));
    try!(tcsetattr(fd, TCSANOW, &termios));
    try!(tcflush(fd, TCIOFLUSH));

    Ok(())
}

fn main() {
    let mut ser_file = match File::open("/dev/ttyUSB0") {
        Ok(val) => val,
        Err(_) => {
            println!("Unable to open port!");
            return;
        }
    };

    match setup_fd(ser_file.as_raw_fd()) {
        Ok(_) => println!("Woo!"),
        Err(_) => {
            println!("Unable to configure serial port!");
            return;
        }
    };

    let mut buffer = [0; 256];
    let mut buf_len = 0;

    match ser_file.read(&mut buffer) {
        Ok(n) => buf_len = n,
        Err(_) => {
            println!("Oh noes couldn't read!");
            return;
        }
    };

    let output = match str::from_utf8(&buffer[..buf_len]) {
        Ok(val) => val,
        Err(_) => {
            println!("Oh noe unable to parse as string!");
            return;
        }
    };

    println!("Got output: {}", output);
}
