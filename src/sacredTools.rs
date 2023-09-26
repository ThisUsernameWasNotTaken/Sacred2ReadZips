#![allow(non_snake_case)]

pub fn LoadAllIntoNewDbFile(entriesToInsert: &Vec<SacredZipFile>) {
    let mut batch: Vec<String> = vec![];
    for (i, entry) in entriesToInsert.iter().enumerate() {
        let insertCommand = format!("INSERT INTO entries (zipPath, zipInsidePath, zipInsideFilename, fileType, name, comment) VALUES ({zipPath}, {zipInsidePath}, {zipInsideFilename}, {zipInsideExtension}, {zipType}, {name}, {comment});",
                                    zipPath = sqliteTextPara(&entry.zipPath),
                                    zipInsidePath = sqliteTextPara(&entry.zipInsidePath),
                                    zipInsideFilename = sqliteTextPara(&entry.zipInsideFilename),
                                    zipInsideExtension = sqliteTextPara(&entry.zipInsideExtension),
                                    zipType = entry.zipType,
                                    name = sqliteTextPara(&entry.name),
                                    comment = sqliteTextPara(&entry.comment)
        );
        batch.push(insertCommand);
    }
    let fullCommand = format!("BEGIN TRANSACTION;\r\n{}\r\nCOMMIT;", batch.join("\r\n"));
    std::fs::remove_file("temporary.sqlite").unwrap();
    let con = sqlite::open("temporary.sqlite").unwrap();
    let query = "CREATE TABLE IF NOT EXISTS entries (zipPath TEXT, zipInsidePath TEXT, zipInsideFilename TEXT, zipType INTEGER, name TEXT, comment TEXT)";
    con.execute(query).unwrap();
    con.execute(fullCommand).unwrap();
}

fn sqliteTextPara(text: &String) -> String { //&[u8] {
    let mut sqliteText = String::new();
    sqliteText.push_str("\'");
    sqliteText.push_str(text);
    sqliteText.push_str("\'");
    return sqliteText;
}

pub fn listAllInsidePaths(sacredFiles: &Vec<SacredZipFile>) -> Vec<String>
{
    return sacredFiles.iter().map(|x| { x.clone().zipInsidePath.clone() }).collect::<Vec<String>>();
}

pub fn readZip(filepathSacredZip: &String) -> Vec<SacredZipFile> {
    let filepath = std::path::PathBuf::from(&*filepathSacredZip.trim());
    let file_res = std::fs::File::open(filepath.clone());
    if file_res.is_err() {
        println!("{filepath:?}")
    }
    let file = file_res.unwrap();

    let mut sacredFiles: Vec<SacredZipFile> = Vec::new();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let SacredZipFile { zipPath, zipInsidePath, zipInsideFilename, zipInsideExtension, zipType, name, comment }: SacredZipFile;

        let mut archiveZipFile = archive.by_index(i).unwrap();
        let archiveZipFilePath = match archiveZipFile.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        zipPath = filepath.clone().into_os_string().into_string().unwrap();
        zipInsidePath = archiveZipFilePath.clone().into_os_string().into_string().unwrap();
        zipInsideFilename = archiveZipFilePath.clone().file_name().unwrap().to_os_string().into_string().unwrap();
        name = archiveZipFile.name().to_string();
        comment = archiveZipFile.comment().to_string();
        let endsWithSlash = (*archiveZipFile.name()).ends_with('/');
        if endsWithSlash {
            zipType = 1;
            zipInsideExtension = String::from("DIRECTORY");
        } else {
            zipType = 0;
            zipInsideExtension = archiveZipFilePath.clone().extension().unwrap().to_os_string().into_string().unwrap();
        }

        sacredFiles.push(SacredZipFile { zipPath, zipInsidePath, zipInsideFilename, zipInsideExtension, zipType, name, comment });
    }
    return sacredFiles;
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
    pub zipInsideFilename: String,
    pub zipInsideExtension: String,
    pub zipType: i64,
    pub name: String,
    pub comment: String,
}

impl SacredZipFile
{
    pub fn isDirectory(&mut self) -> bool {
        return self.zipType == 1;
    }
}