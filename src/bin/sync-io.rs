use std::fs::File;
use std::io::{self, prelude::*, BufReader};

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

fn main() {
    // copy_unbuf("small", "sync-small.txt").unwrap();
    // copy_unbuf("1gfile", "sync-large.txt").unwrap();

    // copy_buf("small", "sync-small.txt").unwrap();
    copy_buf("1gfile", "sync-large.txt").unwrap();
}