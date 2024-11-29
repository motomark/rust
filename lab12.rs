fn main() {

    let password = String::from("Password1");


    if check_length(&password) && check_uppercase(&password) && check_digit(&password) {
        println!("{}", "Success");
    } else {
        println!("{}", "Invalid");
    }
}


fn check_length(password: &str) -> bool {
    if password.len() >= 8 {
        return true;
    } else {
        return false;
    }
    
}

fn check_uppercase(password: &str) -> bool {
    let mut valid: bool = false;
    for c in password.chars() {
        if c.is_ascii_uppercase() {
            valid = true;
            break;
        }
    }
    return valid;
}


fn check_digit(password: &str) ->bool {
    let mut valid: bool = false;
    for c in password.chars() {
        if c.is_numeric() {
            valid = true;
            break;
        }
    }
    return valid;
}