#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "boot.h"
#include "../ram/program_memory.h"
#include "../cpu/control_unit.h"

void print_usage(void) {
    printf("usage: ./njvm [option] [option] ...\n");
    printf("  --prog1          select program 1 to execute\n");
    printf("  --prog2          select program 2 to execute\n");
    printf("  --prog2          select program 3 to execute\n");
    printf("  --version        show version and exit\n");
    printf("  --help           show this help and exit\n");
}

void print_version(void) {
    printf("Ninja Virtual Machine version 0 (compiled Sep 23 2015, 10:36:52\n");
}

void print_err(char *arg) {
    printf("unknown command line argument '%s', try './njvm --help'\n", arg);
    exit(1);
}

void program_1(void) {
    init();
    register_instruction(pushc, 3);
    register_instruction(pushc, 4);
    register_instruction(add, 0);
    register_instruction(pushc, 10);
    register_instruction(pushc, 6);
    register_instruction(sub, 0);
    register_instruction(mul, 0);
    register_instruction(wrint, 0);
    register_instruction(pushc, 10);
    register_instruction(wrchr, 0);
    register_instruction(halt, 0);
    print_memory();
    work();
}

void program_2(void) {
    init();
    register_instruction(pushc, -2);
    register_instruction(rdint, 0);
    register_instruction(mul, 0);
    register_instruction(pushc, 3);
    register_instruction(add, 0);
    register_instruction(wrint, 0);
    register_instruction(pushc, '\n');
    register_instruction(wrchr, 0);
    register_instruction(halt, 0);
    print_memory();
    work();
}

void program_3(void) {
    init();
    register_instruction(rdchr, 0);
    register_instruction(wrint, 0);
    register_instruction(pushc, '\n');
    register_instruction(wrchr, 0);
    register_instruction(halt, 0);
    print_memory();
    work();
}

int check_args(char *argv[]) {
    int help_result = strcmp(argv[1], "--help");
    int version_result = strcmp(argv[1], "--version");
    int prog1 = strcmp(argv[1], "--prog1");
    int prog2 = strcmp(argv[1], "--prog2");
    int prog3 = strcmp(argv[1], "--prog3");
    if (help_result == 0) {
        print_usage();
    } else if (version_result == 0) {
        print_version();
    } else if (prog1 == 0) {
        program_1();
    } else if (prog2 == 0) {
        program_2();
    } else if (prog3 == 0) { 
        program_3();
    } else {
        print_err(argv[1]);
    }
    return 0;
}