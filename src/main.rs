use std::sync::mpsc;
use std::thread;

fn main(){
	let (tx, rx) = mpsc::channel();

	for i in 0..8 {
        let producer = tx.clone();

        thread::spawn(move || {
            let mut sum = 0;
            for j in i*1000..(i+1)*1000-1{
                sum +=j;
            }

            producer.send(sum);
        });

    }

    drop(tx);  //drop tx so that the receiver can know that there is no more data to receive


    let mut final_sum = 0;
    for value in rx{
        println!("Received: {}", value);
        final_sum += value;
    }

    println!("Final sum is: {}", final_sum);
}