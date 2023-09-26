use sqlite::*;

pub mod sacred
{
    use crate::sacredZipFolder::sacred::SacredZipFile;

    pub fn LoadAllIntoNewDbFile(entriesToInsert: &Vec<SacredZipFile>) {
        std::fs::remove_file("temporary.sqlite").unwrap();
        let con = sqlite::open("temporary.sqlite").unwrap();
        let query = "CREATE TABLE IF NOT EXISTS entries (zipPath TEXT, zipInsidePath TEXT, zipInsideFilename TEXT, fileType INTEGER, name TEXT, comment TEXT)";
        con.execute(query).unwrap();

        let mut batch: Vec<String> = vec![];
        for (i, entry) in entriesToInsert.iter().enumerate() {
            let insertCommand = format!("INSERT INTO entries (zipPath, zipInsidePath, zipInsideFilename, fileType, name, comment) VALUES ({zipPath}, {zipInsidePath}, {zipInsideFilename}, {fileType}, {name}, {comment});",
                                        zipPath = sqliteTextPara(&entry.zipPath),
                                        zipInsidePath = sqliteTextPara(&entry.zipInsidePath),
                                        zipInsideFilename = sqliteTextPara(&entry.zipInsideFilename),
                                        fileType = entry.fileType,
                                        name = sqliteTextPara(&entry.name),
                                        comment = sqliteTextPara(&entry.comment)
            );
            batch.push(insertCommand);
        }
        let fullCommand = format!("BEGIN TRANSACTION;\r\n{}\r\nCOMMIT;", batch.join("\r\n"));
        con.execute(fullCommand).unwrap();
    }

    fn sqliteTextPara(text: &String) -> String { //&[u8] {
        let mut sqliteText = String::new();
        sqliteText.push_str("\'");
        sqliteText.push_str(text);
        sqliteText.push_str("\'");
        return sqliteText;
    }
}