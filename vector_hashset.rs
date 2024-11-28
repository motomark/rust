use std::collections::HashSet;
fn main() {
    
    // initialise and de-dupe all on one line.
    let mut ip_vec = vec!["192.168.0.1".to_string(),
            "192.168.0.2".to_string(),
            "192.168.0.3".to_string(),
            "192.168.0.1".to_string()]
        .into_iter().collect::<HashSet<String>>()
        .into_iter().collect::<Vec<String>>();
    
    ip_vec.sort();

     for item in ip_vec {
        println!("Deduped Ip Address: {}", item);
    }

}
