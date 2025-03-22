# MIPS32 based single-cycle model in Veryl-lang

## Dependencies

- Verilator, https://verilator.org/guide/latest/install.html
- Veryl, https://veryl-lang.org/install/
- Surfer, https://gitlab.com/surfer-project/surfer

## Test & simulation 

```shell
veryl test --wave
surfer src/<module.vcd>
```