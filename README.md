# VIPS, MIPS32 based single-cycle model in Veryl-lang

## Dependencies

- Verilator, https://verilator.org/guide/latest/install.html
- Veryl, https://veryl-lang.org/install/
- Optional dependencies:
  - Surfer, https://gitlab.com/surfer-project/surfer (Stand alone wave viewer)
  - Marlin, https://www.ethanuppal.com/marlin/ (used as a Rust library)

## Veryl Test, Simulation, and View 

To test all modules:
```shell
veryl test --wave
surfer src/<module.vcd>
```

One can also test individual modules and their dependencies:
E.g., the `PcPlus4` module (which has no dependency):

```shell
veryl test src/pcplus4.veryl --wave
surfer src/pcplus4.vcd
```

Drag `TOP/test` to the view pane.
![image](images/pc_plus4.png)

## Marlin Test

This allows us to use the rust built in test framework. The module under test is represented by a struct which fields correspond to the module "interface".

```sv
// src/alu.veryl
module Alu (
    a  : input  logic<32>,
    b  : input  logic<32>,
    sub: input  logic    ,
    op : input  logic<2> ,
    r  : output logic<32>,
    v  : output logic    ,
    c  : output logic    ,
    z  : output logic    ,
)
```

```rust
// tests/veryl_tests.rs
...
#[veryl(src = "src/alu.veryl", name = "Alu")]
pub struct Alu;

#[test]
#[snafu::report]
fn test_alu() -> Result<(), Whatever> {
    ...

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
    assert_eq!(alu.r,0); 
    ...
```

To run tests and capture the output:

```shell
cargo test -- --nocapture
```

##