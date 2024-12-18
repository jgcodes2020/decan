use std::path::Path;

use decan::{can::Can, SymbolGroup};

#[derive(SymbolGroup)]
pub struct DecanTestlib {
    pub print_message: extern "C" fn(),
    pub square_int: Option<extern "C" fn(i32) -> i32>,
}

#[test]
fn test_load() {
    let can = unsafe { Can::<_, DecanTestlib>::load(Path::new("./test.so")).unwrap() };

    (can.print_message)();
    assert_eq!((can.square_int.unwrap())(2), 4);
}