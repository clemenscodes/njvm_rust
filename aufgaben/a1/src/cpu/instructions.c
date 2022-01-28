#include "instructions.h"

uint32_t encode_instruction(Opcode opcode, int immediate) {
    return (opcode << 24) | IMMEDIATE(immediate);
}

Instruction decode_instruction(uint32_t bytecode) {
    Instruction instruction;
    instruction.opcode = bytecode >> 24;
    instruction.immediate = SIGN_EXTEND(IMMEDIATE(bytecode));
    return instruction;
}
