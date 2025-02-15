use std::fs;
use std::io;
use std::io::Read;

/// Read file and return contents if possible.
pub fn read_source(filename: &String) -> Result<String, Box<dyn std::error::Error>> {
    // Handle errors accessing file on filesystem
    let mut source = match fs::File::open(filename) {
        Ok(source) => source,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                return Err(format!("ERROR: File '{}' not found.", filename).into());
            }
            io::ErrorKind::PermissionDenied => {
                return Err(format!("ERROR: No permission to read file '{}'.", filename).into());
            }
            _ => {
                return Err(format!("ERROR: Unable to open file '{}': {}", filename, error).into());
            }
        },
    };

    // Handle errors when reading file
    let mut contents = String::new();
    if let Err(error) = source.read_to_string(&mut contents) {
        match error.kind() {
            io::ErrorKind::InvalidData => {
                return Err(
                    format!("ERROR: File '{}' is not a valid UTF-8 text file.", filename).into(),
                );
            }
            _ => {
                return Err(format!("ERROR: Failed to read file '{}': {}", filename, error).into());
            }
        }
    }

    // Debug output
    dbg!(&contents);

    Ok(contents)
}
