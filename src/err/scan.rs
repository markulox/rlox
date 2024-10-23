use super::Err;
pub enum ScanErr {

}

impl Err for ScanErr {
    fn report_str(&self) -> (usize, String) {
        todo!()
    }
}