use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{self, stdin, Read},
};

pub fn main() -> Result<(), String> {
    let args = env::args();
    for key in args {
        println!("key {:#?}", key);
    }
    // let mut input_buffer = String::new();
    // stdin()
    //     .read_line(&mut input_buffer)
    //     .expect("Failed read_line");

    // println!("input line is \n {}", input_buffer);

    // let text = fs::read_to_string("cargo.toml")?;
    // println!("text {:#?}", text);
    // let text = fs::read("cargo.toml");
    // match text {
    //     Ok(str) => println!("text {:#?}", str),
    //     Err(err) => println!("text {:#?}", err),
    // }

    let mut buffer = [0u8; 5];
    let mut file = File::open("cargo.toml").unwrap();
    loop {
        let reader = file.read_exact(&mut buffer);
        match reader {
            Ok(_) => println!("s"),
            Err(err) => {
                println!("read buffer error {:?}", err);
                break;
            }
        }
        println!("buffer {:?}", buffer);
    }

    Ok(())
}
