#[derive(Debug)]
struct Values<T> {
    x: T,
    y: T
}


fn main() {
    let lowest_int = find_mn_value(100, 99);
    let lowest_char = find_mn_value('b', 'a');

    println!("Lowest Int:{}", lowest_int);
    println!("Lowest Char:{}", lowest_char);


    let float_vals = Values{x: 5.5, y: 7.5};
    let char_vals = Values{x: 'a', y: 'b'};
    

    let from_struct_float = find_mn_value_struct(float_vals);
    let from_struct_char = find_mn_value_struct(char_vals);

    println!("Lowest Float from Struct: {}", from_struct_float);
    println!("Lowest Char from Struct: {}", from_struct_char);


}

fn find_mn_value<T: std::cmp::PartialEq + std::cmp::PartialOrd + Clone>(val1: T, val2: T) -> T {
    if val1 <= val2 {
        return val1;
    } else {
        return val2;
    }
}


fn find_mn_value_struct<T: std::cmp::PartialEq + Clone + std::cmp::PartialOrd>(data: Values<T>) -> T {
    if data.x <= data.y {
        return data.x ;
    } else {
        return data.y;
    }
}

// fn find_mn_value_struct_generic<T: std::cmp::PartialEq + Clone + std::cmp::PartialOrd>(data: T) -> T {
//     if data.x <= data.y {
//         return data.x ;
//     } else {
//         return data.y;
//     }
// }

