#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Try to parse the data as a string
    if let Ok(content) = std::str::from_utf8(data) {
        // Create a temporary file with the content
        use std::io::Write;
        if let Ok(mut temp_file) = tempfile::NamedTempFile::new() {
            if temp_file.write_all(content.as_bytes()).is_ok() {
                // Try to parse the file - we don't care if it fails,
                // we just want to make sure it doesn't panic
                let _ = ferriprove_export::parse_file(temp_file.path());
            }
        }
    }
});
