#include <stdio.h>
#include <string.h>
#include "../cpu/instructions.h"
#include "../stack/stack.h"
#include "program_memory.h"

int pc = 0;
uint32_t program_memory[MAXITEMS];

void print_memory(void) {
    for (int i = 0; i < pc; i++) {
        Instruction instruction = decode_instruction(program_memory[i]);
        Opcode opcode = instruction.opcode;
        int immediate = instruction.immediate;
        switch (opcode) {
            case halt:
                printf("%03d:\thalt\n", i);
                break;
            case pushc:
                printf("%03d:\tpushc   %d\n", i, immediate);
                break;
            case add:
                printf("%03d:\tadd\n", i);
                break;
            case sub:
                printf("%03d:\tsub\n", i);
                break;
            case mul:
                printf("%03d:\tmul\n", i);
                break;
            case divide:
                printf("%03d:\tdiv\n", i);
                break;
            case mod:
                printf("%03d:\tmod\n", i);
                break;
            case rdint:
                printf("%03d:\trdint\n", i);
                break;
            case wrint:
                printf("%03d:\twrint\n", i);
                break;
            case rdchr:
                printf("%03d:\trdchr\n", i);
                break;
            case wrchr:
                printf("%03d:\twrchr\n", i);
                break;
            default:
                break;
        }
    }
}

void register_instruction(Opcode opcode, int immediate) {
    uint32_t instruction = encode_instruction(opcode, immediate);
    program_memory[pc] = instruction;
    pc++;
}
