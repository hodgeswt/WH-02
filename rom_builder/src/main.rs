/*
    This file contains the code to build the control logic
    ROM for the WH-02 CPU. It creates an array of arrays, all storing
    the binary representation of the microcode instructions.

    This is indexed by [step][opcode], where step is the current
    step in the instruction cycle. The opcode is the current opcode
    corresponding to the instruction being executed.

    All operations begin with a fetch cycle of

    0x00: 0x0376 (PRGC -> MAR)
    0x01: 0x0789 (RAM -> INST + PRGC++)

    Which fetches the instruction from the RAM and stores it in the
    instruction register. Then, it increments the address register.
*/

use std::{collections::HashMap, time::Instant, fs::File, io::Write};

struct RomBuilder {
    size: usize,
    rom: Vec<u32>,
    output: String,
}

impl RomBuilder {
    pub fn new(size: usize, output: String) -> RomBuilder {
        RomBuilder {
            size,
            rom: vec![0; size], // Fill with NOP
            output,
        }
    }

    fn build(&mut self) {
        self.build_rom();

        let mut output = String::new();
        output += "v3.0 hex words addressed\n000: ";
        let mut counter = 0;
        let mut address = 0;
        for byte in self.rom.clone() {
            output += format!("{:08x} ", byte).as_str();
            counter += 1;
            address += 1;
            if counter % 16 == 0 && address < self.size {
                output += format!("\n{:03x}: ", address).as_str();
            }
        }

        let mut outfile = File::create(self.output.clone()).expect("Failed to create output file.");
        write!(outfile, "{}", output).expect("Faield to write to output file.");
    }

