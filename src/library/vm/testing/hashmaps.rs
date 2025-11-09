use core::panic;

use crate::{
    generate_vm_tests,
    object::{Object, hash},
    vm::testing::setups::run_vm_test,
};

pub(crate) fn should_match_maps(expected: &[[i64; 2]]) -> impl Fn(&Object) {
    move |object: &Object| match object {
        Object::HashMap(elements) => {
            assert_eq!(
                expected.len(),
                elements.len(),
                "Expecting {expected:?} got {elements:?}"
            );
            for [key, value] in expected {
                match elements.get(&hash(&Object::Int(*key))) {
                    Some(entry) => match entry.value.as_ref() {
                        Object::Int(v) => assert_eq!(
                            value, v,
                            "Expecting for {key} value {value} got {:?}",
                            entry.value
                        ),
                        _ => panic!("Expecting for {key} value {value} got {:?}", entry.value),
                    },
                    None => panic!("Expecting {key} in {elements:?}"),
                }
            }
        }
        _ => panic!("Expecting {expected:?} got {:?}", object),
    }
}

generate_vm_tests! {
    empty: ("{}", should_match_maps(&[])),
    numbers: ("{1:1,2:2,3:3}", should_match_maps(&[[1,1],[2,2],[3,3]])),
    complex: ("{1:1+1,2:2*3,3+1:3}", should_match_maps(&[[1,2],[2,6],[4,3]])),
}
