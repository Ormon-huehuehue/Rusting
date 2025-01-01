fn main(){
    let mut vec = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    even_values(&mut vec); // passing in a mutable reference to the vector

    println!("Updated vector : {:?}", vec);
}

fn even_values(v : &mut Vec<i32>){

    let mut i = 0 ;

    while i < v.len(){
        if v[i] % 2 != 0{
            v.remove(i);
        }else{
            i += 1;
        }
    }
}