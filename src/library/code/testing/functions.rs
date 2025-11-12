use crate::code::Instructions;
use crate::code::definitions::OpCodes;
use crate::code::make::make;
use crate::code::testing::test_compiler::Index;
use crate::code::testing::test_compiler::concat_instructions;
use crate::code::testing::test_compiler::test_be_integer;
use crate::code::testing::test_compiler::test_compilation;
use crate::generate_tests_for_compiler;
use crate::object::Object;

fn test_bytecode(values: Vec<Instructions>) -> Box<dyn Fn(&Object, Index)> {
    Box::new(move |object: &Object, i: Index| match object {
        Object::CompiledFunction(v) => {
            let expected = concat_instructions(&values.to_vec());
            for (i, (e, a)) in expected.bytes().iter().zip(v.bytes()).enumerate() {
                assert_eq!(
                    e, &a,
                    "Expecing {e:?} got {a:?} at {i:?} for {v}, expecting {expected}"
                )
            }
        }
        _ => panic!("Expecting compiled function got {:?} at {:?}", object, i),
    })
}

generate_tests_for_compiler! {

no_arg_returning: (
        "fn() {return 5 + 10}",
        vec![
            make(OpCodes::Constant.into(), &[2]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![
            test_be_integer(5),
            test_be_integer(10),
            test_bytecode(vec![
                make(OpCodes::Constant.into(), &[0]),
                make(OpCodes::Constant.into(), &[1]),
                make(OpCodes::Add.into(), &[]),
                make(OpCodes::ReturnValue.into(), &[]),
            ])
        ]
    ),

implicit_no_arg_returning: (
        "fn() {1;2}",
        vec![
            make(OpCodes::Constant.into(), &[2]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![
            test_be_integer(1),
            test_be_integer(2),
            test_bytecode(vec![
                make(OpCodes::Constant.into(), &[0]),
                make(OpCodes::Pop.into(), &[]),
                make(OpCodes::Constant.into(), &[1]),
                make(OpCodes::ReturnValue.into(), &[]),
            ])
        ]
    ),

empty_function: (
        "fn() {}",
        vec![
                make(OpCodes::Constant.into(), &[0]),
                make(OpCodes::Pop.into(), &[]),
        ],
        vec![
            test_bytecode(vec![
            make(OpCodes::ReturnNone.into(), &[]),
            ])
        ]
    ),
call_no_args_literal:(
        "fn(){25}()",
        vec![
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::Call.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![
            test_be_integer(25),
            test_bytecode(vec![
                make(OpCodes::Constant.into(), &[0]),
                make(OpCodes::ReturnValue.into(), &[]),
            ])
        ]
),

call_no_args:(
        "let noArg = fn(){25}; noArg()",
        vec![
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::SetGlobal.into(), &[]),
            make(OpCodes::GetGlobal.into(), &[]),
            make(OpCodes::Call.into(), &[]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![
            test_be_integer(25),
            test_bytecode(vec![
                make(OpCodes::Constant.into(), &[0]),
                make(OpCodes::ReturnValue.into(), &[]),
            ])
        ]
),
locals: (
        "
        fn() {
        let x = 1;
        x;
        }
        ",
        vec![
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![test_be_integer(1), test_bytecode(
        vec![
                make(OpCodes::Constant.into(), &[0]),
                make(OpCodes::SetLocal.into(), &[0]),
                make(OpCodes::GetLocal.into(), &[0]),
                make(OpCodes::ReturnValue.into(), &[]),
        ]
        )]

),
multiple_locals:  (
        "
        fn() {
        let x = 1;
        let y = 4;
        x+y;
        }
        ",
        vec![
            make(OpCodes::Constant.into(), &[2]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![test_be_integer(1), test_be_integer(4), test_bytecode(
        vec![
                make(OpCodes::Constant.into(), &[0]),
                make(OpCodes::SetLocal.into(), &[0]),
                make(OpCodes::Constant.into(), &[1]),
                make(OpCodes::SetLocal.into(), &[1]),
                make(OpCodes::GetLocal.into(), &[0]),
                make(OpCodes::GetLocal.into(), &[1]),
                make(OpCodes::Add.into(), &[]),
                make(OpCodes::ReturnValue.into(), &[]),
        ]
        )]

),

}
