use crate::code::Instructions;
use crate::code::definitions::OpCodes;
use crate::code::make::make;
use crate::code::testing::test_compiler::Index;
use crate::code::testing::test_compiler::concat_instructions;
use crate::code::testing::test_compiler::test_be_integer;
use crate::code::testing::test_compiler::test_compilation;
use crate::generate_tests_for_compiler;
use crate::object::BuiltInFunction;
use crate::object::Object;

fn test_bytecode(values: Vec<Instructions>) -> Box<dyn Fn(&Object, Index)> {
    Box::new(move |object: &Object, i: Index| match object {
        Object::CompiledFunction {
            instructions,
            number_of_locals,
            number_of_parameters: _,
        } => {
            let expected = concat_instructions(&values.to_vec());
            for (i, (e, a)) in expected
                .bytes()
                .iter()
                .zip(instructions.bytes())
                .enumerate()
            {
                assert_eq!(
                    e, &a,
                    "Expecing {e:?} got {a:?} at {i:?} for {number_of_locals}, expecting {expected}"
                )
            }
        }
        _ => panic!("Expecting compiled function got {:?} at {:?}", object, i),
    })
}

generate_tests_for_compiler! {

using_builtins: (
        "
        len([]);
        push([],1);
        ",
        vec![
            make(OpCodes::GetBuiltin.into(), &[BuiltInFunction::Len.index() as u16]),
            make(OpCodes::Array.into(), &[0]),
            make(OpCodes::Call.into(), &[1]),
            make(OpCodes::Pop.into(), &[]),
            make(OpCodes::GetBuiltin.into(), &[BuiltInFunction::Push.index() as u16]),
            make(OpCodes::Array.into(), &[0]),
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Call.into(), &[2]),
            make(OpCodes::Pop.into(), &[]),

        ],
        vec![
            test_be_integer(1),
        ]
    ),

builtin_in_function: (
        "
        fn() {
            len([])
        }
        ",
        vec![
            make(OpCodes::Constant.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
        ],
        vec![
            test_bytecode(vec![
                make(OpCodes::GetBuiltin.into(), &[BuiltInFunction::Len.index() as u16]),
                make(OpCodes::Array.into(), &[0]),
                make(OpCodes::Call.into(), &[1]),
                make(OpCodes::ReturnValue.into(), &[]),
            ])
        ]
    ),
}
