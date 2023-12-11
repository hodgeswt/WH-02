'''
    This file contains the code to build the control logic
    ROM for the WH-02 CPU. It creates an array of arrays, all storing
    the binary representation of the microcode instructions.

    This is indexed by [opcode][step], where step is the current
    step in the instruction cycle. The opcode is the current opcode
    corresponding to the instruction being executed.

    All operations begin with a fetch cycle of

    0x00: 0x0376
    0x01: 0x0798

    Which fetches the instruction from the RAM and stores it in the
    instruction register. Then, it increments the address register.
'''

# Build empty ROM
code = [[0x0 for i in range(0b1000)] for j in range(0b100000)]

# Dicts of bits to set for read/write operations
output_sel_bits = {
    'A': 0x100,
    'B': 0x101,
    'C': 0x102,
    'ACC': 0x103,
    'PRGC': 0x106,
    'MAR': 0x107,
    'INST': 0x108,
    'RAM': 0x109
}

input_sel_bits = {
    'A': 0x200,
    'B': 0x210,
    'C': 0x220,
    'ACC': 0x230,
    'PRGC': 0x260,
    'MAR': 0x270,
    'INST': 0x280,
    'RAM': 0x290
}

# Takes a source/destination, and maps the bits to the proper
# spots for control signals for these bits
def rw_bits(read, write):
    return hex(int(bin(input_sel_bits[read] | output_sel_bits[write]), 2))

# Remaining bits
RESET_STEPC = 0x20000

def set_remaining_to_zero(opcode, step):
    for i in range(step, 0b1000):
        code[opcode][i] = 0x0

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

##
## Assign microcode
##

for i in code:
    # For each opcode, set the fetch cycle
    i[0] = 0x0376
    i[1] = 0x0798
    # and zero out the rest (NOP)
    for j in range(2, 0b1000):
        i[j] = 0x0

# MOV_A_B
code[MOV_A_B][2] = rw_bits('A','B')
code[MOV_A_B][3] = RESET_STEPC

# MOV_A_C
code[MOV_A_C][2] = rw_bits('A','C')
code[MOV_A_C][3] = RESET_STEPC

# MOV_A_O1
code[MOV_A_O1][2] = rw_bits('A','O1')
code[MOV_A_O1][3] = RESET_STEPC

# MOV_A_O2
code[MOV_A_O2][2] = rw_bits('A','O2')
code[MOV_A_O2][3] = RESET_STEPC

# MOV_A_RAM
# - these are different. We treat the last 8 bits
# of the instruction as raw data -- in these cases,
# an address to store in RAM.
code[MOV_A_RAM][2] = rw_bits('INST','MAR')
code[MOV_A_RAM][3] = rw_bits('A','RAM')
code[MOV_A_RAM][4] = RESET_STEPC

# MOV_B_A
code[MOV_B_A][2] = rw_bits('B','A')
code[MOV_B_A][3] = RESET_STEPC

# MOV_B_C
code[MOV_B_C][2] = rw_bits('B','C')
code[MOV_B_C][3] = RESET_STEPC

# MOV_B_O1
code[MOV_B_O1][2] = rw_bits('B','O1')
code[MOV_B_O1][3] = RESET_STEPC

# MOV_B_O2
code[MOV_B_O2][2] = rw_bits('B','O2')
code[MOV_B_O2][3] = RESET_STEPC

# MOV_B_RAM
code[MOV_A_RAM][2] = rw_bits('INST','MAR')
code[MOV_A_RAM][3] = rw_bits('B','RAM')
code[MOV_A_RAM][2] = RESET_STEPC

# MOV_C_A
code[MOV_C_A][2] = rw_bits('C','A')
code[MOV_C_A][3] = RESET_STEPC

# MOV_C_B
code[MOV_C_B][2] = rw_bits('C','B')
code[MOV_C_B][3] = RESET_STEPC

# MOV_C_O1
code[MOV_C_O1][2] = rw_bits('C','O1')
code[MOV_C_O1][3] = RESET_STEPC

# MOV_C_O2
code[MOV_C_O2][2] = rw_bits('C','O2')
code[MOV_C_O2][3] = RESET_STEPC

# MOV_C_RAM
code[MOV_A_RAM][2] = rw_bits('INST','MAR')
code[MOV_A_RAM][3] = rw_bits('C','RAM')
code[MOV_A_RAM][2] = RESET_STEPC
