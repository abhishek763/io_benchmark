use std::fs::File;
use std::io::{self, prelude::*, BufReader};
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

    #[clap(short, long)]
    read: bool,

    #[clap(short, long)]
    write: bool
}


fn read(args: &Args) -> io::Result<u64> {
    let inp_file = File::open(&args.inp_file)?;
    let mut size : u64 = 0;
    let buf_size: usize = args.buf_size;
    let mut reader = BufReader::with_capacity(buf_size, inp_file);
    let now = Instant::now();
    loop {
        let length = {
            let buffer = reader.fill_buf()?;
            buffer.len()
        };
        if length == 0 {
            break;
        }
        reader.consume(length);
        size += length as u64;
    }

    let elapsed = now.elapsed().as_millis();
    
    Ok( size / (1000 * elapsed as u64) )
}

fn write(args: &Args) -> io::Result<u64> {
    let mut out_file = File::create(&args.out_file)?;
    let s = args.out_file[..1].parse::<usize>().unwrap();
    let size: usize = 1024 * 1024 * 1024 * s;
    let buf = vec![87; args.buf_size];
    let now = Instant::now();
    for _ in 0..(size + args.buf_size - 1)/args.buf_size {
            out_file.write_all(&buf[..])?;
    };

    let elapsed = now.elapsed().as_millis();
    
    Ok( (size as u64) / (1000 * elapsed as u64))
}

fn main() {
    let args = Args::parse();
    let mut t : u64 = 0;
    if args.read {
        t = read(&args).unwrap();
    } else {
        t = write(&args).unwrap();
    }
    print!("{}", t);
}