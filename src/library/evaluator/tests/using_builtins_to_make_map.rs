use super::evaluator_tests::eval_input;

#[test]
fn creating_map() {
    let input = r#"
    let map = fn(arr,f){
          let iter = fn(arr,accumulated) {
              if(len(arr)==0){
                  accumulated
              } else {
                iter(rest(arr), push(accumulated, f(first(arr))))
              }
          };
          iter(arr, [])
    };
    let a = [1,2,3];
    let double = fn(x) { x * 2 };
    map(a, double);
    "#;
    let object = eval_input(input);
    assert_eq!(object.to_string(), "[2, 4, 7]");
}