    fn build_rom(&mut self) {
        // Control signal definitions
        let reset_step_counter = 0x20000;
        let enable_program_counter = 0x400;
        let halt = 0x10000;

        // Opcode definitions
        let mov_a_b = 0x1;
        let mov_a_c = 0x2;
        let mov_a_o1 = 0x3;
        let mov_a_o2 = 0x4;
        let mov_a_ram = 0x5;
        let mov_b_a = 0x6;
        let mov_b_c = 0x7;
        let mov_b_o1 = 0x8;
        let mov_b_o2 = 0x9;
        let mov_b_ram = 0xA;
        let mov_c_a = 0xB;
        let mov_c_b = 0xC;
        let mov_c_o1 = 0xD;
        let mov_c_o2 = 0xE;
        let mov_c_ram = 0xF;
        let mov_o1_a = 0x10;
        let mov_o1_b = 0x11;
        let mov_o1_c = 0x12;
        let mov_o1_o2 = 0x13;
        let mov_o1_ram = 0x14;
        let mov_o2_a = 0x15;
        let mov_o2_b = 0x16;
        let mov_o2_c = 0x17;
        let mov_o2_o1 = 0x18;
        let mov_o2_ram = 0x19;
        let mov_ram_a = 0x1A;
        let mov_ram_b = 0x1B;
        let mov_ram_c = 0x1C;
        let mov_ram_o1 = 0x1D;
        let mov_ram_o2 = 0x1E;
        let mov_ram_ram = 0x1F;
        let hlt = 0x20;
        let mov_bus_a = 0x21;
        let mov_bus_b = 0x22;
        let mov_bus_c = 0x23;
        let mov_bus_o1 = 0x24;
        let mov_bus_o2 = 0x25;
        let mov_bus_ram = 0x26;
        let mov_acc_a = 0x27;
        let mov_acc_b = 0x28;
        let mov_acc_c = 0x29;
        let mov_acc_o1 = 0x2A;
        let mov_acc_o2 = 0x2B;
        let mov_acc_ram = 0x2C;

        // Defining microcode
        //
        // All steps begin with a fetch cycle
        for i in 0..0x100 {
            let step0 = self.get_address(0, i);
            self.rom[step0 as usize] = self.read_write("PRGC", "MAR");

            let step1 = self.get_address(1, i);
            self.rom[step1 as usize] = self.read_write("RAM", "INST") | enable_program_counter;
        }

        self.define(2, mov_a_b, self.read_write("A", "B"));
        self.define(3, mov_a_b, reset_step_counter);

        self.define(2, mov_a_c, self.read_write("A", "C"));
        self.define(3, mov_a_c, reset_step_counter);

        self.define(2, mov_a_o1, self.read_write("A", "O1"));
        self.define(3, mov_a_o1, reset_step_counter);

        self.define(2, mov_a_o2, self.read_write("A", "O2"));
        self.define(3, mov_a_o2, reset_step_counter);

        self.define(2, mov_a_ram, self.read_write("INST", "RAM"));
        self.define(3, mov_a_ram, self.read_write("A", "RAM"));
        self.define(4, mov_a_ram, reset_step_counter);

        self.define(2, mov_b_a, self.read_write("B", "A"));
        self.define(3, mov_b_a, reset_step_counter);

        self.define(2, mov_b_c, self.read_write("B", "C"));
        self.define(3, mov_b_c, reset_step_counter);

        self.define(2, mov_b_o1, self.read_write("B", "O1"));
        self.define(3, mov_b_o1, reset_step_counter);

        self.define(2, mov_b_o2, self.read_write("B", "O2"));
        self.define(3, mov_b_o2, reset_step_counter);

        self.define(2, mov_b_ram, self.read_write("INST", "RAM"));
        self.define(3, mov_b_ram, self.read_write("B", "RAM"));
        self.define(4, mov_b_ram, reset_step_counter);

        self.define(2, mov_c_a, self.read_write("C", "A"));
        self.define(3, mov_c_a, reset_step_counter);

        self.define(2, mov_c_b, self.read_write("C", "B"));
        self.define(3, mov_c_b, reset_step_counter);

        self.define(2, mov_c_o1, self.read_write("C", "O1"));
        self.define(3, mov_c_o1, reset_step_counter);

        self.define(2, mov_c_o2, self.read_write("C", "O2"));
        self.define(3, mov_c_o2, reset_step_counter);

        self.define(2, mov_c_ram, self.read_write("INST", "RAM"));
        self.define(3, mov_c_ram, self.read_write("C", "RAM"));
        self.define(4, mov_c_ram, reset_step_counter);

        self.define(2, mov_o1_a, self.read_write("O1", "A"));
        self.define(3, mov_o1_a, reset_step_counter);

        self.define(2, mov_o1_b, self.read_write("O1", "B"));
        self.define(3, mov_o1_b, reset_step_counter);

        self.define(2, mov_o1_c, self.read_write("O1", "C"));
        self.define(3, mov_o1_c, reset_step_counter);

        self.define(2, mov_o1_o2, self.read_write("O1", "O2"));
        self.define(3, mov_o1_o2, reset_step_counter);

        self.define(2, mov_o1_ram, self.read_write("INST", "RAM"));
        self.define(3, mov_o1_ram, self.read_write("O1", "RAM"));

        self.define(2, mov_o2_a, self.read_write("O2", "A"));
        self.define(3, mov_o2_a, reset_step_counter);

        self.define(2, mov_o2_b, self.read_write("O2", "B"));
        self.define(3, mov_o2_b, reset_step_counter);

        self.define(2, mov_o2_c, self.read_write("O2", "C"));
        self.define(3, mov_o2_c, reset_step_counter);

        self.define(2, mov_o2_o1, self.read_write("O2", "O1"));
        self.define(3, mov_o2_o1, reset_step_counter);

        self.define(2, mov_o2_ram, self.read_write("INST", "RAM"));
        self.define(3, mov_o2_ram, self.read_write("O2", "RAM"));
        self.define(4, mov_o2_ram, reset_step_counter);

        self.define(2, mov_ram_a, self.read_write("RAM", "MAR"));
        self.define(3, mov_ram_a, self.read_write("RAM", "A"));
        self.define(4, mov_ram_a, reset_step_counter);

        self.define(2, mov_ram_b, self.read_write("RAM", "MAR"));
        self.define(3, mov_ram_b, self.read_write("RAM", "B"));
        self.define(4, mov_ram_b, reset_step_counter);

        self.define(2, mov_ram_c, self.read_write("RAM", "MAR"));
        self.define(3, mov_ram_c, self.read_write("RAM", "C"));
        self.define(4, mov_ram_c, reset_step_counter);

        self.define(2, mov_ram_o1, self.read_write("RAM", "MAR"));
        self.define(3, mov_ram_o1, self.read_write("RAM", "O1"));
        self.define(4, mov_ram_o1, reset_step_counter);

        self.define(2, mov_ram_o2, self.read_write("RAM", "MAR"));
        self.define(3, mov_ram_o2, self.read_write("RAM", "O2"));
        self.define(4, mov_ram_o2, reset_step_counter);

        self.define(2, mov_ram_ram, self.read_write("RAM", "MAR"));
        self.define(3, mov_ram_ram, self.read_write("RAM", "STK") | enable_program_counter);
        self.define(4, mov_ram_ram, self.read_write("PRGC", "MAR"));
        self.define(5, mov_ram_ram, self.read_write("STK", "RAM") | enable_program_counter);
        self.define(6, mov_ram_ram, reset_step_counter);

        self.define(2, hlt, halt);

        self.define(2, mov_bus_a, self.read_write("PRGC", "MAR"));
        self.define(3, mov_bus_a, self.read_write("RAM", "A") | enable_program_counter);
        self.define(4, mov_bus_a, reset_step_counter);

        self.define(2, mov_bus_b, self.read_write("PRGC", "MAR"));
        self.define(3, mov_bus_b, self.read_write("RAM", "B") | enable_program_counter);
        self.define(4, mov_bus_b, reset_step_counter);

        self.define(2, mov_bus_c, self.read_write("PRGC", "MAR"));
        self.define(3, mov_bus_c, self.read_write("RAM", "C") | enable_program_counter);
        self.define(4, mov_bus_c, reset_step_counter);

        self.define(2, mov_bus_o1, self.read_write("PRGC", "MAR"));
        self.define(3, mov_bus_o1, self.read_write("RAM", "O1") | enable_program_counter);
        self.define(4, mov_bus_o1, reset_step_counter);

        self.define(2, mov_bus_o2, self.read_write("PRGC", "MAR"));
        self.define(3, mov_bus_o2, self.read_write("RAM", "O2") | enable_program_counter);
        self.define(4, mov_bus_o2, reset_step_counter);

        self.define(2, mov_bus_ram, self.read_write("PRGC", "MAR"));
        self.define(3, mov_bus_ram, self.read_write("RAM", "STK") | enable_program_counter);
        self.define(4, mov_bus_ram, self.read_write("PRGC", "MAR"));
        self.define(5, mov_bus_ram, self.read_write("STK", "RAM") | enable_program_counter);

        self.define(2, mov_acc_a, self.read_write("ACC", "A"));
        self.define(3, mov_acc_a, reset_step_counter);

        self.define(2, mov_acc_b, self.read_write("ACC", "B"));
        self.define(3, mov_acc_b, reset_step_counter);

        self.define(2, mov_acc_c, self.read_write("ACC", "C"));
        self.define(3, mov_acc_c, reset_step_counter);

        self.define(2, mov_acc_o1, self.read_write("ACC", "O1"));
        self.define(3, mov_acc_o1, reset_step_counter);

        self.define(2, mov_acc_o2, self.read_write("ACC", "O2"));
        self.define(3, mov_acc_o2, reset_step_counter);

        self.define(2, mov_acc_ram, self.read_write("PRGC", "MAR"));
        self.define(3, mov_acc_ram, self.read_write("RAM", "STK") | enable_program_counter);
        self.define(4, mov_acc_ram, self.read_write("PRGC", "MAR"));
        self.define(5, mov_acc_ram, self.read_write("STK", "RAM") | enable_program_counter);
        self.define(6, mov_acc_ram, reset_step_counter);
    }

