use crate::File;
use std::fs::*;
use std::io::*;
pub struct vfFile {
    path: String,
    data: String,
}

impl File for vfFile {
    fn read(&self) -> Result<String, std::io::Error> {
        let mut file = StdFile::open(self.path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn write(&self) -> Result<(), std::io::Error> {
        let mut file = StdFile::create(self.path)?;
        file.write_all(self.data.as_bytes())?;
        Ok(())
    }
}