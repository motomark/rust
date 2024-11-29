fn main() {

    let mut data:[i32;5] = [10, 20, 30, 40, 50];

    unsafe {

        let value = &mut data[0] as *mut i32;
        *value.offset(1) = -99;

    }



    
    println!("{:?}", data);
}



