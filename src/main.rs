use std::{fs, io};
use std::error::Error;
use std::io::stdin;
use zip;

fn main() {
    loop {
        let res = try_step();
        println!("{res:?}");
    }
}

// #[derive(Debug)]
// enum TryFileError {
//     ErrorStdReadLine,
//     ErrorOpenFile,
// }

fn try_step() -> Result<(), Box<dyn Error>> {
    let mut input_path = String::new();
    println!("Paste Filepath to zip");
    let _ = stdin().read_line(&mut input_path)?;
    let filepath = std::path::Path::new(&*input_path.trim());
    let file_res = std::fs::File::open(filepath);
    if file_res.is_err() {
        println!("{filepath:?}")
    }
    let file = file_res?;
    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let mut file_object = archive.by_index(i).unwrap();
        let output_path = match file_object.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file_object.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}")
            }
        }
        if (*file_object.name()).ends_with('/') {
            print!("File {} extracted to \" {}\"", i, output_path.display());
            // fs::create_dir_all()
        } else {
            println!("File {} extracted to \"{}\" ({} bytes)", i, output_path.display(), file_object.size());
            if let Some(p) = output_path.parent() {
                if !p.exists() {
                    // fs::create_dir_all(p).unwrap();
                }
            }
            // let mut output_file_object = fs::File::create(&output_path).unwrap();
            // std::io::copy(&mut file_object, &mut output_file_object);
        }
    }
    Ok(())
}
