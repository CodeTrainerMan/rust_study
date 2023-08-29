#[cfg(test)]
mod test_module{
    fn module_fn(){
        let result = 4;
        assert_eq!(result,5)

    }
}
pub mod for_object{
    pub fn for_obj(name:String){
        println!("for object progream {}",name)
    }
}
