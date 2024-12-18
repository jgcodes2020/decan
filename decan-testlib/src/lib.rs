
#[no_mangle]
pub extern "C" fn print_message() {
    println!("Hello, world!");
}

#[no_mangle]
pub extern "C" fn square_int(x: i32) -> i32 {
    x * x
}