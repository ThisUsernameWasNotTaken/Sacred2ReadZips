#![allow(non_snake_case)]

mod sacredFolderFunctions;

use std::{fs, io};
use std::error::Error;
use std::io::stdin;
use zip;
use sacredFolderFunctions::sacred::*;

fn main() {
    loop {
        let mut input_path = String::new();
        println!("Paste Filepath to zip");
        let resReadLine = stdin().read_line(&mut input_path);
        println!("{resReadLine:?}");
        if resReadLine.is_err() {
            continue;
        }

        let resReadZip = readZip(&input_path).unwrap();
        //println!("{resReadZip:?}");
        let files = listAllInsidePaths(resReadZip).join("\r\n");
        println!("{}", files);
    }
}

