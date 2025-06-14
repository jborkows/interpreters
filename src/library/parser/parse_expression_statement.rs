#[macro_export]
macro_rules! expected_same_hash {
    ($($name:ident: ($input:expr, $other:expr),)*) => {
        $(
            #[test]
            fn $name() {
                 crate::evaluator::tests::evaluator_tests::should_be_integer_equal_to($input, $expected);
            }
        )*
    };
}
