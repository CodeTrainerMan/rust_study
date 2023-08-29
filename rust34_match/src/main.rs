fn main() {
    let num = &100;

    match num {
        &val=> println!("&val is {:?}",val)
    }

    match *num {
        val => println!(" val is {:?}",val)
    }

    let ref _num3=66;
    let num4 = 5;
    let mut mut_num = 7;
     match num4 {
             r=>println!("num4 is {:?}",r)

    }

    match mut_num {
        ref mut m=>{
            *m+=10;

            println!("mut_num4 is {:?}",m)
        }
    }
    let s = study{
        name:String::from("mutch study"),
        target:String::from("deepleaning study match"),
        spend:1,
    };

    let study{
        name,
        target,
        spend
    } = s;
    println!("name is {}, target is {}, spend is {}",name,target,spend);


     let s2 = study{
        name:String::from("mutch study"),
        target:String::from("deepleaning study match"),
        spend:1,
    };
    let study{name,..} = s2;
    println!("point test {}",name);
    println!("Hello, world!");
}

struct study{
    name:String,
    target:String,
    spend:u32,
}
