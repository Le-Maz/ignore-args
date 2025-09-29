use ignore_args::ignore_args_once;

#[test]
fn works() {
    let droppable = Box::new(0);
    let function = || droppable;
    let modified_function = ignore_args_once(function);
    assert_eq!(modified_function(), Box::new(0));
}
