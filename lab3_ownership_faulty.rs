fn modify_ext(old_file:String) -> usize {
    old_file.push_str(".json");
    return old_file.len()
}

fn main() {
    let tempfile:String = String::from("myfile");
    let namelen: usize = tempfile.len();
	
    println!("filename is {} chars long", namelen);
	
    // cannot assign twice because immutable.
    namelen = modify_ext(tempfile);

    // Cannot print because ownership has moved to modify_ext function.
    // A String does not implement the Copy trait so an only move ownership .. there is no copy.
    // We could clone the string (create a copy) but that would only have lifetime of the function.
    println!("filename is now: {}", tempfile);
    println!("filename is now {} chars long", namelen);
}
