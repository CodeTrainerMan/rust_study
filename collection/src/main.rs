use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
    let mut v = Vec::new();
    v.push("Vector study 1");
    v.push("Vector study 2");
    v.push("Vector study 3");
    println!("{:?} and v length {:?}",v,v.len());

    let mut v2 = vec!["Vector auto create1","for object progream","find element"];
    println!("Vector data is {:?}",v2);

    let x = v2.remove(0);
    println!("remove element vector is {:?}",x);
    println!(" original vector {:?}",v2);

    if v2.contains(&"find element") {
        println!("find out contains element")
    }

    let y = v[0];
    println!("{:?}",y);

    for item in v2 {
        println!("for cycle print v2 data :{:?}",item)
    }

    let mut pricess = HashMap::new();
    pricess.insert("HashMap",1);
    pricess.insert("HashSet",2);
    println!("map data is {:?}, map length is {:?}",pricess,pricess.len());

    match pricess.get(&"HashMap") {
        Some(v)=>{
            println!("find value is :{:?}",v);
        }
        None=>{
            println!("not find element");
        }
    }

    for (k,v) in pricess.iter() {
        println!("k:{:?}，v:{:?}",k,v)
    }

    if pricess.contains_key(&"HashMap") {
        println!("find hash map key")
    }

    let z = pricess.remove(&"HashMap");
    println!("{:?}",z);
    println!("{:?}",pricess);

    let mut studySet = HashSet::new();
    studySet.insert("set study");
    studySet.insert("insert data");

    println!("set data :{:?}",studySet);

    for iteam in studySet.iter() {
        println!("set cycle data :{}",iteam)
    }

    match studySet.get(&"set study") {
        None=>{
            println!("not find element");
        }
        Some(data)=>{
            println!("find data is {}",data)
        }
    }

    if studySet.contains(&"set study") {
        println!("contains function find it")
    }
    studySet.remove(&"set study");
    println!("removed value is {:?}",studySet)

}
