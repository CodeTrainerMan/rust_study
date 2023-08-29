use std::io::Write;

fn main() {
    // let mut in_word = String::new();
    // let result = std::io::stdin().read_line(&mut in_word).unwrap();
    // println!("input data is {}",in_word);

    let result2 = std::io::stdout().write("stdout study".as_bytes()).unwrap();
    // println!("stdout data is {}\n",result2);


    let input_args = std::env::args();
    for arg in input_args {
        println!("terminal input args cycle print is :{}",arg);
    }
    println!("Hello, world!");
}
