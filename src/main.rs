#[macro_use]
extern crate clap;
extern crate chrono;

use clap::{Arg, App};
use chrono::Local;
use data_encoding::HEXUPPER;

pub mod digest;

fn print_header() {

    let local_time = Local::now();

    println!("; Generated by {} v{} on {}",
             crate_name!(),
             crate_version!(),
             local_time.format("%Y-%m-%d at %H:%M:%S"));
}

fn main() {

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("files")
             .multiple(true)
             .required(true))
        .get_matches();

    let iterator = matches.values_of("files");

    print_header();


    for el in iterator.unwrap() {
        let filename = el.to_string();
        print!("{} ", filename);
        let checksum = digest::calculate_digest_mmap(filename, digest::Digest::CRC32);
        println!("{}", HEXUPPER.encode(&checksum.unwrap().to_be_bytes()));
    }
}
