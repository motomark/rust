fn main() {

    let domain = "qa.com";
    let email = String::from("mark.fishpool") + "@" + domain;
    println!("Email is {}", email);


    let name = String::from("mark");
    let name2 = "h";
    let mut name4 = String::from("a");
    
    let name3: String = name+name2;
    name3+&name4;

    println!("{}", name3);



}