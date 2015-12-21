extern crate termios;

use std::str;

use std::fs::File;

use std::io;
use std::io::prelude::*;
use std::io::stdout;

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
    let mut ser_file = File::open("/dev/ttyUSB0").ok().expect("Unable to open file handle!");

    setup_fd(ser_file.as_raw_fd()).ok().expect("Unable to configure serial port!");

    let mut buffer = [0; 256];
    let mut buf_len = 0;
    
    while match ser_file.read(&mut buffer) { Ok(n) => {buf_len = n; true}, Err(_) => false} {
        if buf_len <= 0 {
            break;
        }
        print!("{}", str::from_utf8(&buffer[..buf_len]).ok().expect("Oh noes unable to parse as string!"));
        assert!(stdout().flush().is_ok());
    }

    println!("");
}
