# WH-02

This is a simple 8-bit processor designed in logisim_evolution.
It is a successor to my [WH-01](https://github.com/hodgeswt/WH-01) processor.

It is currently incomplete.

A breakdown of the instructions is available in Assembly.md

# Build and Run

- Install [Logisim-evolution](https://github.com/logisim-evolution/logisim-evolution)
- Clone the repo and open `WH-02.circ` in Logisim-evolution
- Navigate to the ROM in the circuit and right click. Load the image "rom.bin" from the rom_builder folder.
- Naviagate to the RAM in the circuit and double click to expand it. Right click on the RAM and load your desire
bin. Bins can be built from the programs in the programs folder.
- Under "Simulate", set desired auto-tick speed and enable auto-tick.
- The "Reset1" button will reset the processor and start the program.


# Assemble programs

- Ensure Rust is installed
- Navigate to the assembler folder
- Run `cargo run --release ~/path/to/program.asm ~/path/to/output/program.bin`