use super::Err;

enum RunFileError {
    NotFound { f_name: String },
    Unknown { code: u8 },
}

impl Err for RunFileError {
    fn report_str(&self) -> (usize, String) {
        match self {
            RunFileError::NotFound { f_name } => (0, format!("File {f_name} not found")),
            RunFileError::Unknown { code } => (0, format!("Unknown error : {code}")),
        }
    }
}
