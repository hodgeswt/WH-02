'''
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
'''

# Build empty ROM
code = [[0x0 for i in range(0xff)] for j in range(0x8)]
rom = ['00000' for i in range(0b100000000000)]

# Dicts of bits to set for read/write operations
output_sel_bits = {
    'A': 0x100,
    'B': 0x101,
    'C': 0x102,
    'ACC': 0x103,
    'PRGC': 0x106,
    'MAR': 0x107,
    'INST': 0x108,
    'RAM': 0x109,
    'STK': 0x10A
}

input_sel_bits = {
    'A': 0x200,
    'B': 0x210,
    'C': 0x220,
    'ACC': 0x230,
    'O1': 0x240,
    'O2': 0x250,
    'PRGC': 0x260,
    'MAR': 0x270,
    'INST': 0x280,
    'RAM': 0x290,
    'STK': 0x2A0
}

# Takes a source/destination, and maps the bits to the proper
# spots for control signals for these bits
def rw_bits(read, write):
    return int(bin(output_sel_bits[read] | input_sel_bits[write]), 2)

RESET_STEPC = 0x20000
ENABLE_PRGC = 0x00400

##
## Map from opecodes to addresses
##

# Moving data from A register
MOV_A_B = 0x1
MOV_A_C = 0x2
MOV_A_O1 = 0x3
MOV_A_O2 = 0x4
MOV_A_RAM = 0x5
# Moving data from B register
MOV_B_A = 0x6
MOV_B_C = 0x7
MOV_B_O1 = 0x8
MOV_B_O2 = 0x9
MOV_B_RAM = 0xA
# Moving data from C register
MOV_C_A = 0xB
MOV_C_B = 0xC
MOV_C_O1 = 0xD
MOV_C_O2 = 0xE
MOV_C_RAM = 0xF
# Moving data from O1 register
MOV_O1_A = 0x10
MOV_O1_B = 0x11
MOV_O1_C = 0x12
MOV_O1_O2 = 0x13
MOV_O1_RAM = 0x14
# Moving data from O2 register
MOV_O2_A = 0x15
MOV_O2_B = 0x16
MOV_O2_C = 0x17
MOV_O2_O1 = 0x18
MOV_O2_RAM = 0x19
# Moving data from RAM
MOV_RAM_A = 0x1A
MOV_RAM_B = 0x1B
MOV_RAM_C = 0x1C
MOV_RAM_O1 = 0x1D
MOV_RAM_O2 = 0x1E
MOV_RAM_RAM = 0x1F
# HLT
HLT = 0x20
# Moving data from bus to register
MOV_BUS_A = 0x21
MOV_BUS_B = 0x22
MOV_BUS_C = 0x23
MOV_BUS_O1 = 0x24
MOV_BUS_O2 = 0x25
MOV_BUS_RAM = 0x26
# Moving from accumulator to destination
MOV_ACC_A = 0x27
MOV_ACC_B = 0x28
MOV_ACC_C = 0x29
MOV_ACC_O1 = 0x2A
MOV_ACC_O2 = 0x2B
MOV_ACC_RAM = 0x2C

##
## Assign microcode
##

code[0] = [rw_bits('PRGC', 'MAR') for _ in range(0xff)]
code[1] = [rw_bits('RAM', 'INST') | ENABLE_PRGC for _ in range(0xff)]

# MOV_A_B
code[2][MOV_A_B] = rw_bits('A','B')
code[3][MOV_A_B] = RESET_STEPC

# MOV_A_C
code[2][MOV_A_C] = rw_bits('A','C')
code[3][MOV_A_C] = RESET_STEPC

# MOV_A_O1
code[2][MOV_A_O1] = rw_bits('A','O1')
code[3][MOV_A_O1] = RESET_STEPC

# MOV_A_O2
code[2][MOV_A_O2] = rw_bits('A','O2')
code[3][MOV_A_O2] = RESET_STEPC

# MOV_A_RAM
# - these are different. We treat the last 8 bits
# of the instruction as raw data -- in these cases,
# an address to store in RAM.
code[2][MOV_A_RAM] = rw_bits('INST','MAR')
code[3][MOV_A_RAM] = rw_bits('A','RAM')
code[4][MOV_A_RAM] = RESET_STEPC

# MOV_B_A
code[2][MOV_B_A] = rw_bits('B','A')
code[4][MOV_B_A] = RESET_STEPC

# MOV_B_C
code[2][MOV_B_C] = rw_bits('B','C')
code[4][MOV_B_C] = RESET_STEPC

# MOV_B_O1
code[2][MOV_B_O1] = rw_bits('B','O1')
code[4][MOV_B_O1] = RESET_STEPC

# MOV_B_O2
code[2][MOV_B_O2] = rw_bits('B','O2')
code[4][MOV_B_O2] = RESET_STEPC

# MOV_B_RAM
code[2][MOV_B_RAM] = rw_bits('INST','MAR')
code[3][MOV_B_RAM] = rw_bits('B','RAM')
code[4][MOV_B_RAM] = RESET_STEPC

# MOV_C_A
code[2][MOV_C_A] = rw_bits('C','A')
code[3][MOV_C_A] = RESET_STEPC

# MOV_C_B
code[2][MOV_C_B] = rw_bits('C','B')
code[3][MOV_C_B] = RESET_STEPC

# MOV_C_O1
code[2][MOV_C_O1] = rw_bits('C','O1')
code[3][MOV_C_O1] = RESET_STEPC

