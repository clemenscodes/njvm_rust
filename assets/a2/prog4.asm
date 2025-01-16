//
// prog4.asm -- an assembler example with local variables
//


// local integer w
// local integer x
// local integer y
// local integer z
// w = -6
// x = 2
// y = 4
// z = w + x + y
// writeInteger(z)
// writeCharacter('\n')


    asf 4
    pushc   -6
    popl    0
    pushc   2
    popl    1
    pushc   4
    popl    2
    pushl   0
    pushl   1
    pushl   2
    add
    add
    popl    3
    pushl   3
    wrint
    pushc	'\n'
    wrchr
    rsf
    halt
