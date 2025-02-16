use std::fs;
use std::io;
use std::io::Read;

/// Read file and return contents if possible.
pub fn read_source(filename: &String) -> Result<String, Box<dyn std::error::Error>> {
    // We use `match` here to handle both successful and error cases.
    let mut source = match fs::File::open(filename) {
        Ok(source) => source,
        // Handle errors accessing file on filesystem
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
    // We use `if let` here to focus on just the error cases that can result from reading the file into contents.
    if let Err(error) = source.read_to_string(&mut contents) {
        // Ff no issues with don't enter the below, move to returning contents as our Ok value.
        if error.kind() == io::ErrorKind::InvalidData {
            return Err(
                format!("ERROR: File '{}' is not a valid UTF-8 text file.", filename).into(),
            );
        } else {
            return Err(format!("ERROR: Failed to read file '{}': {}", filename, error).into());
        }
    }
    // Debug output
    dbg!(&contents);

    // Send the contents on
    Ok(contents)
}
