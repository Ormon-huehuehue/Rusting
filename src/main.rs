use std::fs;


fn main(){
    let result = fs::read_to_string("hello.txt");

    match result{
        Ok(content)=>{
            println!("File content : {}", content);
        }
        Err(error)=>{
            println!("Error reading file : {}", error);
        }
    }
}