    fn define(&mut self, step: u16, opcode: u16, val: u32) {
        let address = self.get_address(step, opcode);
        self.rom[address as usize] = val;
    }

    fn get_address(&self, step: u16, opcode: u16) -> u16 {
        // Addresses are of the form 0b000_0000_0000,
        // where the 3 MSB are the step and the 8 LSB
        // are the opcode. When given a step as a number,
        // We must shift it left 8 bits to make room for
        // the opcode.
        return (step << 8) | opcode;
    }

    fn read_write(&self, read: &str, write: &str) -> u32 {
        // Registers are selected via a decoder and an enable bit
        // for that decoder. This maps the register name to the bits
        // needed to output/input that register
        let output_selection: HashMap<&str, u32> = HashMap::from([
            ("A", 0x100),
            ("B", 0x101),
            ("C", 0x102),
            ("ACC", 0x103),
            ("O1", 0x104),
            ("O2", 0x105),
            ("PRGC", 0x106),
            ("MAR", 0x107),
            ("INST", 0x108),
            ("RAM", 0x109),
            ("STK", 0x10A),
        ]);

        let input_selection: HashMap<&str, u32> = HashMap::from([
            ("A", 0x200),
            ("B", 0x210),
            ("C", 0x220),
            ("O1", 0x230),
            ("O2", 0x240),
            ("FLG", 0x250),
            ("PRGC", 0x260),
            ("MAR", 0x270),
            ("INST", 0x280),
            ("RAM", 0x290),
            ("STK", 0x2A0),
        ]);

        return output_selection[read] | input_selection[write];
    }
}

fn main() {
    println!("Building ROM...");
    let start = Instant::now();
    let mut rom = RomBuilder::new(0x800, "rom.bin".to_string());
    rom.build();

    let duration = start.elapsed();
    println!("Completed in {}ms ({}ns)", duration.as_millis(), duration.as_nanos());
}
