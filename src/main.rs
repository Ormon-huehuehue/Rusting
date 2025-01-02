fn main(){
	let nums = vec![1,2,3];
	let iter = nums.into_iter();    //ownership of nums transferred to iter

    for val in iter{
        println!("{}", val);
    }

    println!("{:?}", nums); //nums is no longer valid
}