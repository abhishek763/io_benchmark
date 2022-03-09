use io_uring::{opcode, types, IoUring};
use std::os::unix::io::AsRawFd;
use std::{fs, io, str};
use std::time::{Instant};
use std::io::{ IoSliceMut};
use rand::Rng;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "")]
    inp_file: String,

    #[clap(short, long, default_value = "")]
    out_file: String,

    #[clap(short, long, default_value_t = 4096)]
    buf_size: usize,

    #[clap(short, long, default_value_t = 8)]
    num_concurrent: usize,

    #[clap(short, long)]
    read: bool,

    #[clap(short, long)]
    write: bool,

    #[clap(short, long)]
    copy: bool,

    #[clap(short, long, default_value_t = 2)]
    vector_len: usize,

    #[clap(short, long, default_value_t = 3)]
    test_duration: u64,

    #[clap(long)]
    random: bool,

}

const RING_SIZE: u32 = 8192;

pub fn test_file_write_read() {
    let ring = io_uring::IoUring::new(1).unwrap();
    let mut probe = io_uring::Probe::new();

    if ring.submitter().register_probe(&mut probe).is_err() {
        eprintln!("No probe supported");
        return; 
    }

    if probe.is_supported(io_uring::opcode::Read::CODE) {
        println!("Read is supported!");
    } else {
        println!("Read is NOT supported!");
    }

    if probe.is_supported(io_uring::opcode::Readv::CODE) {
        println!("Readv is supported!");
    } else {
        println!("Readv is NOT supported!");
    }
}

fn copy_async(args : &Args) -> io::Result<()> {
    let mut ring = IoUring::new(8).unwrap();
    let read_fd = fs::File::open(&args.inp_file)?;
    let write_fd = fs::File::create(&args.out_file)?;
    let mut buf = vec![0; 4096];
    let read_buf = vec![IoSliceMut::new(&mut buf)];

    let read_e = opcode::Readv::new(types::Fd(read_fd.as_raw_fd()), read_buf.as_ptr().cast(), read_buf.len() as _)
                 .build()
                 .user_data(0x42);

    unsafe {
        ring.submission()
            .push(&read_e)
            .expect("Submission Queue Full");
    }
    ring.submit_and_wait(1)?;

    let cqe = ring.completion().next().expect("completion queue is empty");

    assert_eq!(cqe.user_data(), 0x42);
    assert!(cqe.result() >= 0, "read error: {}", cqe.result());
    println!("read {} bytes", cqe.result());

    let write_e = opcode::Writev::new(types::Fd(write_fd.as_raw_fd()), read_buf.as_ptr().cast(), read_buf.len() as _)
                    .build()
                    .user_data(0x43);

    unsafe {
        ring.submission()
            .push(&write_e)
            .expect("Submission Queue Full");
    }

    ring.submit_and_wait(1)?;

    let cqe = ring.completion().next().expect("completion queue is empty");

    assert_eq!(cqe.user_data(), 0x43);
    assert!(cqe.result() >= 0, "write error: {}", cqe.result());
    println!("write {} bytes", cqe.result());
    Ok(())
}

