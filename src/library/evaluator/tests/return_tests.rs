use super::evaluator_tests::{should_be_boolean_equal_to, should_be_integer_equal_to};

#[test]
fn test_return() {
    should_be_integer_equal_to("return 10;", 10);
    should_be_integer_equal_to("1;return 2;3", 2);
}

#[test]
fn return_in_nested() {
    should_be_integer_equal_to(
        r#"
        if (10 > 1) {
           if( 10 > 1){
               return 10;
           }
           return 1;
        }
        "#,
        10,
    );
}
#[test]
fn return_and_calculation() {
    should_be_integer_equal_to(
        r#"
        2 + if (10 > 1) {
           if( 10 > 1){
               return 10;
           }
           return 1;
        }
        "#,
        10,
    );
}

#[test]
fn return_and_nested_if_logic() {
    should_be_boolean_equal_to(
        r#"
        !if (10 > 1) {
           if( 10 > 1){
               return true;
           }
           return false;
        } else {
           return false;
        }
        "#,
        true,
    );
}
