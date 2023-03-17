#[allow(dead_code)]
#[test]
fn test_make_answer() {
    use macros::{make_answer, AnswerFn, HelperAttr};

    make_answer!();

    #[derive(AnswerFn, HelperAttr)]
    struct Foo {
        #[helper]
        bar: i32,
    }
    assert_eq!(answer(), 42);
    assert_eq!(answer_fn(), 42);
    let _foo = Foo { bar: 42 };
}
