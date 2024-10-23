use super::{report::error, ErrReport};

enum RunFileError {
    NotFound { f_name: String },
    Unknown { code: u8 },
}

impl ErrReport for RunFileError {
    fn report(&self) {
        match self {
            RunFileError::NotFound { f_name } => error(0, format!("File {f_name} not found").as_str()),
            RunFileError::Unknown { code } => error(0, format!("Unknown error : {code}").as_str()),
        }
    }

    // fn report_values(&self) -> (usize, String) {
    //     match self {
    //         RunFileError::NotFound { f_name } => (0, format!("File {f_name} not found")),
    //         RunFileError::Unknown { code } => (0, format!("Unknown error : {code}")),
    //     }
    // }
}
