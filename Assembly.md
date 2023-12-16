# WH-02 Assembly Language

## Introduction

Operations consist of a 3-letter word and one to two operands. Operands are separated by a comma.

An operand can be:
1. A register.

These operands are mapped in this table.

| Location             | Name |
| -------------------- | ---- |
| A Register           | @A   |
| B Register           | @B   |
| C Register           | @C   |
| Output Register 1    | @O1  |
| Output Register 2    | @O2  |

Registers can be both source and destination operands

1. A memory address.

Memory addresses are denoted by a `$` followed by a four-hexit number. These operands can be sources or destinations.

2. A hexadecimal value

Hex values are denoted by a `#` followed by a four-hexit number. These operands can only be source operands.

## MOV
MOV takes two arguments. The first is the source, the second is the destination.

A source can be either a register, a value, or a RAM address.
    - If the source is a register, the value stored in that register is moved to the destination
    - If the source is a value, that value is moved to the destination
    - If the source is a memory address, the value at that address is moved to the destination
A destination can either be a register or a RAM address.
    - If the destination is a register, source value is moved to that register
    - If the destination is a memory address, the source value is moved to that address

## CMP
CMP takes two arguments, both values to compare. If the argument is a register, the value in that register is used. If the argument is a memory address, the value at that address is used. If the argument is a value, that value is used.

This operation latches the ALU flags, allowing for conditional jumps.

## BNE
BNE takes one argument, a memory address. If the ALU flags are not equal, the program counter is set to the value at that address. If the operand is a register, the program counter is set to the value in that register.

## BEQ
BEQ takes one argument, a memory address. If the ALU flags are equal, the program counter is set to the value at that address. If the operand is a register, the program counter is set to the value in that register.

## JMP
JMP is an unconditional jump. It takes one argument, a memory address. The program counter is set to the value at that address. If the operand is a register, the program counter is set to the value in that register.

## NOP
No operation

## HLT
Halts the processor

## START
Takes an address as its operand, and defines where the program will be loaded into memory. This must be the first instruction in the program.