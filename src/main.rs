fn main(){
	let nums = vec![1,2,3];
	let iter = nums.iter();    

    let iter2 = iter.map(|x| x+1);

    let nums2 : Vec<i32> = iter2.collect();

    println!("nums2 : {:?}", nums2);

    for i in iter2 {    //iter2 is no longer valid
        println!("{}", i);
    }
    println!("nums still intact : {:?}", nums);
}