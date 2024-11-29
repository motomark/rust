use std::collections::HashSet;

fn main() {
    
    let mut address_list: Vec<&str> = vec!["130.0.242.1", "130.0.242.2","130.0.242.3","130.0.242.3","130.0.242.1", "130.0.242.1"];
    address_list.sort();
    address_list.dedup();

    for item in &address_list {
       println!("Deduped {}", item);
    }

    let mut set = HashSet::new();




    
    



}