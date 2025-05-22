use core::str;
use std::{io::{stdin, Read, Stderr}, process::Stdio};

mod Algorithims;
// Fletcher16 

use Algorithims::Fletcher::*;

fn main() {
    loop { 
        let mut input: [u8; 1024] = [0; 1024];
        let _ = stdin().read(&mut input);

        let input_stripped = str::from_utf8(&input).unwrap().trim_matches(['\n', '\r','\0']).as_bytes();
        let input_len = input_stripped.len();

        println!("Checksum: {:x}", Fletcher_32(input_stripped));
    }
}