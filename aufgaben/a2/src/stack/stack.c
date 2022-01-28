#include <stdio.h>
#include <stdlib.h>
#include "stack.h"

int sp = 0;
int stack[MAXITEMS];

void print_stack(void) {
    printf("\n\tStack\n");
    printf(".-------+-------.\n");
    for (int i = sp; i >= 0; i--) {
        if (i == sp) {
            printf("|sp->%3d|%7d|\n", i, stack[i]);
        } else {
            printf("|%7d|%7d|\n", i, stack[i]);
        }
    }
    printf("'-------+-------'\n\n");
}

void push(int immediate) {
    if (sp > MAXITEMS) {
        printf("Stack overflow: Stack is full, not more than %d items allowed\n", MAXITEMS);
        exit(1);
    }
    stack[sp] = immediate;
    sp++;
}

int pop(void) {
    if ((sp == 0) && stack[sp] == 0) {
        printf("Stack underflow: popped from empty stack\n");
        exit(1);
    }
    sp--;
    int tmp = stack[sp];
    stack[sp] = 0;
    return tmp;
}
