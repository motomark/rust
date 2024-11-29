fn main() {
    let lowest_int = find_mn_value(100, 99);
    let lowest_char = find_mn_value('b', 'a');
    println!("{}", lowest_int);
    println!("{}", lowest_char);
}


//fn find_mn_value(val1: i32, val2: i32) -> i32 {

fn find_mn_value<T: std::cmp::PartialEq + std::cmp::PartialOrd + Clone>(val1: T, val2: T) -> T {
    if val1 <= val2 {
        return val1;
    } else {
        return val2;
    }
}

