mod fs;
mod lib;
mod log;
mod vec;

use core::time;
use std::{
    sync::mpsc::{self, channel},
    thread,
};

fn main() {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("xxx");
        sender.send(val).unwrap()
    });
    let received_content = receiver.recv().unwrap();
    println!("received_content={}", received_content);
    // let str = "string";
    // let handle = thread::spawn(move || {
    //     for i in 0..10 {
    //         println!("spawned thread print str={}", str);
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // for i in 0..3 {
    //     println!("main thread print {}", i);
    //     thread::sleep(Duration::from_secs(1));
    // }
    // println!("str={}", str);
    // handle.join().unwrap();
    // let r;
    // {
    //     let s1 = "rust".to_string();
    //     let s2 = "rust-lang".to_string();
    //     let s3 = "rust-2";
    //     r = longer(s1, s2);
    // }
    // println!("{} is longer", r);
    // fs::main();
    // vec::call_str();

    println!("Done");
    // let i = 0;
    // match_number(i);
    // if i == 0 {zaa
    //     println!("center");
    // }
    // let prefix: String = "main".to_string();
    // let Log = crate::log::Console::new(prefix.clone());
    // let Log = self::log::Console::new(prefix.clone());
    // let Log = console::Console::new(prefix.clone()).log("str".to_string());
    // let Log = alert::Alert::new(prefix).info("str".to_string());
}

fn fn1(i: u8) -> Result<u8, String> {
    if i >= 5 {
        return Ok(i);
    }
    Err("Invalid".to_string())
}
fn fn2(i: u8) -> Result<u8, String> {
    let result = fn1(3)?;
    Ok(result)
}

// fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }
fn longer(s1: String, s2: String) -> String {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn match_number(i: i32) {
    match i {
        0 => print!("center"),
        1 => println!("left"),
        2 => println!("right"),
        4 => println!("4"),
        _ => (),
    }
}

// {
//     #[derive(Debug)]
//     struct A {
//       a::name: String,
//         nick: u8,
//     }
//     let a = A {
//         name: "a".to_string(),
//         nick: 0,
//     };

//     let b = A {
//         // name: "micheal".to_string(),
//         ..a
//     };
//     println!(
//         "b={:#?} name={}",
//         b,
//         b.name.to_string() + &b.nick.to_string()[..]
//     );
// }

// {
//     // struct
//     #[derive(Debug)]
//     struct A {
//         width: u32,
//         height: u32,
//     }
//     impl A {
//         fn create(width: u32, height: u32) -> A {
//             A { width, height }
//         }
//         fn new(width: u32, height: u32) -> A {
//             A { width, height }
//         }
//     }
//     println!("a={:#?},a2={:#?}", A::create(1200, 299), A::new(1, 2));
// }

// {
//     #[derive(Debug)]
//     enum A {
//         A { name: String },
//         B,
//         C,
//     }
//     let a = A::A {
//         name: "tang siji".to_string(),
//     };
//     println!("enum ={:#?}", a);
//     if let A::A { name } = &a {
//         println!("name={}", name);
//         if name == "micheal" {
//             print!("is micheal");
//         }
//     }

//     if let A::A { name } = a {
//         println!("name={}", name)
//     }
// }
