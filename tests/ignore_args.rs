use ignore_args::{ignore_args};

#[test]
fn works() {
    let function = || "hello";
    let modified_function = ignore_args(function);
    assert_eq!(modified_function(), "hello");
    assert_eq!(modified_function(1), "hello");
    assert_eq!(modified_function(1, ""), "hello");
}

#[test]
fn is_transparent() {
    let original_function = || "hello";
    let modified_function = ignore_args(original_function);
    assert_eq!(
        std::mem::size_of_val(&original_function),
        std::mem::size_of_val(&modified_function)
    );
}
