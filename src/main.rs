fn main(){
	let nums = vec![1,2,3];
	let mut iter = nums.iter();

    while let Some(value) = iter.next() {
        println!("{}", value);
    }
    println!("{:?}", nums);
}