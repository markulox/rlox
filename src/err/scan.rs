use super::{report::error, ErrReport};
pub enum ScanErr {
    UnknownChar(usize, char),
    UnterminatedString(usize),
    InvalidNumber(usize, String)
}

impl ErrReport for ScanErr {
    // 
    fn report(&self) {
        match self {
            ScanErr::UnknownChar(line,c) => {
                error(*line, format!("Unexpected character: {c}").as_str());
            },
            ScanErr::UnterminatedString(line) => {
                error(*line, "Unterminated string");
            },
            ScanErr::InvalidNumber(line, invalid_num) => {
                error(*line, format!("Invalid number format: {invalid_num}").as_str());
            }
        }
    }
}