# MOV_C_O2
code[2][MOV_C_O2] = rw_bits('C','O2')
code[3][MOV_C_O2] = RESET_STEPC

# MOV_C_RAM
code[2][MOV_C_RAM] = rw_bits('INST','MAR')
code[3][MOV_C_RAM] = rw_bits('C','RAM')
code[4][MOV_C_RAM] = RESET_STEPC

# MOV_RAM_A
code[2][MOV_RAM_A] = ENABLE_PRGC
code[3][MOV_RAM_A] = rw_bits('RAM','MAR')
code[4][MOV_RAM_A] = rw_bits('RAM','A')
code[5][MOV_RAM_A] = RESET_STEPC

# MOV_RAM_B
code[2][MOV_RAM_B] = ENABLE_PRGC
code[3][MOV_RAM_B] = rw_bits('RAM','MAR')
code[4][MOV_RAM_B] = rw_bits('RAM','B')
code[5][MOV_RAM_B] = RESET_STEPC

# MOV_RAM_C
code[2][MOV_RAM_C] = ENABLE_PRGC
code[3][MOV_RAM_C] = rw_bits('RAM','MAR')
code[4][MOV_RAM_C] = rw_bits('RAM','C')
code[5][MOV_RAM_C] = RESET_STEPC

# MOV_RAM_O1
code[2][MOV_RAM_O1] = ENABLE_PRGC
code[3][MOV_RAM_O1] = rw_bits('RAM','MAR')
code[4][MOV_RAM_O1] = rw_bits('RAM','O1')
code[5][MOV_RAM_O1] = RESET_STEPC

# MOV_RAM_O2
code[2][MOV_RAM_O2] = ENABLE_PRGC
code[3][MOV_RAM_O2] = rw_bits('RAM','MAR')
code[4][MOV_RAM_O2] = rw_bits('RAM','O2')
code[5][MOV_RAM_O2] = RESET_STEPC

# MOV_RAM_RAM
code[2][MOV_RAM_RAM] = ENABLE_PRGC
code[3][MOV_RAM_RAM] = rw_bits('RAM','MAR')
code[4][MOV_RAM_RAM] = rw_bits('RAM','STK')
code[5][MOV_RAM_RAM] = ENABLE_PRGC
code[6][MOV_RAM_RAM] = rw_bits('RAM','MAR')
code[7][MOV_RAM_RAM] = rw_bits('STK','RAM')

# HLT
code[2][HLT] = 0x10000

# MOV_BUS_A
code[2][MOV_BUS_A] = rw_bits('PRGC','MAR')
code[3][MOV_BUS_A] = rw_bits('RAM','A')
code[4][MOV_BUS_A] = RESET_STEPC

# MOV_BUS_B
code[2][MOV_BUS_A] = rw_bits('PRGC','MAR')
code[3][MOV_BUS_B] = rw_bits('RAM','B')
code[4][MOV_BUS_B] = RESET_STEPC

# MOV_BUS_C
code[2][MOV_BUS_A] = rw_bits('PRGC','MAR')
code[3][MOV_BUS_C] = rw_bits('RAM','C')
code[4][MOV_BUS_C] = RESET_STEPC

# MOV_BUS_O1
code[2][MOV_BUS_A] = rw_bits('PRGC','MAR')
code[3][MOV_BUS_O1] = rw_bits('RAM','O1')
code[4][MOV_BUS_O1] = RESET_STEPC

# MOV_BUS_O2
code[2][MOV_BUS_A] = rw_bits('PRGC','MAR')
code[3][MOV_BUS_O2] = rw_bits('RAM','O2')
code[4][MOV_BUS_O2] = RESET_STEPC

# MOV_ACC_A
code[2][MOV_ACC_A] = rw_bits('ACC','A')
code[3][MOV_ACC_A] = RESET_STEPC

# MOV_ACC_B
code[2][MOV_ACC_B] = rw_bits('ACC','B')
code[3][MOV_ACC_B] = RESET_STEPC

# MOV_ACC_C
code[2][MOV_ACC_C] = rw_bits('ACC','C')
code[3][MOV_ACC_C] = RESET_STEPC

# MOV_ACC_O1
code[2][MOV_ACC_O1] = rw_bits('ACC','O1')
code[3][MOV_ACC_O1] = RESET_STEPC

# MOV_ACC_O2
code[2][MOV_ACC_O2] = rw_bits('ACC','O2')
code[3][MOV_ACC_O2] = RESET_STEPC

# MOV_ACC_RAM
code[2][MOV_ACC_RAM] = rw_bits('INST','MAR')
code[3][MOV_ACC_RAM] = rw_bits('ACC','RAM')
code[4][MOV_ACC_RAM] = RESET_STEPC

def to_rom_index(i, j):
    return f"{bin(i)[2:].rjust(3,'0')}{bin(j)[2:].rjust(8,'0')}"

# turn code into a block of hex code,
# indexed such that the 3 MSB are the step
# and the 8 LSB are the opcode
for i in range(0x8):
    for j in range(0xff):
        rom[int(to_rom_index(i,j), 2)] = hex(code[i][j])[2:].rjust(5,'0')

# Convert to string
s = "v3.0 hex words addressed\n000: "
c = 0
for i in rom:
    s += i + " "
    c += 1
    if (c % 15 == 0):
        s += f"\n{hex(c)[2:].rjust(3,'0')}: "
# Write to file
f = open("rom.bin", 'w')
f.write(s)
f.close()