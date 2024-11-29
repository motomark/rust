#[allow(dead_code)]

#[derive(Debug)]
enum Protocol {
    TCP,
    UDP
}

use Protocol::TCP;

#[derive(Debug)]
struct Connection {
    ipv4: [u8; 4],
    port: u32,
    r#type: Protocol
}

impl Connection {
    fn new()->Self{
        Self{
            ipv4: [0, 0, 0, 0], 
	  	  port: 0,
	  	  r#type: TCP
        }
    }
}

fn main() {
   let mut my_connection = Connection::new();
   my_connection.ipv4 = [192,168,1,66];
//    my_connection.ipv4[0] = 192;
//    my_connection.ipv4[1] = 168;
//    my_connection.ipv4[2] = 1;
//    my_connection.ipv4[3] = 66;
   my_connection.port = 1066;
   println!("{:?}", my_connection);
   println!("{:?}", my_connection.r#type);
}