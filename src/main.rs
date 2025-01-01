use std::collections::HashMap;

fn main(){
    let mut users = HashMap::new();
    
    users.insert(String::from("armaan") , 21);
    users.insert(String::from("ormon"), 3 );

    let age_of_user = users.get("ormon");

    match age_of_user{
        Some(number) => println!("age of user: {}", number),
        None => println!("User not found")
    }

}