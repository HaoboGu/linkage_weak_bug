#![feature(linkage)]

#[linkage = "weak"]
#[no_mangle]
const fn get_foo() -> i32 {
    0
}

pub const FOO: i32 = get_foo();

pub fn hello() {
    println!("hello: {}, {}", FOO, get_foo());
}