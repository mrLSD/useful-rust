pub use self::fs_ops::*;
pub mod fs_ops {
    pub fn main() {
        super::file_ops();
    }
}

fn file_ops() {
    use std::fs;
    use std::env;
    use std::io::{BufReader, BufRead};
    use std::error::Error;

    let mut path = env::current_dir().unwrap();
    path.push("data");

    // Is directory exist
    if !fs::metadata(&path).is_ok() {
        match fs::create_dir(&path) {
            Err(err) => panic!("Error to create dir: {}", err.description()),
            _ => (),
        }
    }

    // Clone variable
    let mut file = path.clone();
    file.push("some_file.dat");
    if !fs::metadata(&file).is_ok() {
        match fs::File::create(&file) {
            Ok(_) => println!("File created: {}", file.display()),
            Err(err) => panic!("Can't created file. Err: {}", err)
        }
    }
    let f = match fs::File::open(&file) {
        Err(err) => panic!("Can't created file. Err: {}", err),
        Ok(ok) => ok,
    };
    let mut reader = BufReader::with_capacity(1024, f);
    loop {
        let length = {
            let buffer = reader.fill_buf().expect("Failed reading");
            buffer.len()
        };
        if length == 0 { break }
        reader.consume(length);
    }
    // Remove dir and their data
    let _  = fs::remove_dir_all(&path);
    println!("Directory revomed: {:?}", path.display());
}
