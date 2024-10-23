pub mod file;
pub mod report;
pub mod scan;

pub trait ErrReport{
    fn report(&self);
    // TODO: Probably need a value extractor...
}