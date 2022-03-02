// extern crate argparse;

// use argparse::{ArgumentParser, Store};

// pub struct Args {
//     pub inp_file: String,
//     pub out_file: String,
//     pub buf: bool,
//     pub bufSize: usize,
//     pub numConcurrent: usize,
//     pub sequential: bool,
// }

// impl Default for Args {
//     fn default() -> Self {
//         Args {
//             inp_file: String::from(""),
//             out_file: String::from(""),
//             buf: false,
//             bufSize: 4096,
//             numConcurrent: 8,
//             sequential: false,
//             copy: false,
//         }
//     }
// }

// impl Args {
//     pub fn new() -> Args {
//         let args = Args::default()
//         {
//             let mut ap = ArgumentParser::new();
//             ap.set_description("Async IO!");
//             ap.refer(&mut args.inp_file)
//                 .add_option(&["-i", "--input"], Store, "Input File");
//             ap.refer(&mut args.out_file)
//                 .add_option(&["-f", "--file"], Store, "File to search in");
//             ap.parse_args_or_exit();
//         }
//         args
//     }
// }