trait Summary{
    fn summarize(&self)->String{
        return String::from("Summary")
    }
}

struct Fix{}
impl Summary for Fix{}

fn main(){

    let fix : Fix = Fix{};

	notify(fix)
}

fn notify(u : impl Summary){
	println!("{}", u.summarize());
}