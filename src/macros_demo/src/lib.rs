use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Hello)]
pub fn hello(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Omit)]
pub fn hello2(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    let name = &ast.ident;
    let attr = &ast.attrs;
    let data = &ast.data;
    let generic = &ast.generics;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // println!("attr {}", stringify!(#attr));
                // println!("attr {:?}!", #attr.clone());
                // println!("data {:?}!", #data);
                // println!("generic {:?}!", #generic);
            }
        }
    };
    gen.into()
}

// #[macro_export]
// macro_rules! vec2 {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }
