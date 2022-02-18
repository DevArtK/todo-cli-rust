use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("Hello.txt");
    let result: File = open_file(path);
    println!("{:?}", result);
}

fn path_to_string(input_path: String) -> u8 {
    Path::new(&input_path)


}

fn open_file(path: &Path) -> File {
    let display = path.display();
    let mut file = match File::open(path) {
        Err(e) => panic!("Couldn't open {} : {}", display, e),
        Ok(file) => file,
    };

    let mut result = String::new();

    match file.read_to_string(&mut result) {
        Err(e) => panic!("Couldn't Read {} : {}", display, e),
        Ok(_) => println!("{} contains:\n{}", display, result)
    };

    return file;
}

fn read_file(file: &File) {
    let path = Path::new("hello.txt");
    let display = path.display();
}
