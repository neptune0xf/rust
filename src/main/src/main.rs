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
