use super::{report::error, ErrReport};
pub enum ScanErr {
    UnknownChar(usize, char)
}

impl ErrReport for ScanErr {
    // 
    fn report(&self) {
        match self {
            ScanErr::UnknownChar(line,c) => {
                error(*line, format!("Unknown character: {c}").as_str());
            }
        }
    }
}