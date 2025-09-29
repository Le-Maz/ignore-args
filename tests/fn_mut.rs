use ignore_args::ignore_args;

#[test]
fn works_for_fn_mut() {
    let mut counter = 0;
    let mut function = move || {
        counter += 1;
        counter
    };
    let mut modified_function = ignore_args(function);
    assert_eq!(modified_function(), 1);
    assert_eq!(modified_function(1), 2);
    assert_eq!(modified_function(1, ""), 3);
}
