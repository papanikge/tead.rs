#[crate_id(name="tead", version="1.0.0", author="papanikge")];
//
// George 'papanikge' Papanikolaou 2014
// head + tail = tead
//

extern mod extra;
use extra::getopts::{optopt, optflag, getopts};
use std::io::buffered::BufferedReader;
use std::io::fs::File;
use std::path::Path;

fn usage() {
    println("Usage: tead [options]");
    println("\t-n <number of lines to print>");
    println("\t-h --help");
    println("\t-V --version");
}

// Ideally we would use DoubleEndedIterator but that doesn't work with files,
// since we don't have access to the whole file. So, obviously this is not
// optimized, since we are loading the entire file on memory.
fn tead<T: Reader>(reader: &mut BufferedReader<T>, count: uint) {
    let all: ~[~str] = reader.lines().collect();
    // head
    for line in all.iter().take(count) {
        print(*line);
    }
    // tail
    for line in all.rev_iter().take(count) {
        print(*line);
    }
}

fn main() {
    let args = std::os::args();

    let available = [
        optopt("n"),  // Number of lines to print
        optflag("h"), optflag("help"),
        optflag("V"), optflag("version")
    ];

    // test for the available options upon the provided ones
    let given = match getopts(args.tail(), available) {
        Ok (m) =>  m,
        Err(_) => {
            usage();
            fail!();
        }
    };

    // help and version outta the way
    if given.opt_present("h") || given.opt_present("help") {
        usage();
        return;
    }
    if given.opt_present("V") || given.opt_present("version") {
        println("tead -- version 1.0.0");
        return;
    }

    // let's find out the lines requested and move on
    // Beware: Rust weirdness ahead!
    let lines = match given.opt_str("n") {
        Some(s) => {
            match from_str::<uint>(s) {
                Some(x) => { x }
                None    => { fail!("a string for number of lines, dude?") }
            }
        }
        None    => { 10u }
    };

    // let's go
    let files = given.free;
    if files.is_empty() {
        // No files provided. stdin() is a reader so we can do:
        let mut buffer = BufferedReader::new(std::io::stdin());
        tead(&mut buffer, lines);
    } else {
        for file in files.iter() {
            // the argument below needs to be a ** so we use 'as_slice'
            let path = Path::new(file.as_slice());
            // to avoid all that 'match Some' malarkey we just use 'unwrap'
            let reader = File::open(&path).unwrap();
            // opening the reader
            let mut buffer = BufferedReader::new(reader);
            tead(&mut buffer, lines);
        }
    }
}
