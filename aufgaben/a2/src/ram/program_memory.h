#ifndef _PROGRAM_MEMORY_H
#define _PROGRAM_MEMORY_H

#include "../stack/stack.h"
#include "../cpu/instructions.h"

extern int pc;
extern uint32_t program_memory[MAXITEMS];
void print_memory(void);
void register_instruction(Opcode opcode, int immediate);

#endif
