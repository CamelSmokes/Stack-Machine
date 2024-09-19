# Stack Machine
A proof of concept stack based virtual machine operating off of 8bit opcodes. 

## Structure
The machine consists of

 - A 64 bit word size
 - A stack of 64 words
 - 256 Instructions (not all implemented)

## Assembler
Though not directly part of the machine, an assembler is provided in this repository to make programming for the machine bearable.
The assembler also has a preprocessor which adds some extra functionality (like the INC to increment).
## Usage

To use the VM you must first have a binary file to run on it.
You can obtain one of these files by assembling one of the example `.xasm` assembly files.
```shell
stack_machine -A prime.xasm out.hex
```
Then the assembled bytecode can be run with
```shell
stack_machine -R out.hex
```

## Documentation
There is no documentation.
For a list of Opcodes, see `opcode.rs` and for the implementation of those opcodes, see `instruction.rs`. 