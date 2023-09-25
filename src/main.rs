#![allow(non_snake_case)]

/// read aaaaalll ze zippsss
/// und then... try to extract the gr2 files.
/// dafür brauch ich noch die c++ grannyconverter tests ob man die so überlagern kann wie ich denke
/// wenn ich durch bin kann ich den pfad zur sacred install dir versuchen autom. zu suchen

mod sacredFolderFunctions;

use std::{fs, io};
use std::error::Error;
use std::io::stdin;
use zip;
use sacredFolderFunctions::sacred::*;

fn main() {
    loop {
        // get zip locations
        let defaultFolderPath = String::from("E:\\Programs\\Steam\\steamapps\\common\\Sacred 2 Gold\\pak");
        let mut userInputText = String::new();
        println!("Paste folder to sacred 2 zips (empty is {})", defaultFolderPath);
        let resReadLine = stdin().read_line(&mut userInputText);
        println!("{resReadLine:?}");
        if resReadLine.is_err() {
            continue;
        }
        if userInputText.trim().eq("") {
            userInputText = defaultFolderPath;
        }
        userInputText = userInputText.trim().to_string();

        // get list of actual zip files. then read all and index them
        let allZipsInDirectory = listAllZipPaths(&userInputText);
        for zipFilePath in allZipsInDirectory {
            let resReadZip = readZip(&zipFilePath).unwrap();
            //println!("{resReadZip:?}");
            let files = listAllInsidePaths(resReadZip).join("\r\n");
            println!("{}", files);
        }
    }
}

