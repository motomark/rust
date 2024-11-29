macro_rules! printdupln {
    ($text:expr, $repeat:expr) => {
        println!("{}",$text.repeat($repeat));
    }
}


fn main() {
    printdupln!("hello", 3); 
    printdupln!("Mark", 10);

}
