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


fn copy_buf(inp_file_name: &str, out_file_name: &str) -> io::Result<()> {
    let inp_file = File::open(inp_file_name)?;
    let out_file = File::create(out_file_name)?;
    let mut reader = BufReader::new(inp_file);
    let mut writer = io::BufWriter::new(out_file);
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        writer.write_all(line.as_bytes())?;
        line.clear();
    }
    Ok(())
}

fn copy_unbuf(inp_file_name : &str, out_file_name: &str) -> io::Result<()> {
    let mut inp_file = File::open(inp_file_name)?;
    let mut out_file = File::create(out_file_name)?;
    let mut input_buffer = String::new();
    inp_file.read_to_string(&mut input_buffer)?;

    out_file.write_all(input_buffer.as_bytes())?;

    Ok(())
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
    let size: usize = 1024 * 1024 * 1024 * 3;
    let buf: [u8; 4096] = [87; 4096];
    let now = Instant::now();
    for _ in 0..(size + args.buf_size - 1)/args.buf_size {
            out_file.write_all(&buf[..])?;
    };


    let elapsed = now.elapsed().as_millis();
    
    Ok(elapsed as u64)
}

fn main() {
    let args = Args::parse();
    let mut t : u64 = 0;
    if args.read {
        t = read(&args).unwrap();
    } else {
        t = write(&args).unwrap();
    }
    println!("{}", t);
}