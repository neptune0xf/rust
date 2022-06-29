fn main() {
    println!("Hello, world!");
    call_hello(&mut HelloArg {
        name: "xxx".to_string(),
        id: 100,
    });
}

#[derive(Debug)]
struct HelloArg {
    name: String,
    id: u8,
}

fn call_hello(arg: &mut HelloArg) -> HelloArg {
    arg.id = 100;
    println!("arg {:?}", arg);
    HelloArg {
        name: "x".to_string(),
        id: 3,
    }
}
