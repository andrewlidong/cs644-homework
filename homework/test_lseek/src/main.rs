use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write, Read};
use std::os::unix::fs::OpenOptionsExt;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let filename = "testfile.txt";
    
    // Open file with read & write permissions
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;

    // Step 1: Write data to the file
    let data = b"Hello, world!";
    file.write_all(data)?;
    
    // Step 2: Seek back to the beginning
    file.seek(SeekFrom::Start(0))?;

    // Step 3: Read back the data
    let mut buffer = vec![0; data.len()];
    file.read_exact(&mut buffer)?;

    // Step 4: Print results
    println!("Expected: {:?}", std::str::from_utf8(data).unwrap());
    println!("Read back: {:?}", std::str::from_utf8(&buffer).unwrap());

    // Step 5: Check if data matches
    if data == &buffer[..] {
        println!("✅ Data matches (buffered read successful)");
    } else {
        println!("❌ Data mismatch (write buffering issue)");
    }

    // Clean up
    drop(file);
    std::fs::remove_file(filename)?;

    Ok(())
}
