#ifndef _STACK_H
#define _STACK_H

#define MAXITEMS 100

extern int sp;
extern int stack[MAXITEMS];

void print_stack(void);
void push(int immediate);
int pop(void);

#endif
