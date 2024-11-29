fn main() {

    let foo: bool = search((&"Hello World").to_string());
    println!("{}",foo);    

}


fn search(val:String)  -> bool{
    println!("{}",val);
    return true;
}