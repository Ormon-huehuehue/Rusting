fn main(){
    let index = find_first_a(String::from("jella"));

    match index{
        Some(i) => println!("The index of first a is {}", i),
        None => println!("There is no a in the string")
    }
}

fn find_first_a(s : String)->Option<i32>{
    for(index, char) in s.chars().enumerate(){
        if char=='a'{
            return Some(index as i32);
        }
    }

    return None;
}
