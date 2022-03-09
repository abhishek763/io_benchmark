use std::fs;
use std::fs::{File};
use std::io::{self, prelude::*, SeekFrom};
use std::time::{Instant};
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

    #[clap(short, long)]
    read: bool,

    #[clap(short, long)]
    write: bool,

    #[clap(short, long, default_value_t = 3)]
    test_duration: u64,

    #[clap(long)]
    random: bool
}


fn read(args: &Args) -> io::Result<(u64, u64)> {
    let mut inp_file = File::open(&args.inp_file)?;
    let buf_size: usize = args.buf_size;
    let mut buf = vec![0; buf_size];
    let file_size : i64 = fs::metadata(&args.inp_file)?.len() as i64;

    let mut read_size = 0;
    let now = Instant::now();
    while now.elapsed().as_secs() < args.test_duration {
        let length = io::Read::by_ref(&mut inp_file).take(buf_size as u64).read_to_end(&mut buf)?;
        if length == 0 {
           inp_file.seek(SeekFrom::Start(0))?;
        }
        read_size += length as u64;

        if args.random {
            rand::thread_rng().gen_range(0, file_size);
        }
    }

    let elapsed = now.elapsed().as_millis();

    Ok((read_size, (elapsed as u64)))
}

fn write(args: &Args) -> io::Result<(u64, u64)> {
    let mut out_file = File::create(&args.out_file)?;
    let buf = vec![80; args.buf_size];

    let mut write_size = 0;
    let now = Instant::now();
    while now.elapsed().as_secs() < args.test_duration {
        let length = io::Write::by_ref(&mut out_file).write(&buf)?;
        write_size += length as u64;
        out_file.sync_all()?;
    }
    let elapsed = now.elapsed().as_millis();
    
    Ok((write_size, elapsed as u64))
}

fn main() {
    let args = Args::parse();
    if args.read {
        let (bytes, ms) = read(&args).unwrap();
        print!("read,{:.4}MB,{:.2}MB/s", (bytes as f64) / 1000000.0, (bytes as f64) / (1000.0 * ms as f64));
    } else {
        let (bytes, ms) = write(&args).unwrap();
        print!("write,{:.4}MB,{:.2}MB/s", (bytes as f64) / 1000000.0, (bytes as f64) / (1000.0 * ms as f64));
    }
}