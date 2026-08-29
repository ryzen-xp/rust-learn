// Topic: Stop_gninnipS_My_sdroW! 
// Run with:  cargo run --bin Stop_gninnipS_My_sdroW!

fn main() {
assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(spin_words("You are almost to the last test"), "You are tsomla to the last test");
        assert_eq!(spin_words("Just kidding there is still one more"), "Just gniddik ereht is llits one more");
        assert_eq!(spin_words("Seriously this is the last one"), "ylsuoireS this is the last one");

        println!("Pass");
}
fn spin_words(words: &str)-> String  {
    let  chars: Vec<String>  = words.split(" ").map(|x| {
       if x.len() >=5 {
            reversal(x)
        }
        else {
            x.to_string()
        }
    }).collect();

    println!("{:?}" , chars.join(" "));


   chars.join(" ")

}

fn reversal( word: &str) -> String{
   
     word.chars().rev().collect()

}

