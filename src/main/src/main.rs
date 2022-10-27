use std::fmt::{Binary, Display};

use macros_demo::{Hello, Omit};

fn main() {
    // println!("Hello, world!");
    // Hi::hello_macro();
    // let a = vec2!(1, 2, 3);

    #[derive(Debug)]
    struct A {
        name: String,
        id: u8,
    }

    // #[derive(Omit(AA,"name"))]
    // struct B {}

    // A::hello_macro();
    // let a = A {};
    // println!("a is {:?}", a);
}

#[derive(Hello, Debug, Clone)]
struct Hi {}
pub trait HelloMacro {
    fn hello_macro() {}
}

#[derive(Omit, Debug)]
struct A {}

#[macro_export]
macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[test] // Tuples can be used as function arguments and as return values
fn main_test() {
    // Globals are declared outside all other scopes.
    static LANGUAGE: &str = "Rust";
    static mut THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        // Access constant in some function
        unsafe { n > THRESHOLD }
    }

    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", unsafe { THRESHOLD });
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.

    unsafe {
        THRESHOLD = 5;
    }
    // FIXME ^ Comment out this line
}
