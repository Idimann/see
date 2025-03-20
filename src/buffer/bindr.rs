#[derive(Debug)]
pub enum BindErr {}

impl std::error::Error for BindErr {}

impl std::fmt::Display for BindErr {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub trait Bindr {
    fn save(&self) -> Result<(), BindErr>;
    fn load(&mut self) -> Result<(), BindErr>;
}

pub struct DefaultBind;

impl Bindr for DefaultBind {
    fn save(&self) -> Result<(), BindErr> {
        return Ok(());
    }

    fn load(&mut self) -> Result<(), BindErr> {
        return Ok(());
    }
}

pub struct FileBind {
    pub file: String,

}

impl Bindr for FileBind {
    fn save(&self) -> Result<(), BindErr> {
        todo!()
    }

    fn load(&mut self) -> Result<(), BindErr> {
        todo!()
    }
}
