#![allow(non_snake_case)]

use zip;

pub mod sacred {
    pub fn listAllInsidePaths(sacredFiles: &Vec<SacredZipFile>) -> Vec<String>
    {
        return sacredFiles.iter().map(|x| { x.clone().zipInsidePath.clone() }).collect::<Vec<String>>();
    }

    pub fn readZip(filepathSacredZip: &String) -> Result<Vec<SacredZipFile>, Box<dyn std::error::Error>> {
        let filepath = std::path::PathBuf::from(&*filepathSacredZip.trim());
        let file_res = std::fs::File::open(filepath.clone());
        if file_res.is_err() {
            println!("{filepath:?}")
        }
        let file = file_res?;

        let mut sacredFiles: Vec<SacredZipFile> = Vec::new();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        for i in 0..archive.len() {
            let SacredZipFile { zipPath, zipInsidePath, fileType, name, comment }: SacredZipFile;

            let mut archiveZipFile = archive.by_index(i).unwrap();
            let archiveZipFilePath = match archiveZipFile.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            zipPath = filepath.clone().into_os_string().into_string().unwrap();
            zipInsidePath = archiveZipFilePath.into_os_string().into_string().unwrap();
            name = archiveZipFile.name().to_string();
            comment = archiveZipFile.comment().to_string();
            let endsWithSlash = (*archiveZipFile.name()).ends_with('/');
            if endsWithSlash {
                fileType = 1;
            } else {
                fileType = 0;
            }

            sacredFiles.push(SacredZipFile { zipPath, zipInsidePath, fileType, name, comment });
        }
        return Ok(sacredFiles);
    }

    pub fn listAllZipPaths(sacredFolderpathToZips: &String) -> Vec<String>
    {
        let allFilepaths: Vec<String> = std::fs::read_dir(sacredFolderpathToZips).unwrap().map(|x| x.unwrap().path().into_os_string().into_string().unwrap()).collect();
        return allFilepaths;
    }

    #[derive(Debug)]
    pub struct SacredZipFile {
        pub zipPath: String,
        pub zipInsidePath: String,
        pub fileType: i64,
        pub name: String,
        pub comment: String,
    }

    impl SacredZipFile
    {
        pub fn isDirectory(&mut self) -> bool {
            return self.fileType == 1;
        }
    }
}