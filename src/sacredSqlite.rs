use sqlite::*;

pub mod sacred
{
    use crate::sacredZipFolder::sacred::SacredZipFile;


    pub fn LoadAllIntoNewDbFile(entriesToInsert: &Vec<SacredZipFile>) {
        /*
        pub struct SacredZipFile {
            zipPath: String,
            zipInsidePath: String,
            endsWithSlash: bool,
            name: String,
            comment: String,
        }
        */
        let con = sqlite::open("temporary.sqlite").unwrap();
        let query = "CREATE TABLE IF NOT EXISTS entries (zipPath TEXT, zipInsidePath TEXT, fileType INTEGER, name TEXT, comment TEXT)";
        con.execute(query).unwrap();

        let insertCommand = "INSERT INTO entries VALUES (:zipPath, :zipInsidePath, :endsWithSlash, :name, :comment)";
        let mut insertStatement = con.prepare(insertCommand).unwrap().into_iter();
        for (i, entry) in entriesToInsert.iter().enumerate() {
            con.("")
            // .bind((":zipPath", sqliteTextPara(&entry.zipPath)))
            // .bind((":zipInsidePath", sqliteTextPara(&entry.zipInsidePath)))
            // //.bind((":fileType", entry.fileType))
            // .bind((":name", sqliteTextPara(&entry.name)))
            // .bind((":comment", sqliteTextPara(&entry.comment)));

            insertStatement.bind((":zipPath", sqliteTextPara(&entry.zipPath))).unwrap();
            insertStatement.bind((":zipInsidePath", sqliteTextPara(&entry.zipInsidePath))).unwrap();
            // insertStatement.bind((":fileType", entry.fileType)).unwrap();
            insertStatement.bind((":name", sqliteTextPara(&entry.name))).unwrap();
            insertStatement.bind((":comment", sqliteTextPara(&entry.comment))).unwrap();
            // let temp = con.execute(insertStatement);
            // if temp.is_err() {
            //     println!("{i}");
            // }
            insertStatement.map(|row| row.unwrap());
        }
    }

    fn sqliteTextPara(text: &String) -> &[u8] {
        return text.as_bytes();
    }
}