// this program reads all zip files inside the "pak" folder of any sacred2 installation and puts them in a sqlite database file
#![allow(non_snake_case)]
#![allow(warnings)]

/// try to extract the gr2 files.
// todo, extract equivalent inside-path files into a workspace folder
// todo, link to c++ part of granny lib converter
/// dafür brauch ich noch die c++ grannyconverter tests ob man die so überlagern kann wie ich denke
// todo, implement re-load entries from .sqlite
// todo investigate, use filename field on the .sqlite table and detect possible mismatches between full inside path groups and filename groups
// todo investigate "E:\Programs\Steam\steamapps\common\Sacred 2 Gold\pak\graphics04.zip\lq\maps\gui\gui_quest_signs-subquests" has no ext but its clearly a .dds
// todo, wenn ich durch bin kann ich den pfad zur sacred install dir versuchen autom. zu suchen
// todo, add file size info to sqlite db

mod sacredTools;

use std::error::Error;
use std::io::stdin;
use std::path::PathBuf;
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

        // LoadAllIntoNewDbFile(&allOfThem);
        let queryString = String::from("models/npc/highelves/inquisitor-w-soldier/v_inq-w-soldier_idlea_zw.GR2");
        let mut temp = QueryForPath(&mut allOfThem, &*queryString);
        let extractPath = PathBuf::from("C:\\Users\\ruben\\Desktop\\sacred extract test");
        ExtractTo(&mut temp, extractPath.clone());
        let list = temp.iter().map(|x| x.meta_temporaryExtractFilepath.clone().expect("previous code ensures this is set").into_os_string().into_string().unwrap()).collect();
        let result = ffi::extractFbx(&extractPath.into_os_string().into_string().unwrap(), list);
    }
}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        // include!("main.h");
        fn extractFbx(baseFilepath: &str, list: Vec<String>) -> usize;
    }
}
// extern "C" {
//     fn extractFbx(baseFilepath: &cty::c_char, list: &Vec<String>) -> usize;
// }

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
