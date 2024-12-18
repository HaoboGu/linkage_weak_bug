use linkage_weak_bug::hello;

#[no_mangle]
const fn get_foo() -> i32 {
    1
}

fn main() {
    // Output: hello: 0, 1
    // Expected: hello: 1, 1
    hello();
}
