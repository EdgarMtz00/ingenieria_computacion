use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::io::Write;

thread_local! (static TEMP_FILE: RefCell<String> = RefCell::new(String::new()));
thread_local! (static OUTPUT_FILE: RefCell<String> = RefCell::new(String::new()));

pub fn initialize_files(source_file: &str) {
    if !source_file.contains(".op") {
        panic!("The file must be a .op file");
    }

    let mut contents = String::new();
    let mut f = File::open(source_file).unwrap();
    f.read_to_string(&mut contents).unwrap();

    TEMP_FILE.with(|file_name| {
        *file_name.borrow_mut() = source_file.replace(".op", ".tmp");
    });

    OUTPUT_FILE.with(|file_name| {
        *file_name.borrow_mut() = source_file.replace(".op", ".out");
    });

    TEMP_FILE.with(|file_name| {
        let mut f = File::create(file_name.borrow().to_string()).unwrap();
        write!(f, "{}", contents).unwrap();
    });
}

pub fn read_temp_file() -> String {
    let mut contents = String::new();

    TEMP_FILE.with(|file_name| {
        let mut file = File::open(file_name.borrow().to_string()).unwrap();
        file.read_to_string(&mut contents).unwrap();
    });

    contents
}

pub fn write_temp_file(contents: &str) {
    TEMP_FILE.with(|file_name| {
        let mut file = File::create(file_name.borrow().to_string()).unwrap();
        write!(file, "{}", contents).unwrap();
    });
}

pub fn write_output_file(contents: &str) {
    OUTPUT_FILE.with(|file_name| {
        let mut file = File::create(file_name.borrow().to_string()).unwrap();
        write!(file, "{}", contents).unwrap();
    });
}