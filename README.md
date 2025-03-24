# VIPS, MIPS32 based single-cycle model in Veryl-lang

## Dependencies

- Verilator, [install](https://verilator.org/guide/latest/install.html)
- Veryl, [install](https://veryl-lang.org/install/)
- Optional dependencies:
  - Surfer, [install](https://gitlab.com/surfer-project/surfer) (Stand alone wave viewer)
  - Marlin, [library](https://www.ethanuppal.com/marlin/) (used as a Rust library)

## Veryl Test, Simulation, and View

To test all modules:

```shell
veryl test --wave
surfer src/<module.vcd>
```

One can also test individual modules and their dependencies:
E.g., the `PcPlus4` module (which depends on `adder`, which depends on `full_adder`):

```shell
veryl test src/pc_plus4.veryl src/adder.veryl src/full_adder.veryl --wave
surfer src/pcplus4.vcd
```

For now verilator is broken (falsely reporting circular dependency). Remove the `--wave` and tests will pass, but you'll get no waveforms.

Once this work, Drag `TOP/test` to the view pane, and you should get:

![image](images/pc_plus4.png)

## Marlin Test

This allows us to use the Rust built in test framework. The module under test is represented by a struct which fields correspond to the module "interface".

```sv
// src/alu.veryl
module Alu #(
    param Width: u32 = 32,
) (
    a  : input  logic<Width>,
    b  : input  logic<Width>,
    sub: input  logic       ,
    op : input  logic<2>    ,
    r  : output logic<Width>,
    v  : output logic       ,
    c  : output logic       ,
    z  : output logic       ,
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

## Modules

### Alu

The Alu module, configured for 4 bit wide inputs:

![image](images/vips_alu.svg)

## List of current tests

For now using the explicit syntax for declaring dependencies.

```shell
veryl test src/mux.veryl --wave
veryl test src/half_adder.veryl --wave
veryl test src/pc_plus4.veryl src/adder.veryl src/full_adder.veryl --wave
veryl test src/decoder.veryl --wave
veryl test src/zero_extend.veryl --wave
veryl test src/regfile.veryl --wave
veryl test src/arith.veryl src/full_adder.veryl --wave
veryl test src/alu.veryl src/mux.veryl src/zero_extend.veryl src/arith.veryl src/full_adder.veryl 
```

Notice, the `--wave` option for `alu` test does not currently work.
