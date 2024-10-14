![Build](https://github.com/Spacerulerwill/AQA-ASM/actions/workflows/build.yml/badge.svg)
![Tests](https://github.com/Spacerulerwill/AQA-ASM/actions/workflows/tests.yml/badge.svg)
[![codecov](https://codecov.io/gh/Spacerulerwill/AQA-ASM/graph/badge.svg?token=XXWP13W957)](https://codecov.io/gh/Spacerulerwill/AQA-ASM)


# AQA-ASM

AQA-ASM is a simple assembly simulator designed for the weird AQA A Level instruction set. This simulator operates on 8-bit unsigned integers and features 13 general-purpose 8-bit registers labeled R0 to R12, along with 256 bytes of memory for loading instructions and storing variables.

## Memory Layout

- Memory addresses are zero-indexed.
- The first byte after your program instructions is where the memory begins.
- For a program that occupies `n` bytes, you will have `256 - n` memory addresses available for use.

### Comments

You can include comments in your assembly code for clarity:

- **Line comments** begin with `//`
- **Block comments** are enclosed between `/*` and `*/`

## Core Instruction Set

The following table outlines the core instruction set available in AQA-ASM. Instructions can be delimited by either a semicolon (`;`) or a newline character, while instruction arguments are separated by commas.

| Instruction          | Description                                                                 |
|---------------------|-----------------------------------------------------------------------------|
| `LDR Rd, <memory ref>`  | Load the value in the memory location `<memory ref>` into register `d`    |
| `STR Rd, <memory ref>`  | Store the value in register `d` into the memory location `<memory ref>`   |
| `ADD Rd, Rn, <operand>` | Add the value specified by `<operand>` to the value in register `n` and store the result in register `d` |
| `SUB Rd, Rn, <operand>` | Subtract the value specified by `<operand>` from the value in register `n` and store the result in register `d` |
| `CMP Rn, <operand>`     | Compare the value stored in register `n` with the value specified by `<operand>` |
| `MOV Rd, <operand>`     | Copy the value specified by `<operand>` into register `d`                 |
| `B <label>`             | Branch to the label specified by `<label>`                                 |
| `B<condition> <label>`  | Branch to the label specified by `<label>` if the last comparison met the criterion specified by `<condition>`. Condition can be one of: `EQ` (equal to), `NE` (not equal to), `GT` (greater than), `LT` (less than) |
| `AND Rd, Rn, <operand>` | Perform a bitwise AND between the value in register `n` and the value specified by `<operand>`, storing the result in register `d` |
| `ORR Rd, Rn, <operand>` | Perform a bitwise OR between the value in register `n` and the value specified by `<operand>`, storing the result in register `d` |
| `EOR Rd, Rn, <operand>` | Perform a bitwise XOR between the value in register `n` and the value specified by `<operand>`, storing the result in register `d` |
| `MVN Rd, <operand>`     | Perform a bitwise NOT on the value specified by `<operand>` and store it in register `n` |
| `LSL Rd, Rn, <operand>` | Perform a bitwise left shift on the value in register `n` by the number of bits specified by `<operand>`, storing the result in register `d` |
| `LSR Rd, Rn, <operand>` | Perform a bitwise right shift on the value in register `n` by the number of bits specified by `<operand>`, storing the result in register `d` |
| `HALT`                  | Terminate the program                                                      |

## Extra Instructions

In addition to the core instructions, the following extra instructions make it so programs can actually be debugged

| Instruction                | Description                                                                    |
|----------------------------|--------------------------------------------------------------------------------|
| `PRINT Rd \| <memory ref>`   | Print the numerical value stored in register `d` or at the memory address specified by `<memory ref>` |
| `INPUT Rd \| <memory ref>`   | Take numerical input and store it in register `d` or at the memory address specified by `<memory ref>` |

## Usage

To use AQA-ASM, follow these steps:

```bash
git clone https://github.com/Spacerulerwill/AQA-ASM
cd AQA-ASM
cargo run <filename>
```
This will compile and run your assembly program