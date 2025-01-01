
fn main(){
    let s1 = String::from("Hello");

    do_something(s1.clone());      
    // s1 is moved to do_something into s2 and s1 is no longer valid

    println!("{}", s1);
}

fn do_something(s2 : String){
    println!("{}", s2)
}