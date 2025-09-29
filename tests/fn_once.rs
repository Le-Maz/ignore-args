use ignore_args::ignore_args;

#[test]
fn works_for_fn_once() {
    let droppable = Box::new(0);
    let function = move || droppable;
    let modified_function = ignore_args(function);
    assert_eq!(modified_function(), Box::new(0));
}
