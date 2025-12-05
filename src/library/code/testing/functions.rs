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
        Object::CompiledFunction(f) => {
            let expected = concat_instructions(&values.to_vec());
            for (i, (e, a)) in expected
                .bytes()
                .iter()
                .zip(f.instructions.bytes())
                .enumerate()
            {
                assert_eq!(
                    e, &a,
                    "Expecing {e:?} got {a:?} at {i:?} for {}, expecting:\n{expected}\ngot\n{}",
                    f.number_of_parameters, f.instructions
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
            make(OpCodes::Closure.into(), &[2]),
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
            make(OpCodes::Closure.into(), &[2]),
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
                make(OpCodes::Closure.into(), &[0]),
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
            make(OpCodes::Closure.into(), &[1]),
            make(OpCodes::Call.into(), &[0]),
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
            make(OpCodes::Closure.into(), &[1]),
            make(OpCodes::SetGlobal.into(), &[]),
            make(OpCodes::GetGlobal.into(), &[]),
            make(OpCodes::Call.into(), &[0]),
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
            make(OpCodes::Closure.into(), &[1]),
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
            make(OpCodes::Closure.into(), &[2]),
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
one_argument: (
"
let fun = fn(a){ }
fun(6)
",
    vec![
         make(OpCodes::Closure.into(), &[0]),
         make(OpCodes::SetGlobal.into(), &[0]),
         make(OpCodes::GetGlobal.into(), &[0]),
         make(OpCodes::Constant.into(), &[1]),
         make(OpCodes::Call.into(), &[1]),
         make(OpCodes::Pop.into(), &[]),

    ],
    vec![
        test_bytecode(vec![
            make(OpCodes::ReturnNone.into(), &[])
        ]
        ),
        test_be_integer(6)
    ]
),
many_argument: (
"
let fun = fn(a,b,c){}
fun(6,7,8)
",
    vec![
         make(OpCodes::Closure.into(), &[0]),
         make(OpCodes::SetGlobal.into(), &[0]),
         make(OpCodes::GetGlobal.into(), &[0]),
         make(OpCodes::Constant.into(), &[1]),
         make(OpCodes::Constant.into(), &[2]),
         make(OpCodes::Constant.into(), &[3]),
         make(OpCodes::Call.into(), &[3]),
         make(OpCodes::Pop.into(), &[]),

    ],
    vec![
        test_bytecode(vec![
            make(OpCodes::ReturnNone.into(), &[])
        ]),
        test_be_integer(6),
        test_be_integer(7),
        test_be_integer(8),
    ]
),

one_argument_used: (
r#"
let one = fn(a){ a }
one(6)
"#,
    vec![
         make(OpCodes::Closure.into(), &[0]),
         make(OpCodes::SetGlobal.into(), &[0]),
         make(OpCodes::GetGlobal.into(), &[0]),
         make(OpCodes::Constant.into(), &[1]),
         make(OpCodes::Call.into(), &[1]),
         make(OpCodes::Pop.into(), &[]),

    ],
    vec![
        test_bytecode(vec![
            make(OpCodes::GetLocal.into(), &[0]),
            make(OpCodes::ReturnValue.into(), &[])
        ]
        ),
        test_be_integer(6)
    ]
),
many_argument_used: (
"
let fun = fn(a,b,c){ a;b;c;}
fun(6,7,8)
",
    vec![
         make(OpCodes::Closure.into(), &[0]),
         make(OpCodes::SetGlobal.into(), &[0]),
         make(OpCodes::GetGlobal.into(), &[0]),
         make(OpCodes::Constant.into(), &[1]),
         make(OpCodes::Constant.into(), &[2]),
         make(OpCodes::Constant.into(), &[3]),
         make(OpCodes::Call.into(), &[3]),
         make(OpCodes::Pop.into(), &[]),

    ],
    vec![
        test_bytecode(vec![
            make(OpCodes::GetLocal.into(), &[0]),
            make(OpCodes::Pop.into(), &[]),
            make(OpCodes::GetLocal.into(), &[1]),
            make(OpCodes::Pop.into(), &[]),
            make(OpCodes::GetLocal.into(), &[2]),
            make(OpCodes::ReturnValue.into(), &[])
        ]),
        test_be_integer(6),
        test_be_integer(7),
        test_be_integer(8),
    ]
),

closure: (
"
fn(a) { fn(b){ a + b } }
",
    vec![
         make(OpCodes::Closure.into(), &[1,0]),
         make(OpCodes::Pop.into(), &[]),

    ],
    vec![
        test_bytecode(vec![
            make(OpCodes::GetFree.into(), &[0]),
            make(OpCodes::GetLocal.into(), &[0]),
            make(OpCodes::Add.into(), &[]),
            make(OpCodes::ReturnValue.into(), &[])
        ]),
        test_bytecode(vec![
            make(OpCodes::GetLocal.into(), &[0]),
            make(OpCodes::Closure.into(), &[0, 1]),
            make(OpCodes::ReturnValue.into(), &[])
        ])
    ]
),
deep_closure: (
"
fn(a) { fn(b){ fn(c) { a + b + c } }
",
    vec![
         make(OpCodes::Closure.into(), &[2,0]),
         make(OpCodes::Pop.into(), &[]),

    ],
    vec![
        test_bytecode(vec![
            make(OpCodes::GetFree.into(), &[0]),
            make(OpCodes::GetFree.into(), &[1]),
            make(OpCodes::Add.into(), &[]),
            make(OpCodes::GetLocal.into(), &[0]),
            make(OpCodes::Add.into(), &[]),
            make(OpCodes::ReturnValue.into(), &[])
        ]),
        test_bytecode(vec![
            make(OpCodes::GetFree.into(), &[0]),
            make(OpCodes::GetLocal.into(), &[0]),
            make(OpCodes::Closure.into(), &[0, 2]),
            make(OpCodes::ReturnValue.into(), &[])
        ]),
        test_bytecode(vec![
            make(OpCodes::GetLocal.into(), &[0]),
            make(OpCodes::Closure.into(), &[1, 1]),
            make(OpCodes::ReturnValue.into(), &[])
        ])
    ]
),
using_closures: (
"
let global = 1;
fn() {
  let a = 2;
  fn(){
    let b = 3;
    fn() {
      let c = 4
      global + a + b + c  
    }
  } 
}
",
    vec![
         make(OpCodes::Constant.into(), &[0]),
         make(OpCodes::SetGlobal.into(), &[0]),
         make(OpCodes::Closure.into(), &[6,0]),
         make(OpCodes::Pop.into(), &[]),
    ],
    vec![
        test_be_integer(1),
        test_be_integer(2),
        test_be_integer(3),
        test_be_integer(4),
        test_bytecode(vec![
            make(OpCodes::Constant.into(), &[3]),
            make(OpCodes::SetLocal.into(), &[0]),
            make(OpCodes::GetGlobal.into(), &[0]),
            make(OpCodes::GetFree.into(), &[0]),
            make(OpCodes::Add.into(), &[]),
            make(OpCodes::GetFree.into(), &[1]),
            make(OpCodes::Add.into(), &[]),
            make(OpCodes::GetLocal.into(), &[0]),
            make(OpCodes::Add.into(), &[]),
            make(OpCodes::ReturnValue.into(), &[])
        ]),
        test_bytecode(vec![
            make(OpCodes::Constant.into(), &[2]),
            make(OpCodes::SetLocal.into(), &[0]),
            make(OpCodes::GetFree.into(), &[0]),
            make(OpCodes::GetLocal.into(), &[0]),
            make(OpCodes::Closure.into(), &[4, 2]),
            make(OpCodes::ReturnValue.into(), &[])
        ]),
        test_bytecode(vec![
            make(OpCodes::Constant.into(), &[1]),
            make(OpCodes::SetLocal.into(), &[0]),
            make(OpCodes::GetLocal.into(), &[0]),
            make(OpCodes::Closure.into(), &[5, 1]),
            make(OpCodes::ReturnValue.into(), &[])
        ]),
    ]
),
recursive: (
"
let countDown = fn(x) { countDown(x-1); }
countDown(1)
",
vec![
    make(OpCodes::Closure.into(),&[1,0]),
    make(OpCodes::SetGlobal.into(), &[0]),
    make(OpCodes::GetGlobal.into(), &[0]),
    make(OpCodes::Constant.into(), &[2]),
    make(OpCodes::Call.into(), &[1]),
    make(OpCodes::Pop.into(), &[]),
],
vec![
    test_be_integer(1),
    test_bytecode(vec![
        make(OpCodes::CurrentClosure.into(),&[]),
        make(OpCodes::GetLocal.into(),&[0]),
        make(OpCodes::Constant.into(),&[0]),
        make(OpCodes::Subtitute.into(),&[]),
        make(OpCodes::Call.into(),&[1]),
        make(OpCodes::ReturnValue.into(),&[]),
    ]),
    test_be_integer(1)
]
),

recursive_wrapped: (
"
let wrapper = fn(){
    let countDown = fn(x) { countDown(x-2); }
    countDown(1)
};
wrapper()
",
vec![
    make(OpCodes::Closure.into(),&[3,0]),
    make(OpCodes::SetGlobal.into(), &[0]),
    make(OpCodes::GetGlobal.into(), &[0]),
    make(OpCodes::Call.into(), &[0]),
    make(OpCodes::Pop.into(), &[]),
],
vec![
    test_be_integer(2),
    test_bytecode(vec![
        make(OpCodes::CurrentClosure.into(),&[]),
        make(OpCodes::GetLocal.into(),&[0]),
        make(OpCodes::Constant.into(),&[0]),
        make(OpCodes::Subtitute.into(),&[]),
        make(OpCodes::Call.into(),&[1]),
        make(OpCodes::ReturnValue.into(),&[]),
    ]),
    test_be_integer(1),
    test_bytecode(vec![
        make(OpCodes::Closure.into(),&[1,0]),
        make(OpCodes::SetLocal.into(),&[0]),
        make(OpCodes::GetLocal.into(),&[0]),
        make(OpCodes::Constant.into(),&[2]),
        make(OpCodes::Call.into(),&[1]),
        make(OpCodes::ReturnValue.into(),&[]),
    ]),
]
),
}