fn read_async(args : Args) -> io::Result<(u64, u64)> {
    let mut ring = IoUring::new(RING_SIZE).unwrap();
    let read_fd  = fs::File::open(&args.inp_file)?;
    let num_concurrent = args.num_concurrent;

    let mut buf = vec![vec![vec![0 as u8; args.buf_size]; args.vector_len]; num_concurrent];
    let raw = &mut buf as *mut Vec<Vec<Vec<u8>>>;
    let mut offsets = vec![0 as i64; num_concurrent];

    let mut read_buf : Vec<Vec<IoSliceMut> > = Vec::new();

    unsafe {
        for i in 0..num_concurrent {
            let mut v = Vec::new();
            for j in 0..args.vector_len {
                v.push(IoSliceMut::new(& mut (*raw)[i][j]));
            }
            read_buf.push(v);
        }
    }

    for i in 0..num_concurrent {
        offsets[i] = (i as i64) * (args.vector_len as i64) * (args.buf_size as i64);
    }

    let offset_incr : i64 = (args.buf_size as i64) * (args.vector_len as i64) * (num_concurrent as i64);

    let file_size : i64 = fs::metadata(&args.inp_file)?.len() as i64;

    let mut read_size = 0;

    let now = Instant::now();
    while now.elapsed().as_secs() < args.test_duration {
        for j in 0..num_concurrent {
            unsafe {
                let read_e = opcode::Readv::new(types::Fd(read_fd.as_raw_fd()), read_buf[j].as_ptr().cast(), read_buf[j].len() as _)
                 .offset(offsets[j])
                 .build()
                 .user_data(j as u64);

                ring.submission().push(&read_e)
                    .expect("Submission Queue Full");  
            }
        }
        ring.submit_and_wait(num_concurrent).expect("read submit failed");

        // If it is a random read, we need to randomize offsets
        if args.random {
            for i in 0..num_concurrent {
                offsets[i] = rand::thread_rng().gen_range(0, file_size-offset_incr);
            }
        } else {
            for i in 0..num_concurrent {
                offsets[i] = (offsets[i] + offset_incr) % file_size;
            }
        }
        ring.completion().for_each(|x| {
            assert!(x.result() >= 0, "read failed {}", x.result());
            read_size += x.result() as u64;
        });
    }
    let elapsed = now.elapsed().as_millis();
    Ok((read_size as u64, elapsed as u64))
}

fn write_async(args: Args) -> io::Result<(u64, u64)> {
    let mut ring = IoUring::new(RING_SIZE).unwrap();
    let write_fd = fs::File::create(&args.out_file)?;
    let num_concurrent = args.num_concurrent;
    let mut buf = vec![vec![vec![87 as u8; args.buf_size]; args.vector_len]; num_concurrent];
    let raw = &mut buf as *mut Vec<Vec<Vec<u8>>>;
    let mut offsets = vec![0 as i64; num_concurrent];

    for i in 0..num_concurrent {
        offsets[i] = (i as i64) * (args.vector_len as i64) * (args.buf_size as i64);
    }

    let offset_incr : i64 = (args.buf_size as i64) * (args.vector_len as i64) * (num_concurrent as i64);

    let mut write_buf : Vec<Vec<IoSliceMut> > = Vec::new();
    unsafe {
        for i in 0..num_concurrent {
            let mut v = Vec::new();
            for j in 0..args.vector_len {
                v.push(IoSliceMut::new(& mut (*raw)[i][j]));
            }
            write_buf.push(v);
        }
    }

   let mut write_size = 0;
    let now = Instant::now();
    while now.elapsed().as_secs() < args.test_duration {
        for j in 0..num_concurrent {
            unsafe {
                let write_e = opcode::Writev::new(types::Fd(write_fd.as_raw_fd()), write_buf[j].as_ptr().cast(), write_buf[j].len() as _)
                 .offset(offsets[j])
                 .build()
                 .user_data(j as u64);

                ring.submission().push(&write_e)
                    .expect("Submission Queue Full");  
            }
        }
        ring.submit_and_wait(num_concurrent).expect("write submit failed");
        for j in 0..num_concurrent {
            offsets[j] += offset_incr;
        }
        ring.completion().for_each(|x| {
            assert!(x.result() >= 0, "write failed {}", x.result());
            write_size += x.result() as u64;
        });
    }
    let elapsed = now.elapsed().as_millis();
    Ok((write_size as u64, elapsed as u64))
}

fn main() {
    let args = Args::parse();
    if args.read {
        let (bytes, ms) = read_async(args).unwrap();
        print!("read,{:.4}MB,{:.2}MB/s", (bytes as f64) / 1000000.0, (bytes as f64) / (1000.0 * ms as f64));
    } else if args.write {
        let (bytes, ms) = write_async(args).unwrap();
        print!("write,{:.4}MB,{:.2}MB/s", (bytes as f64) / 1000000.0, (bytes as f64) / (1000.0 * ms as f64));
    } else {
        copy_async(&args).unwrap();
    }
}