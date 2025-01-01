// struct User {
//     first_name : String,
//     last_name  : String,
//     age : u8
// }

// struct Rect{
//     width : u32,
//     height : u32
// }

// impl Rect{
//     fn area(&self)-> u32{
//         return self.width * self.height;
//     }
// }

enum Direction{
    Up(String, f64), 
    Down(String, f64)
}

fn main(){
   let new_direction = Direction::Up(String::from("Up"), 30.0);
   let _down_direction: Direction = Direction::Down(String::from("Down"), 40.0);
   

   match new_direction{
    Direction::Up(direction, distance)=>{
        println!("Moving {}km in {} direction", distance, direction);
    }

    Direction::Down(direction, distance)=>{
        println!("Moving {}km in {} direction", distance, direction);
    }
   }
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