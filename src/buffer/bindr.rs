use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Debug)]
pub enum BindErr {
    FileCreate,
    FileOpen,
    FileSave,
    FileLoad,
}

impl Error for BindErr {}
impl Display for BindErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self);
    }
}

pub trait Bindr {
    fn save(&self, buf: &String) -> Result<(), BindErr>;
    fn load(&self, but: &mut Vec<String>) -> Result<(), BindErr>;
}

pub struct DefaultBind;

impl Bindr for DefaultBind {
    fn save(&self, _: &String) -> Result<(), BindErr> {
        return Ok(());
    }

    fn load(&self, _: &mut Vec<String>) -> Result<(), BindErr> {
        return Ok(());
    }
}

pub struct FileBind {
    pub file: String,
}

impl Bindr for FileBind {
    fn save(&self, buf: &String) -> Result<(), BindErr> {
        let mut file = match File::create(self.file.to_string()) {
            Ok(x) => x,
            Err(_) => return Err(BindErr::FileCreate),
        };

        return match file.write_all(buf.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(BindErr::FileSave),
        };
    }

    fn load(&self, buf: &mut Vec<String>) -> Result<(), BindErr> {
        let mut file = match File::open(self.file.to_string()) {
            Ok(x) => x,
            Err(_) => return Err(BindErr::FileOpen),
        };

        let mut text = String::new();
        match file.read_to_string(&mut text) {
            Ok(len) => match len {
                0 => return Err(BindErr::FileLoad),
                _ => {
                    *buf = text.split_inclusive('\n').map(|x| x.to_string()).collect();
                    return Ok(());
                }
            },
            Err(_) => return Err(BindErr::FileLoad),
        };
    }
}
