fn main(){
    let word = "ormon verma";
    let word2 = find_first_word(word);
    
    println!("{}", word);
    println!("{}", word2);

}

fn find_first_word(word : &str)-> &str{
    let mut index= 0 ;
    for (_, i) in word.chars().enumerate() {
        if i == ' '{
            break;
        }
        index = index+1;
    }

    return &word[0..index];
}