// this program reads all zip files inside the "pak" folder of any sacred2 installation and puts them in a sqlite database file
#![allow(non_snake_case)]

/// try to extract the gr2 files.
// todo, extract equivalent inside-path files into a workspace folder
// todo, link to c++ part of granny lib converter
/// dafür brauch ich noch die c++ grannyconverter tests ob man die so überlagern kann wie ich denke
// todo, re-load entries from .sqlite
// todo investigate, use filename field on the .sqlite table and detect possible mismatches between full inside path groups and filename groups
// todo, wenn ich durch bin kann ich den pfad zur sacred install dir versuchen autom. zu suchen

mod sacredTools;

use std::error::Error;
use std::io::stdin;
use zip;
use sacredTools::*;

fn main() {
    loop {
        // get sacred folder path
        let userInputText = getUserInput();

        // get list of actual zip files. then read all and index them
        let mut allOfThem: Vec<SacredZipFile> = vec![];
        let allZipsInDirectory = listAllZipPaths(&userInputText);
        for zipFilePath in allZipsInDirectory {
            let mut resReadZip = readZip(&zipFilePath);
            allOfThem.append(&mut resReadZip);
        }

        LoadAllIntoNewDbFile(&allOfThem);
        // ExtractToWorkspace(&allOfThem, "");
    }
}

fn getUserInput() -> String {
    let defaultFolderPath = String::from("E:\\Programs\\Steam\\steamapps\\common\\Sacred 2 Gold\\pak");
    let mut userInputText = String::new();
    println!("Paste folder to sacred 2 zips (leave empty for {})", defaultFolderPath);
    let resReadLine = stdin().read_line(&mut userInputText);
    println!("{resReadLine:?}");
    if resReadLine.is_err() {
        panic!("something is wrong with the terminal. stdin().readline() was not successful.");
    }
    if userInputText.trim().eq("") {
        userInputText = defaultFolderPath;
    }
    return userInputText.trim().to_string();
}
