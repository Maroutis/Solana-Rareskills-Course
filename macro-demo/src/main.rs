// src/main.rs
// Import the macro_demo crate and bring all items into scope with the `*` wildcard
// (basically everything in this crate, including our macro in `src/lib.rs`
use macro_demo::*;

// Apply the `foo_bar_attribute` procedural attribute-like macro we created in `src/lib.rs` to `struct MyStruct`
// The procedural macro will generate a new struct definition with specified fields and methods
// #[foo_bar_attribute]
#[destroy_attribute]
struct MyStruct {
    baz: i32,
}

fn main() {
    // Create a new instance of `MyStruct` using the `default()` method
    // This method is provided by the `Default` trait implementation generated by the macro
    // let demo = MyStruct::default();

    // // Print the contents of `demo` to the console
    // // The `Debug` trait implementation generated by the macro allows formatted output with `println!`
    // println!("struct is {:?}", demo);

    // // Call the `double_foo()` method on `demo`
    // // This method is generated by the macro and returns double the value of the `foo` field
    // let double_foo = demo.double_foo();

    // // Print the result of calling `double_foo` to the console
    // println!("double foo: {}", double_foo);

    let demo = MyStruct { baz: 3, qux: 4 };

    println!("struct is {:?}", demo);
}

