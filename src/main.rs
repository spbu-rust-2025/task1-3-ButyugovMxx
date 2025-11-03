use std::io::{self, Read};
use std::fs::File;

fn main() {
    let mut s = String::new();
    if io::stdin().read_line(&mut s).is_err(){
        println!("failure");
        return;
    }

    let path = s.trim();

    let res = File::open(path);

    let mut file = match res{
        Ok(f) => f,
        Err(_) => {
            println!("failure");
            return;
        }
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_ok(){
        println!("success");
    }else{
        println!("failure");
    }
}
