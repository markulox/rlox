pub mod file;
pub mod report;
pub mod scan;

pub trait Err{
    fn report_str(&self) -> (usize, String);
}