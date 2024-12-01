
fn main() {
    let mut str_input = String::new();
    std::io::stdin().read_line(&mut str_input).expect("Unable to read from stdin");

    let mut res : bool = is_palindrome(&str_input);
    if res == true {
        println!("Yes String {} is a Palindrome", str_input.trim());
    } else {
        println!("No String {} is not Palindrome", str_input.trim());
    }

    let slice: &str = &str_input;
    res = is_palindrome(slice);

    if res == true {
        println!("Yes Slice {} is a Palindrome", str_input.trim());
    } else {
      
        println!("No Slice {} is not Palindrome", str_input.trim());
    }


}

/// Test if palindrome. We only need to pass in a immutable string Slice to test.
fn is_palindrome(input: &str) -> bool {

    let pal = input.trim().to_lowercase().replace(" ", "").replace(",","").replace("!","");
    let lap = pal.chars().rev().collect::<String>();

    if pal == lap  {
        return true;
    } else {
        return false;
    }

   
}