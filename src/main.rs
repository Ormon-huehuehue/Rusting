use std::collections::HashMap;

fn group_values_by_key(vec : Vec<(String, i32)>)-> HashMap<String, i32>{
    let mut hm = HashMap::new();

    for (key, value) in vec{
        hm.insert(key, value);
    }

    return hm;

}

fn main(){
   let pairs = vec![
    (String::from("Armaan"),1),
    (String::from("Ormon"), 2)
   ];

   println!("hashmap obtained :-");

   for (key, value) in group_values_by_key(pairs){
    println!("{} : {:?}", key, value);
   }

}