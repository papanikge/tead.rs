//
// George 'papanikge' Papanikolaou 2014-2018
// tail + head = tead
//
// This was originally written in an ancient form of rust.
// Rewritten for rust 1.22.
//

extern crate getopts;
use getopts::Options; // use extra::getopts::{optopt, optflag, getopts};
// renamed to BufReader. but probaby do not want it // use std::io::buffered::BufferedReader;
use std::env;

static VERSION: &'static str = "1.1.0";
static DEFAULT_LINE_NUMBER: &'static str = "5";

fn usage(program_name: &str) {
    println!("Usage: {} [options]", program_name);
    println!("\t-n <number of lines to print>");
    println!("\t-h --help");
    println!("\t-v --version");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    let mut available = Options::new();
    available.optopt("n", "lines-number", "number of lines to print [default: 5]", "");
    available.optflag("v", "version", "print tead's version");
    available.optflag("h", "help", "print this help menu");

    let given = match available.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if given.opt_present("h") {
        usage(&program_name);
        return;
    }
    if given.opt_present("v") {
        println!("tead -- Version {}", VERSION);
        return;
    }

    let lines = given.opt_str("n").or(Some(String::from(DEFAULT_LINE_NUMBER))).unwrap();

    let files = given.free;
    if files.is_empty() {
        usage(&program_name);
        return;
        // No files provided. stdin() is a reader so we can do:
        // let mut buffer = BufferedReader::new(std::io::stdin());
        // call tead here
    } else {
        println!("temp - lines: {}", lines);
        files[0].clone();
        // call tead here
    }
}
