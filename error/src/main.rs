use std::fs::File;


fn main() {
    //
    // let vector = vec!["1","2"];
    // // vector[2];
    //
    // let f = File::open("abc.jpg");
    // println!("open file result {:?}",f);

    let result = is_even(6).unwrap();
    println!("result value {}",result);

    let result2 = is_even(61).unwrap();
    println!("result2 value {}",result2);
    println!("Hello, world!");
}

fn is_even(no:i32)->Result<bool,String>{
    return if no%2==0{
        Ok(true)
    }else{
        Err("input error".to_string())
    }
}
