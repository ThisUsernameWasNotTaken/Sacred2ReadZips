#![allow(non_snake_case)]

use std::{fs, io};
use std::error::Error;
use std::io::stdin;
use zip;

fn main() {
    loop {
        let mut input_path = String::new();
        println!("Paste Filepath to zip");
        let resReadLine = stdin().read_line(&mut input_path);
        println!("{resReadLine:?}");
        if resReadLine.is_err() {
            continue;
        }

        let resReadZip = read_all_filepaths_from_zip(&input_path).unwrap();
        //println!("{resReadZip:?}");
        let files = resReadZip.iter().map(|x| { x.clone().zipInsidePath.clone() });
        let x: Vec<String> = files.collect();
        let y = x.join("\r\n");
        println!("{}", y);
    }
}

// #[derive(Debug)]
// enum TryFileError {
//     UnknownError(Box<str>)
//     // ErrorStdReadLine,
//     // ErrorOpenFile,
// }

fn read_all_filepaths_from_zip(input_path: &String) -> Result<Vec<SacredZipFile>, Box<dyn Error>> {
    let filepath = std::path::PathBuf::from(&*input_path.trim());
    let file_res = std::fs::File::open(filepath.clone());
    if file_res.is_err() {
        println!("{filepath:?}")
    }
    let file = file_res?;

    let mut sacredFiles: Vec<SacredZipFile> = Vec::new();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let SacredZipFile { zipPath, zipInsidePath, endsWithSlash, name, comment }: SacredZipFile;

        let mut archiveZipFile = archive.by_index(i).unwrap();
        let archiveZipFilePath = match archiveZipFile.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        zipPath = filepath.clone().into_os_string().into_string().unwrap();
        zipInsidePath = archiveZipFilePath.into_os_string().into_string().unwrap();
        endsWithSlash = (*archiveZipFile.name()).ends_with('/');
        name = archiveZipFile.name().to_string();
        comment = archiveZipFile.comment().to_string();

        sacredFiles.push(SacredZipFile { zipPath, zipInsidePath, endsWithSlash, name, comment });
    }
    return Ok(sacredFiles);
}

#[derive(Debug)]
struct SacredZipFile {
    zipPath: String,
    zipInsidePath: String,
    endsWithSlash: bool,
    name: String,
    comment: String,
}