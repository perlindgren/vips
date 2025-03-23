import cocotb
from cocotb.triggers import Timer

@cocotb.test
async def test(dut):
    await Timer(1, units="ns")
    dut.a = 2147483648
    dut.b = 2147483648

    await Timer(1, units="ns")
    assert dut.sum.value == 0 and dut.carry.value == 1