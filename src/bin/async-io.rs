use io_uring::{opcode, types, IoUring, squeue};
use std::os::unix::io::AsRawFd;
use std::{fs, io, str};
use std::time::{Instant};

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
    sequential: bool,

    #[clap(short, long)]
    copy: bool,

    #[clap(short, long)]
    read: bool,

    #[clap(short, long)]
    write: bool,

    #[clap(short, long)]
    amortize_cqe: bool,

}

const RING_SIZE: u32 = 8192;

fn copy_async(inp_file_name : &str, out_file_name: &str) -> io::Result<()> {
    let mut ring = IoUring::new(8).unwrap();
    let read_fd = fs::File::open(inp_file_name)?;
    let write_fd = fs::File::create(out_file_name)?;
    let mut buf = vec![0; 1024];

    let read_e = opcode::Read::new(types::Fd(read_fd.as_raw_fd()), buf.as_mut_ptr(), buf.len() as _)
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
    let s = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    

    let write_e = opcode::Write::new(types::Fd(write_fd.as_raw_fd()), s.as_ptr(), s.len() as _)
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

fn copy_async_buf(inp_file_name : &str, out_file_name: &str) -> io::Result<()> {
    let mut ring = IoUring::new(8192).unwrap();
    let read_fd  = fs::File::open(inp_file_name)?;
    let write_fd = fs::File::create(out_file_name)?;
    let size : usize = fs::metadata(inp_file_name)?.len() as usize;
    let mut buf = vec![0; 4096];

    let read_e = opcode::Read::new(types::Fd(read_fd.as_raw_fd()), buf.as_mut_ptr(), buf.len() as _)
                 .offset(-1)
                 .build()
                 .user_data(0x1)
                 .flags(squeue::Flags::IO_LINK);

    let write_e = opcode::Write::new(types::Fd(write_fd.as_raw_fd()), buf.as_ptr(), buf.len() as _)
                 .offset(-1)
                 .build()
                 .user_data(0x2);

    let times = (size+4095)/buf.len();
    let mut events = 0;
    let mut done = 0;
    for i in 0..times {
        unsafe {
            ring.submission().push(&read_e)
                .expect("Submission Queue Full");
            events += ring.submit_and_wait(1)?;
            let cqe = ring.completion().collect::<Vec<_>>();
            done += cqe.len();
            ring.submission().push(&write_e)
                .expect("Submission Queue Full");
            events += ring.submit_and_wait(1)?;
            let cqe = ring.completion().collect::<Vec<_>>();
            done += cqe.len();
        }
    }
   

    println!("size: {} times: {} events: {} done: {}", size, times, events, done);
   
    Ok(())
}

fn copy_async_parallel(inp_file_name : &str, out_file_name: &str) -> io::Result<()> {
    let mut ring = IoUring::new(8192).unwrap();
    let read_fd  = fs::File::open(inp_file_name)?;
    let write_fd = fs::File::create(out_file_name)?;
    let size : usize = fs::metadata(inp_file_name)?.len() as usize;
    let num_bufs = 32;
    let mut buf = vec![vec![0; 4096]; num_bufs];
    let mut offsets = vec![0 as i64; num_bufs];

    for i in 0..num_bufs {
        offsets[i] = (i as i64) * (buf[0].len() as i64);
    }

    let offset_incr : i64 = 4096 * (buf.len() as i64);

    let times = (size+4095)/(buf.len()*buf[0].len());

    for i in 0..times {
        println!("{}", i);
        for j in 0..num_bufs {
            unsafe {
                let read_e = opcode::Read::new(types::Fd(read_fd.as_raw_fd()), buf[j].as_mut_ptr(), buf[j].len() as _)
                 .offset(offsets[j])
                 .build()
                 .user_data(j as u64);

                ring.submission().push(&read_e)
                    .expect("Submission Queue Full");  
            }
        }
        ring.submit_and_wait(num_bufs).expect("read submit failed");

        for j in 0..num_bufs {
            unsafe {
                let write_e = opcode::Write::new(types::Fd(write_fd.as_raw_fd()), buf[j].as_ptr(), buf[j].len() as _)
                    .offset(offsets[j])
                    .build()
                    .user_data(j as u64);
                
                ring.submission().push(&write_e)
                    .expect("Submission Queue Full");
            }
        }
        for j in 0..num_bufs {
            offsets[j] += offset_incr;
        }
        ring.submit_and_wait(num_bufs).expect("write submit failed");
        let cqe_len = ring.completion().len(); // .collect::<Vec<_>>();
        if cqe_len > 8000 {
            // Need to benchmark if greedy exhaustion is really better?
            ring.completion().for_each(drop);
        }
    }
    Ok(())
}

fn read_async(args : Args) -> io::Result<u64> {
    let mut ring = IoUring::new(RING_SIZE).unwrap();
    let read_fd  = fs::File::open(&args.inp_file)?;
    let size : usize = fs::metadata(&args.inp_file)?.len() as usize;
    let num_bufs = args.num_concurrent;
    let mut buf = vec![vec![0; args.buf_size]; num_bufs];
    let mut offsets = vec![0 as i64; num_bufs];

    for i in 0..num_bufs {
        offsets[i] = (i as i64) * (buf[0].len() as i64);
    }

    let offset_incr : i64 = (args.buf_size as i64) * (buf.len() as i64);

    let times = (size+args.buf_size-1)/(buf.len()*buf[0].len());

    let now = Instant::now();
    for _i in 0..times {
        for j in 0..num_bufs {
            unsafe {
                let read_e = opcode::Read::new(types::Fd(read_fd.as_raw_fd()), buf[j].as_mut_ptr(), buf[j].len() as _)
                 .offset(offsets[j])
                 .build()
                 .user_data(j as u64);

                ring.submission().push(&read_e)
                    .expect("Submission Queue Full");  
            }
        }
        ring.submit_and_wait(num_bufs).expect("read submit failed");
        for j in 0..num_bufs {
            offsets[j] += offset_incr;
        }
        if args.amortize_cqe {
            let cqe_len = ring.completion().len(); // .collect::<Vec<_>>();
            if (cqe_len as u32) > RING_SIZE-200 {
                // Need to benchmark if greedy exhaustion is really better?
                ring.completion().for_each(drop);
            }
        } else {
            ring.completion().for_each(drop);
        }
    }
    let elapsed = now.elapsed().as_millis();
    Ok( (size as u64) / (1000 * elapsed as u64) )
}

fn write_async(args: Args) -> io::Result<u64> {
    let mut ring = IoUring::new(RING_SIZE).unwrap();
    let write_fd = fs::File::create(args.out_file)?;
    let num_bufs = args.num_concurrent;
    let mut buf = vec![vec![0; args.buf_size]; num_bufs];
    let mut offsets = vec![0 as i64; num_bufs];

    for i in 0..num_bufs {
        offsets[i] = (i as i64) * (buf[0].len() as i64);
    }
    let size : usize = 1024 * 1024 * 1024 * 3;
    let offset_incr : i64 = (args.buf_size as i64) * (buf.len() as i64);

    let times = (size+args.buf_size-1)/(buf.len()*buf[0].len());

    let now = Instant::now();
    for _i in 0..times {
        for j in 0..num_bufs {
            unsafe {
                let write_e = opcode::Read::new(types::Fd(write_fd.as_raw_fd()), buf[j].as_mut_ptr(), buf[j].len() as _)
                 .offset(offsets[j])
                 .build()
                 .user_data(j as u64);

                ring.submission().push(&write_e)
                    .expect("Submission Queue Full");  
            }
        }
        ring.submit_and_wait(num_bufs).expect("write submit failed");
        for j in 0..num_bufs {
            offsets[j] += offset_incr;
        }
        if args.amortize_cqe {
            let cqe_len = ring.completion().len(); // .collect::<Vec<_>>();
            if (cqe_len as u32) > RING_SIZE-200 {
                // Need to benchmark if greedy exhaustion is really better?
                ring.completion().for_each(drop);
            }
        } else {
            ring.completion().for_each(drop);
        }
    }
    let elapsed = now.elapsed().as_millis();
    Ok(elapsed as u64)
}

fn main() {
    let args = Args::parse();
    let mut t : u64 = 0;
    if args.read {
        t = read_async(args).unwrap();
    } else {
        t = write_async(args).unwrap();
    }
    println!("{}", t);
}