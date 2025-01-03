use std::{thread, time::Duration};

fn main(){
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    let async_handle = thread::spawn(|| {
        for i in 1..200 {
            println!("Hi number {i} from the async spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    async_handle.join().unwrap();



    for i in 1..500{
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}