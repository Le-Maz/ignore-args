use ignore_args::ignore_args_mut;

#[test]
fn works() {
    let mut counter = 0;
    let function = move || {
        counter += 1;
        counter
    };
    let mut modified_function = ignore_args_mut(function);
    assert_eq!(modified_function(), 1);
    assert_eq!(modified_function(1), 2);
    assert_eq!(modified_function(1, ""), 3);
}
