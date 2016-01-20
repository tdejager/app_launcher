extern crate getopts;
use getopts::Options;
use std::env;


fn print_usage(program: &str, opts: Options){
    let brief = "Usage launch_apps [Programs..]";
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Argumens given {:?}", args[1:])
}
