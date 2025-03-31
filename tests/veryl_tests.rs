use std::env;

use marlin::veryl::prelude::*;
use snafu::Whatever;

#[veryl(src = "src/full_adder.veryl", name = "FullAdder")]
pub struct FullAdder;

#[veryl(src = "src/alu32.veryl", name = "Alu32")]
pub struct Alu32;

#[test]
#[snafu::report]
fn test_full_adder() -> Result<(), Whatever> {
    let runtime = VerylRuntime::new(VerylRuntimeOptions {
        call_veryl_build: env::var("RUNNING_TESTS_INDEPENDENTLY")
            .map(|value| &value == "1")
            .unwrap_or(false),
        ..Default::default()
    })?;

    let mut full_adder = runtime.create_model::<FullAdder>()?;

    full_adder.a = 0;
    full_adder.b = 0;
    full_adder.c = 0;
    full_adder.eval();
    println!("sum {}, carry {}", full_adder.sum, full_adder.carry);
    assert!(full_adder.sum == 0 && full_adder.carry == 0);

    full_adder.a = 1;
    full_adder.eval();
    println!("sum {}, carry {}", full_adder.sum, full_adder.carry);
    assert!(full_adder.sum == 1 && full_adder.carry == 0);

    full_adder.b = 1;
    full_adder.eval();
    println!("sum {}, carry {}", full_adder.sum, full_adder.carry);
    assert!(full_adder.sum == 0 && full_adder.carry == 1);

    full_adder.c = 1;
    full_adder.eval();
    println!("sum {}, carry {}", full_adder.sum, full_adder.carry);
    assert!(full_adder.sum == 1 && full_adder.carry == 1);

    // struct assign, there are some hidden fields, thus ..full_adder is needed.
    full_adder = FullAdder {
        a: 1,
        b: 0,
        c: 0,
        ..full_adder
    };

    full_adder.eval();

    println!("sum {}, carry {}", full_adder.sum, full_adder.carry);
    assert!(full_adder.sum == 1 && full_adder.carry == 0);

    Ok(())
}

#[test]
#[snafu::report]
fn test_alu() -> Result<(), Whatever> {
    //let _ = INIT.clone();
    fn dump(alu: &Alu32) {
        println!(
            "a {}, b {}, sub {}, op {}‚ r {}, v {}, c {}‚ z {}",
            alu.a, alu.b, alu.sub, alu.op, alu.r, alu.v, alu.c, alu.z
        );
    }

    let runtime = VerylRuntime::new(VerylRuntimeOptions {
        call_veryl_build: env::var("RUNNING_TESTS_INDEPENDENTLY")
            .map(|value| &value == "1")
            .unwrap_or(false),
        ..Default::default()
    })?;

    let mut alu = runtime.create_model::<Alu32>()?;

    // assignment by struct
    alu = Alu32 {
        a: 0,
        b: 0,
        sub: 0,
        op: 0,
        ..alu
    };
    assert_eq!(alu.r, 0);

    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, 0);

    // test add
    alu = Alu32 {
        a: 100,
        op: 2,
        ..alu
    };

    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, 100);

    // assignment by field, both are possible
    alu.b = 100;
    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, 200);

    alu.sub = 1;
    alu.eval();
    dump(&alu);

    alu.b = 101;
    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, u32::MAX);

    // testing for SLT
    // 2 < 1
    alu.a = 2;
    alu.b = 1;
    alu.sub = 1;
    alu.op = 3; // N bit to bit 0
    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, 0);

    // 1 < 2
    alu.a = 1;
    alu.b = 2;
    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, 1);

    // 1 < 1
    alu.a = 1;
    alu.b = 1;
    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, 0);

    // -1 < 2
    alu.a = -1i32 as u32;
    alu.b = 2;
    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, 1);

    // -1 < -2
    alu.a = -1i32 as u32;
    alu.b = -2i32 as u32;
    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, 0);

    // -2 < -1
    alu.a = -2i32 as u32;
    alu.b = -1i32 as u32;
    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, 1);

    // -0x7000_0000 < 0
    alu.a = -0x7000_0000i32 as u32;
    alu.b = 0;
    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, 1);

    // -0x7000_0000 < 0x7000_0000
    // you need to take overflow into account
    alu.a = -0x7000_0000i32 as u32;
    alu.b = 0x7000_0000i32 as u32;
    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, 1);

    // 0x7000_0000 < -0x7000_0000
    // you need to take overflow into account
    alu.a = 0x7000_0000i32 as u32;
    alu.b = -0x7000_0000i32 as u32;
    alu.eval();
    dump(&alu);
    assert_eq!(alu.r, 0);

    Ok(())
}
