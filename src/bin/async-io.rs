use io_uring::{opcode, types, IoUring};
use std::os::unix::io::AsRawFd;
use std::{fs, io};


fn main() {
    let mut ring = IoUring::new(8).unwrap();
}