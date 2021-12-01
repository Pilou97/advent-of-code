pub mod puzzles;

pub mod utils {
    use std::{fs::File, io::Read, path::Path};

    pub fn read_string(path: &str) -> String {
        let path = Path::new(path);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut raw_data = String::new();

        match file.read_to_string(&mut raw_data) {
            Err(err) => panic!("couldn't read {}: {}", display, err),
            Ok(_) => raw_data,
        }
    }
}

pub mod prelude {
    pub use crate::puzzles::day1::day1;
    pub use crate::utils;
}

// 1532 1571
