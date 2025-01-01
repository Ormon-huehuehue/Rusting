// struct User {
//     first_name : String,
//     last_name  : String,
//     age : u8
// }

struct Rect{
    width : u32,
    height : u32
}

impl Rect{
    fn area(&self)-> u32{
        return self.width * self.height;
    }
}


fn main(){
    // let name = String::from("Armaan");
    // let len = get_string_length(name);

    // println!("Lenght of name {}", len);

    // let user = User{
    //     first_name : String::from("Armaan"),
    //     last_name : String::from("Verma"),
    //     age : 18
    // };


    // println!("first name : {}", user.first_name);
    // println!("last name : {}", user.last_name);
    // println!("age : {}", user.age);

    let rect1 = Rect {
        width:  30,
        height : 30
    };

    println!("Area of rectangle : {}", rect1.area())
}





// fn get_string_length(s : String) -> usize {
//     s.chars().count()
// }


// fn is_even(num : i32) -> bool {
//     if num % 2 == 0 {
//         return true;
//     }
//     else{
//         return false;
//     }
// }

// fn fib(num : i32)-> i32 {
//     let mut first = 0;
//     let mut second = 1;

//     println!("num entered  : {}", num);
//     if num == 0 {
//         return first;
//     }
//     if num ==1 {
//         return 1;
//     }

//     for _i in 2..=num {
    
//          let temp = second;
//          second =  second + first;
//          first = temp;
//     } 

//     return second;
// }