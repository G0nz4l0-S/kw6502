# kw6502 &ndash; A MOS 6502 emulator written in Rust

- [kw6502 &ndash; A MOS 6502 emulator written in Rust](#kw6502--a-mos-6502-emulator-written-in-rust)
  - [Introduction](#introduction)
  - [Inputing the program](#inputing-the-program)
  - [Notation](#notation)
    - [For the comments](#for-the-comments)
    - [For number bases](#for-number-bases)
    - [For the opcodes](#for-the-opcodes)
  - [Using the interactive prompt](#using-the-interactive-prompt)
  - [TODO](#todo)
  - [Known bugs](#known-bugs)
  - [References and further reading](#references-and-further-reading)
## Introduction

The kw6502 is a not yet cycle-accurate MOS 6502 CPU emulator written in Rust. Although the thread is not put to sleep due to inaccuracy issues, each instruction appropriately calls a clock tick function to keep track of every instruction's cycle consumption as per stated in the bibliography.

The kw6502 emulates the full set of legal opcodes, 151 in total. The unofficial opcodes are not yet supported.

## Inputing the program

At the moment, the only way to input a program is by using a plain-text hexdump-like file. The emulator does not care about the formatting of the file, however, if the first column contains addresses, **they will be ignored** and the program will begin at the default starting point (PC=$0600). In this case, the flag `addresses` (-a, --addresses) needs to be passed as a command line argument.

## Notation

### For the comments

Each function contains a brief description of what it does. The number of cycles it requires is specified between parenthesis.

### For number bases

- For binary numbers (base 2), Rust uses the `0b` prefix, while most 6502 assemblers use the `%` prefix. The later is used in the comments.
- For decimal numbers (base 10), no prefix or other indicator whatsoever is used.
- For hexadecimal numbers (base 16), Rust uses the `0x` prefix, while most 6502 assemblers use the `$` prefix. The latter one is used in the comments. The usage of uppercase and lowercase might be inconsistent in the code, but both of them are valid. Hexadecimal is the preferred output and memory access format for numbers. 

Example:

| Notation     |  Binary   | Decimal | Hexadecimal |
|--------------|-----------|---------|-------------|
| **Rust**     | 0b1001100 |      76 | 0x4C        |
| **Comments** | %1001100  |      76 | $4C         |

### For the opcodes

Opcodes ('operation codes') are stored as public `u8` constants of the the [P6502 type](src/p6502.rs). They are stored in the following format:
>INS_ + INSTRUCTION_ + ADDRESSING MODE

where *INS_* is a common prefix, *INSTRUCTION* is the mnemonic for the opcode and *ADDRESSING MODE* is the abbreviation of the addressing mode for the instruction. If no addressing mode is specified, then it is assumed to be Implicit. For example, `INS_LDA_IMM` represents the LDA instruction and the Immediate addressing mode. The abbreviations used are listed in the following table:


| Abbreviation |     Meaning      |
|--------------|------------------|
| ---          | Implicit         |
| ACC          | Accumulator      |
| IMM          | Immediate        |
| ZP0          | Zero Page        |
| ZPX          | Zero Page,X      |
| ZPY          | Zero Page,Y      |
| REL          | Relative         |
| ABS          | Absolute         |
| ABX          | Absolute,X       |
| ABY          | Absolute,Y       |
| IND          | Indirect         |
| IDX          | Indexed Indirect |
| IDY          | Indirect Indexed |

## Using the interactive prompt

Once a `BRK` instruction (opcode $00) is read, the program will terminate its execution and an interactive prompt will appear. In this prompt, simple commands regarding the processor's status and the memory can be run. The available commands are:

|         Command          |  Arguments  |                          Description                           |
|--------------------------|-------------|----------------------------------------------------------------|
| `memory` (short: `mem`)  | START [END] | Lists the contents of the specified memory locations.          |
| `status` (short: `stat`) | None        | Outputs the contents of the registers and the program counter. |
| `clear`                  | None        | Clears the screen.                                             |
| `exit` or `quit`         | None        | Terminates the prompt.                                         |

**Special note regarding the `memory` command**: even though both START and END are parsed as hexadecimal integers, a '$' prefix must not be used.   

## TODO
- More code reutilization (instructions which have different addressing modes).
- Fully implement clock cycle accuracy (add one cycle when a page is crossed, etc.).
- Design an assembler to more easily input code.
- Implement a debugging mode.
## Known bugs
- This emulation maintains the original 6502's indirect `JMP` bug. When the indirect vector begins at the end of a page (\$xxFF), the LSB is fetched from that address, however, the MSB is taken from the beginning of that page (\$xx00) rather than from the beginning of the next page. For more information, refer to the [6502.org's explanation of this bug](http://www.6502.org/tutorials/6502opcodes.html#JMP).
## References and further reading
1. [Obelisk.me.uk's](http://www.obelisk.me.uk) 6502 reference guide, including:
   - [Registers](http://www.obelisk.me.uk/6502/registers.html), which provides an overview of the processor's registers. 
   - [Instructions and reference](http://www.obelisk.me.uk/6502/instructions.html), which provides an explanation of what each instruction does, including the flags each one sets.
   - [Addressing modes](http://www.obelisk.me.uk/6502/addressing.html), which provides an explanation of how each addressing mode works.

2. [The Nesdev Wiki](https://wiki.nesdev.com/w/index.php?title=Nesdev_Wiki), which contains several articles regarding the 6502's functions and detailed explanations on how some instructions work.
3. [Mark Andrews' Atari Roots](https://www.atariarchives.org/roots/), a guide to Atari assembly language.
4. [Computerphile's YouTube channel](https://www.youtube.com/user/Computerphile), which explains, among other things, binary logic and computer hardware fundamentals.
5. [Skilldrick's 6502js](https://github.com/skilldrick/6502js), a JavaScript 6502 assembler and simulator in which 6502 code can be run and his [Easy 6502 ebook](https://skilldrick.github.io/easy6502/), which provides an introduction to the 6502 assembly programming. 
6. [The Rust Standard Library](https://doc.rust-lang.org/std/), specially [the reference for the `u8` data type](https://doc.rust-lang.org/std/primitive.u8.html) since it is the way a byte is represented in this emulation.