pub struct File {
    pub pathname: String,
    pub desc: String,
 } 
 
 pub struct Mac {
    pub addr: String,
    pub desc: String,
 }
 
 pub struct SepDataRow {
    pub text: String,
    pub desc: String,   
 }
 

 pub trait Separation {
    fn separator(&self, sep:&str) -> String;
 }


 impl Separation for File {
    fn separator(&self, sep:&str) -> String {
       self.pathname.replace(r#"\"#,sep)
    }
  }
  
  impl Separation for Mac {
    fn separator(&self, sep:&str) -> String {
       self.addr.replace("-",sep)
    }
  }

  impl Separation for SepDataRow {
    fn separator(&self, sep:&str) -> String {
       self.text.replace(",",sep)
    }
  }
  
 


fn main() {

    let new_file = File {
        pathname: String::from(r#"\temp\vol\file1.xml"#),
        desc: String::from("XML data file")
    };     
   
    let new_mac = Mac {
        addr: String::from("22-33-44-AA-BB-CC"),
	    desc: String::from("Network switch")
    };

    let new_csv = SepDataRow {
        text: String::from("100,200,300,400"),
	    desc: String::from("Integer List")
    };


    println!("SepDataRow is {}", new_csv.text);
    println!("SepDataRow is {}", new_csv.desc);
    println!("SepDataRow changed is {}", new_csv.separator("\t"));
    

}



 