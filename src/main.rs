fn main(){
	let nums = vec![1,2,3];
	let iter = nums.iter();    

    let sum : i32 = iter.sum();     // sum() consumes the iterator

    println!("{}", sum);

    for val in iter{    //iter is no longer valid
        println!("{}", val);
    }

    println!("{:?}", nums); 
}