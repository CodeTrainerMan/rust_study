use std::fs::OpenOptions;
use std::io::{Write, Read};

fn main() {

    // let file = std::fs::File::open("data.txt");
    // println!("open file is {:?}",file);
    // let mkfile = std::fs::File::create("data2.txt").expect("create file fail");
    // println!("create file is {:?}",mkfile);
    // std::fs::remove_file("data.txt");
    // println!("Hello, world!");


    // let mut input_data = OpenOptions::new().append(true).open("data2.txt")
    //     .expect("input  file data fail");
    // input_data.write("file input data".as_bytes()).expect("input fail");
    // println!("data input success {:?}",input_data);
    // input_data.write_all("   Rust study".as_bytes()).expect("input fail");
    // input_data.write_all("   \nRust study".as_bytes()).expect("input fail");

    let mut file = std::fs::File::open("data2.txt").expect("open file fail");
    let mut context = String::new();
    file.read_to_string(&mut context).unwrap();
    println!("read data is {}", context)

}
