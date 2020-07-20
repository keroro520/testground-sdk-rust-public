use crate::runtime::runenv::RunEnv;
use std::fs::{File, OpenOptions};
use std::path::Path;

impl RunEnv {
    // create_raw_asset creates an output asset.
    //
    // Output assets will be saved when the test terminates and available for
    // further investigation. You can also manually create output assets/directories
    // under re.TestOutputsPath.
    pub fn create_raw_asset(&self, filename: &str) -> Result<File, String> {
        let filepath = Path::new(&self.test_outputs_path).join(filename);
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&filepath)
            .map_err(|err| err.to_string())
    }
}
