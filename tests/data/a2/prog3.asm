//
// prog3.asm -- an assembler example with local variables
//

// local Integer x;
// local Integer y;
// x = 5;
// y = x + -3;
// x = 2 * y + x;
// writeInteger(x + -33);
// writeCharacter('\n');


    asf 2 
    pushc   5
    popl    0
    pushl   0
    pushc   -3
    add
    popl    1
    pushc   2
    pushl   1
    mul
    pushl   0
    add
    popl    0
    pushl   0
    pushc   -33
    add
    wrint
    pushc   '\n'
    wrchr   
    halt
