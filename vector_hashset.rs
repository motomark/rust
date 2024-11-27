use std::collections::HashSet;
fn main() {
    
    //let mut set: HashSet<String> = vec!["a".to_string(), "c".to_string(), "c".to_string()].into_iter().collect();

    let mut ip_vec: Vec<String> = Vec::new();
    ip_vec.push("192.168.0.1".to_string());
    ip_vec.push("192.168.0.2".to_string());
    ip_vec.push("192.168.0.3".to_string());
    ip_vec.push("192.168.0.1".to_string());
    
    //let mut set_alt: HashSet<String> = HashSet::new();
    let set_alt: HashSet<String> = ip_vec.into_iter().collect();
    let mut vac_deduped: Vec<String> = set_alt.into_iter().collect();
    vac_deduped.sort();

    for item in vac_deduped {
        println!("Deduped Ip Address: {}", item);
    }

}
