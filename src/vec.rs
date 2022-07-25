pub fn call() {
    let mut arr = vec![1, 2, 3, 4];
    // let mut arr: Vec<u8> = Vec::new();
    arr.push(2);
    for item in &arr {
        println!("item is {}", item);
    }
}

pub fn call_str() {
    let mut s = String::from("hello");
    s.push_str("string");
    s.push('c');
    let mut s2 = "s2";
    let s3 = s.clone() + s2;
    let s4 = format!("{}-{}-{}", s, s2, s3);
    println!("s4{}", s4);
}
