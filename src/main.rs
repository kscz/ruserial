extern crate termios;

use std::str;

use std::fs::File;

use std::io;
use std::io::prelude::*;
use std::io::stdout;

use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;

use termios::*;

// TODO: pass in baud rate
fn setup_fd(fd: RawFd) -> io::Result<()> {
    let mut termios = try!(Termios::from_fd(fd));

    termios.c_iflag = IGNPAR | IGNBRK;
    termios.c_oflag = 0;
    termios.c_cflag = CS8 | CREAD | CLOCAL;
    termios.c_lflag = 0;

    // TODO: figure out why 115200 isn't an option!
    try!(cfsetspeed(&mut termios, B9600));
    try!(tcsetattr(fd, TCSANOW, &termios));
    try!(tcflush(fd, TCIOFLUSH));

    Ok(())
}

fn main() {
    // TODO: option parsing for serial port, baud, and hardware/software handshaking
    let mut ser_file = File::open("/dev/ttyUSB0").ok().expect("Unable to open file handle to serial port!");

    setup_fd(ser_file.as_raw_fd()).ok().expect("Unable to configure serial port!");

    // TODO: Is there a way to limit the existence of "buffer" to the loop?  I don't really need
    //       an eternal mutable buffer reference
    let mut buffer = [0; 256];
    while let Ok(n) = ser_file.read(&mut buffer) {
        // Catch EOF and other "not-quite-errors"
        if n <= 0 {
            break;
        }

        // Woo! Data! Print and flush
        print!("{}", str::from_utf8(&buffer[..n]).unwrap());
        stdout().flush().ok().expect("Unable to flush stdout!");
    }

    // Make sure we print out a newline to cap off output
    println!("");
}
