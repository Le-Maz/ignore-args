use ignore_args::ignore_args;

#[test]
fn works_for_fn() {
    let function = || "hello";
    let modified_function = ignore_args(function);
    assert_eq!(modified_function(), "hello");
    assert_eq!(modified_function(1), "hello");
    assert_eq!(modified_function(1, ""), "hello");
}

#[test]
fn works_for_fn_by_ref() {
    let function = || "hello";
    let modified_function = ignore_args(&function);
    assert_eq!(modified_function(), "hello");
    assert_eq!(modified_function(1), "hello");
    assert_eq!(modified_function(1, ""), "hello");
}

#[test]
fn works_for_fn_mut_by_ref() {
    let mut counter = 0;
    let mut function = move || {
        counter += 1;
        counter
    };
    let mut modified_function = ignore_args(&mut function);
    assert_eq!(modified_function(), 1);
    assert_eq!(modified_function(1), 2);
    assert_eq!(modified_function(1, ""), 3);
}

#[test]
fn has_correct_layout() {
    let original_function = || "hello";
    let modified_function = ignore_args(original_function);
    assert_eq!(
        std::mem::size_of_val(&original_function),
        std::mem::size_of_val(&modified_function)
    );
}
