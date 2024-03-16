# AQA-ASM
A simple assembly simulator for the weird AQA A level instruction set. The simulation itself is 8 bit and the instructions all work with 8 bit unsigned integers. You are provided with 13 general purpose 8 bit registers labelled R0-R12 and 256 bytes of memory to load your instructions and store variables. Memory addresses start at zero, but bear in mind you will need to account for the space required by your instructions.
# Core Instruction Set
`<operand>` - either a register (Rn) or a literal value (#n)    
`Rd | <memory ref>` - either a register (Rn) or a memory address (n)
| Instruction | Description |
| ----------- | ----------- |
| `LDR Rd <memory ref>`  | Load the value in the memory location `<memory ref>` into register `d` |
| `STR Rd <memory ref>` | Store the value in register `d` into the memory location `<memory ref>` |
| `ADD Rd Rn <operand>` | Add the value specified by `<operand>` to the value in register `n` and store the result in register `d` |
| `SUB Rd Rn <operand>` | Subtract the value specified by `<operand>` from the value in register `n` and store the result in register `d` |
| `MOV Rd <operand>` | Copy the value specified by `<operand>` into register `d` |
| `B <label>` | Branch to the label specified by `<label>` |
| `B<condition> <label>` | Branch to the label specified by `<label>` if the last comparison met the critereon specified by `<condition`>. Condition can be one of: `EQ` (equal too), `NEQ` (not equal too), `GT` (greater than), `LT` (less than) |
| `AND Rd Rn <operand>` | Perform a bitwise AND between the value in register n and the value specified by `<operand>` and store in register `n` |
| `ORR Rd Rn <operand>` | Perform a bitwise OR between the value in register n and the value specified by `<operand>` and store in register `n` |
| `EOR Rd Rn <operand>` | Perform a bitwise XOR between the value in register n and the value specified by `<operand>` and store in register `n` |
| `MVN Rd <operand>` | Perform a bitwise NOT on the value specified by `<operand>` and store in register `n` |
| `LSL Rd Rn <operand>` | Perform a bitwise left shift on the value in register n by the number of bits specified by `<operand>` and store in register `d` |
| `LSR Rd Rn <operand>` | Perform a bitwise right shift on the value in register n by the number of bits specified by `<operand>` and store in register `d` |
| `HALT` | Terminate the program |
# Extra instructions
I have added extra instructions to make the program a bit less useless
| Instruction | Description |
| ----------- | ----------- |
| `PRINT Rd \| <memory ref>` | Print the numerical of value of the value stored at the register `d` or at the memory address specified by `<memory ref>` |
| `INPUT Rd \| <memory ref>` | Take a numerical input and store it in the register `d` or at the memory address specified by `<memory ref>` |
# Using
```
git clone https://github.com/Spacerulerwill/AQA-ASM
cd AQA-ASM
cargo run <filename>
```
# Contributions
This code is horrid and I hate it. Pull requests are welcome, I am quite new to rust.
