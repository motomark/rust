
fn main() {
    let mut str_input = String::new();
    std::io::stdin().read_line(&mut str_input).expect("Unable to read from stdin");
    let pal = str_input.trim().to_lowercase().replace(" ", "").replace(",","").replace("!","");
    let lap = pal.chars().rev().collect::<String>();

    
    if pal == lap  {
        println!("Yes its a Palindrome");
    } else {
        println!("No Palindrome");
    }

}