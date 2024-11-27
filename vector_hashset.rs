use std::collections::HashSet;
fn main() {
    
    //let mut set: HashSet<String> = vec!["a".to_string(), "c".to_string(), "c".to_string()].into_iter().collect();

    let mut ip_vec: Vec<String> = Vec::new();
    ip_vec.push("192.168.0.1".to_string());
    ip_vec.push("192.168.0.2".to_string());
    ip_vec.push("192.168.0.3".to_string());
    ip_vec.push("192.168.0.1".to_string());
    
    // Collect the Vector into a HashSet - then collect a deduped Vector.
    let mut de_duped = ip_vec.into_iter().collect::<HashSet<String>>().into_iter().collect::<Vec<String>>();
    de_duped.sort();

     for item in de_duped {
        println!("Deduped Ip Address: {}", item);
    }

}
