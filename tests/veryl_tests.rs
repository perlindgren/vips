// file: tests/simple_test.rs
use marlin::veryl::prelude::*;
use snafu::Whatever;

#[veryl(src = "src/full_adder.veryl", name = "FullAdder")]
pub struct FullAdder;

#[veryl(src = "src/alu.veryl", name = "Alu")]
pub struct Alu;

// #[test]
// #[snafu::report]
// fn test_full_adder() -> Result<(), Whatever> {
//     let runtime = VerylRuntime::new(VerylRuntimeOptions {
//         call_veryl_build: true, /* warning: not thread safe! don't use if you
//                                  * have multiple tests */
//         ..Default::default()
//     })?;

//     let mut full_adder = runtime.create_model::<FullAdder>()?;

//     full_adder.a = 0;
//     full_adder.b = 0;
//     full_adder.c = 0;
//     full_adder.eval();
//     println!("sum {}, carry {}", full_adder.sum, full_adder.carry);
//     assert!(full_adder.sum == 0 && full_adder.carry == 0);

//     full_adder.a = 1;
//     full_adder.eval();
//     println!("sum {}, carry {}", full_adder.sum, full_adder.carry);
//     assert!(full_adder.sum == 1 && full_adder.carry == 0);

//     full_adder.b = 1;
//     full_adder.eval();
//     println!("sum {}, carry {}", full_adder.sum, full_adder.carry);
//     assert!(full_adder.sum == 0 && full_adder.carry == 1);

//     full_adder.c = 1;
//     full_adder.eval();
//     println!("sum {}, carry {}", full_adder.sum, full_adder.carry);
//     assert!(full_adder.sum == 1 && full_adder.carry == 1);

//     // full_adder = FullAdder { a: 1, ..full_adder };
//     // // full_adder.eval();

//     // println!("sum {}, carry {}", full_adder.sum, full_adder.carry);
//     // assert!(full_adder.sum == 1 && full_adder.carry == 0);

//     Ok(())
// }

#[test]
#[snafu::report]
fn test_alu() -> Result<(), Whatever> {
    fn dump(alu: &Alu) {
        println!(
            "a {}, b {}, sub {}, op {}‚ r {}, v {}, c {}‚ z {}",
            alu.a, alu.b, alu.sub, alu.op, alu.r, alu.v, alu.c, alu.z
        );
    }

    let runtime = VerylRuntime::new(VerylRuntimeOptions {
        call_veryl_build: true, /* warning: not thread safe! don't use if you
                                 * have multiple tests */
        ..Default::default()
    })?;

    let mut alu = runtime.create_model::<Alu>()?;

    alu.a = 0;
    alu.b = 0;
    alu.sub = 0;
    alu.op = 0;

    alu.eval();
    dump(&alu);

    alu.a = 100;
    alu.op = 2;
    alu.eval();
    dump(&alu);

    alu.b = 100;
    alu.eval();
    dump(&alu);

    alu.sub = 1;
    alu.eval();
    dump(&alu);

    alu.b = 101;
    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, u32::MAX);

    // alu = Alu { ..alu };

    Ok(())
}
