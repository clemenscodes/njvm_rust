(PROG1
  (XCHGPNAME (QUOTE FALSE) (QUOTE NIL))
  (PURGENAME (QUOTE FALSE))

  (XCHGPNAME (QUOTE TRUE) (QUOTE T))
  (PURGENAME (QUOTE TRUE))

  (XCHGPNAME (QUOTE MACHINEFUNCTION) (QUOTE SUBR))
  (PURGENAME (QUOTE MACHINEFUNCTION))
  (PURGENAME (QUOTE SUBR))

  (XCHGPNAME (QUOTE MACHINESUBROUTINE) (QUOTE NSUBR))
  (PURGENAME (QUOTE MACHINESUBROUTINE))
  (PURGENAME (QUOTE NSUBR))

  (XCHGPNAME (QUOTE FUNCTION) (QUOTE LAMBDA))
  (PURGENAME (QUOTE FUNCTION))

  (XCHGPNAME (QUOTE SUBROUTINE) (QUOTE NLAMBDA))
  (PURGENAME (QUOTE SUBROUTINE))

  (XCHGPNAME (QUOTE SCAN) (QUOTE RATOM))
  (PURGENAME (QUOTE SCAN))

  (XCHGPNAME (QUOTE READCHAR) (QUOTE READCH))
  (PURGENAME (QUOTE READCHAR))

  (XCHGPNAME (QUOTE PRINT) (QUOTE PRIN1))
  (XCHGPNAME (QUOTE PRINTLINE) (QUOTE PRINT))
  (PURGENAME (QUOTE PRINTLINE))

  (XCHGPNAME (QUOTE FIRST) (QUOTE CAR))
  (PURGENAME (QUOTE FIRST))

  (XCHGPNAME (QUOTE REST) (QUOTE CDR))
  (PURGENAME (QUOTE REST))

  (XCHGPNAME (QUOTE SECOND) (QUOTE CADR))
  (PURGENAME (QUOTE SECOND))

  (XCHGPNAME (QUOTE RREST) (QUOTE CDDR))
  (PURGENAME (QUOTE RREST))

  (XCHGPNAME (QUOTE THIRD) (QUOTE CADDR))
  (PURGENAME (QUOTE THIRD))

  (XCHGPNAME (QUOTE RRREST) (QUOTE CDDDR))
  (PURGENAME (QUOTE RRREST))

  (PURGENAME (QUOTE CAAR))
  (PURGENAME (QUOTE CDAR))
  (PURGENAME (QUOTE CAAAR))
  (PURGENAME (QUOTE CDAAR))
  (PURGENAME (QUOTE CADAR))
  (PURGENAME (QUOTE CDDAR))
  (PURGENAME (QUOTE CAADR))
  (PURGENAME (QUOTE CDADR))

  (XCHGPNAME (QUOTE ADJOIN) (QUOTE CONS))
  (PURGENAME (QUOTE ADJOIN))

  (XCHGPNAME (QUOTE REPLACEF) (QUOTE RPLACA))
  (PURGENAME (QUOTE REPLACEF))

  (XCHGPNAME (QUOTE REPLACER) (QUOTE RPLACD))
  (PURGENAME (QUOTE REPLACER))

  (XCHGPNAME (QUOTE CONCATEN) (QUOTE NCONC))
  (PURGENAME (QUOTE CONCATEN))

  (XCHGPNAME (QUOTE ASSIGN) (QUOTE SET))
  (PURGENAME (QUOTE ASSIGN))

  (PURGENAME (QUOTE REMPROP))
  (PURGENAME (QUOTE FLAGP))
  (PURGENAME (QUOTE FLAG))
  (PURGENAME (QUOTE REMFLAG))

  (XCHGPNAME (QUOTE COMPRESS) (QUOTE PACK))
  (PURGENAME (QUOTE COMPRESS))

  (XCHGPNAME (QUOTE EXPLODE) (QUOTE UNPACK))
  (PURGENAME (QUOTE EXPLODE))

  (XCHGPNAME (QUOTE EMPTY) (QUOTE NULL))
  (PURGENAME (QUOTE EMPTY))

  (XCHGPNAME (QUOTE =) (QUOTE EQUAL))
  (PURGENAME (QUOTE =))

  (XCHGPNAME (QUOTE NEWLINE) (QUOTE TERPRI))
  (PURGENAME (QUOTE NEWLINE))

  (XCHGPNAME (QUOTE INTEGER) (QUOTE NUMBERP))
  (PURGENAME (QUOTE INTEGER))

  (XCHGPNAME (QUOTE GREATER) (QUOTE GREATERP))
  (PURGENAME (QUOTE GREATER))

  (XCHGPNAME (QUOTE LESSER) (QUOTE LESSP))
  (PURGENAME (QUOTE LESSER))

  (XCHGPNAME (QUOTE POSITIVE) (QUOTE PLUSP))
  (PURGENAME (QUOTE POSITIVE))

  (XCHGPNAME (QUOTE NEGATIVE) (QUOTE MINUSP))
  (PURGENAME (QUOTE NEGATIVE))

  (XCHGPNAME (QUOTE ZERO) (QUOTE ZEROP))
  (PURGENAME (QUOTE ZERO))

  (XCHGPNAME (QUOTE MOD) (QUOTE REMAINDER))
  (PURGENAME (QUOTE MOD))

  (XCHGPNAME (QUOTE ') (QUOTE QUOTE))
  (PURGENAME (QUOTE '))

  (PURGENAME (QUOTE PROG1))

  (XCHGPNAME (QUOTE :) (QUOTE SETQ))
  (PURGENAME (QUOTE :))

  (PURGENAME (QUOTE XCHGPNAME))
  (PURGENAME (QUOTE PURGENAME))
)

% -------------------------------------------- %

(RDS PARSE MUS)

(: RPAR (' ")"))
(: LPAR (' "("))
(: COMMA (' ","))
(: BELL TRUE)
(: NEWLINE 1)
(: PRTMATH (' PRTMATH))

(PUTD (' ECHO)
      (' (FUNCTION () (OR (NOT RDS) ECHO))) )

(PUTD (' MAKDEF)
      (' (FUNCTION (EX1 EX2 LEX1)
           ((= (REST (GETD EX1)) LEX1) EX1)
           (((AND (GETD EX1) (ECHO))
             (PRINT " *** REDEFINED: ")
             (PRINTLINE EX1) ))
           (PUTD EX1 (ADJOIN EX2 LEX1))
           EX1 )) )

(PUTD (' FUNCTION)
      (' (SUBROUTINE LEX1
           (MAKDEF (FIRST LEX1) (' FUNCTION) (REST LEX1)) )) )

(PUTD (' SUBROUTINE)
      (' (SUBROUTINE LEX1
           (MAKDEF (FIRST LEX1) (' SUBROUTINE) (REST LEX1)) )) )

% -------------------------------------------- %

(SUBROUTINE PUTPROP (EX1 EX2 EX3)
  ((OR
      (= (GET EX1 EX2) EX3)
      (= (GETD (GET EX1 EX2)) EX3) ) EX1)
  ( ((NOT (GET EX1 EX2)))
    ((EQ SCAN $))
    (PRINT " *** REDEFINED: ") )
  ((EQ (FIRST EX3) (' FUNCTION))
    (PUTD (PUT EX1 EX2 (COMPRESS (LIST EX1 EX2))) EX3)
    EX1 )
  (PUT EX1 EX2 EX3)
  EX1 )

% -------------------------------------------- %

(: DELIMITER (' (EXIT ENDLOOP ENDBLOCK ENDFUN ENDSUB ")" ",")))

(FUNCTION DELIMITER ()
  (OR
    (TERMINATOR)
    (MEMBER SCAN DELIMITER) ) )

(FUNCTION TERMINATOR ()
  (OR
    (EQ SCAN ;)
    (EQ SCAN $)
    (EQ SCAN &) ) )

(FUNCTION MATCH (DELIM LEX1)
  (LOOP
    ( ((EQ SCAN COMMA)
        (SCAN) ) )
    ((EQ SCAN DELIM)
      (SCAN)
      (REVERSE LEX1) )
    ((DELIMITER)
      (SYNTAX DELIM "NOT FOUND") )
    (PUSH (PARSE SCAN 0) LEX1) ) )

(PUTPROP INFIX "(" (COND
  ((NAME EX1)
    (ADJOIN EX1 (MATCH RPAR)) )
  ((SYNTAX)) ))

(PUTPROP INFIX : (COND
  ((NAME EX1)
    (LIST : EX1 (PARSE SCAN 20)) )
  ((SYNTAX)) ))

(PUTPROP LBP "(" 200)

(FUNCTION MATCHNOP (EX1 DELIM)
  ((EQ SCAN DELIM)
    (SCAN)
    EX1 )
  (SYNTAX DELIM "NOT FOUND") )

(PUTPROP PREFIX "(" (MATCHNOP (PARSE EX2 0) RPAR))

(FUNCTION PARSE (EX1 RBP EX2)
  ((DELIMITER)
    (SYNTAX EX1 "USED AS" NAME) )
  (: EX2 (SCAN))
  ( ((GET (' PREFIX) EX1)
      (: EX1 (EVAL (GET (' PREFIX) EX1))) )
    ((GET (' LBP) EX1)
      (SYNTAX EX1 "USED AS" PREFIX OPERATOR) )
    ((GET (' RBP) EX1)
      (: EX1 (LIST EX1 (PARSE EX2 (GET (' RBP) EX1)))) ) )
  (LOOP
    (: EX2 SCAN)
    ((NOT (LESSER RBP (GET (' LBP) EX2))) EX1)
    (SCAN)
    ( ((GET (' INFIX) EX2)
        (: EX1 (EVAL (GET (' INFIX) EX2))) )
      ((GET (' RBP) EX2)
        ((GET (' LBP) EX2)
          (: EX1 (LIST EX2 EX1 (PARSE SCAN (GET (' RBP) EX2)))) )
        (SYNTAX EX2 "USED AS" INFIX OPERATOR) )
      ((GET (' LBP) EX2)
        (: EX1 (LIST EX2 EX1)) )
      (SYNTAX EX2 "USED AS" INFIX OPERATOR) ) ) )

(FUNCTION READLIST (EX1)
  ((EQ EX1 LPAR)
    (LOOP
      ((NOT (EQ (SCAN) COMMA))) )
    ((EQ SCAN RPAR)
      (SCAN)
      FALSE )
    (READREST SCAN) )
  ((EQ EX1 RPAR)
    (SYNTAX) )
  (SCAN)
  ((EQ EX1 COMMA)
    (READLIST SCAN) )
  EX1 )

(PUTPROP PREFIX ' (LIST (' ') (READLIST SCAN)))

(FUNCTION READREST (EX1)
  ((EQ EX1 RPAR)
    (SCAN)
    FALSE )
  ((EQ EX1 ".")
    (: EX1 (READLIST (SCAN)))
    ((EQ SCAN RPAR)
      (SCAN)
      EX1 )
    (SYNTAX) )
  (ADJOIN (READLIST EX1) (READREST SCAN)) )

(FUNCTION PUTPARSE (EX1 EX2)
  ((AND
      (NAME EX1)
      (NAME EX2) )
    (LOOP
      ((NOT (EQ SCAN COMMA)))
      (SCAN) )
    (LIST (' PUTPROP) EX1 EX2 (PARSE SCAN 0)) )
  (SYNTAX) )

(PUTPROP PREFIX PROPERTY (PUTPARSE (READLIST SCAN) (READLIST SCAN)))

(FUNCTION DEFFUN (EX1 EX2 EX3)
  ((AND
      (NAME EX1)
      EX1 )
    (ADJOIN EX2 (ADJOIN EX1 (ADJOIN (READLIST SCAN) (MATCH EX3)))) )
  (ADJOIN EX2 (ADJOIN EX1 (MATCH EX3))) )

(PUTPROP PREFIX FUNCTION (DEFFUN (READLIST SCAN) (' FUNCTION) (' ENDFUN)))

(PUTPROP PREFIX SUBROUTINE (DEFFUN (READLIST SCAN) (' SUBROUTINE) (' ENDSUB)))

(FUNCTION IDENTITY (EX1)
  EX1 )

(FUNCTION WHENPARSE (EX1)
  ((ATOM EX1)
    (ADJOIN (LIST (' IDENTITY) EX1) (MATCH (' EXIT))) )
  (ADJOIN EX1 (MATCH (' EXIT))) )

(PUTPROP PREFIX WHEN (WHENPARSE (PARSE SCAN 0)))

(PUTPROP PREFIX BLOCK (COND
  ((EQ SCAN (' WHEN))
    (ADJOIN (WHENPARSE (PARSE (SCAN) 0)) (MATCH (' ENDBLOCK))) )
  ((SYNTAX)) ))

(PUTPROP PREFIX LOOP (ADJOIN (' LOOP) (MATCH (' ENDLOOP))))

(FUNCTION DRIVER (EX1 EX2)
  (: RDS EX1)
  (: WRS)
  (NEWLINE 2)
  (LOOP
    (: ERR)
    ( ((ECHO)
        (PRINT (' ?))
        (SPACES 1)
        % !!!!! Delete next line when output flushing is implemented! %
        (NEWLINE)
        ((AND
            (NOT RDS)
            BELL )
          (PRINT "") ) ) )
    (: EX1)
    (: EX1 (PARSE (SCAN) 0))
    (: EX2 SCAN)
    ( ((ECHO)
        (NEWLINE NEWLINE) ) )
    ( ((OR
          ERR
          (NOT (TERMINATOR)) )
        (SYNTAX)
        (NEWLINE) )
      ((EQ EX2 $)
        (: @ (EVAL EX1))
        ((ECHO)
          (NEWLINE) ) )
      (PRINT (' @))
      (PRINT :)
      (: @ (EVAL EX1))
      (SPACES 1)
      ( ((EQ EX2 ;)
          (PRTMATH @ 0 0 TRUE) )
        (PRINT @) )
      (NEWLINE 2)
      (NEWLINE NEWLINE) ) ) )

(FUNCTION SYNTAX LEX1
  ((EVAL ERR))
  (: ERR TRUE)
  (NEWLINE)
  (PRINT " *** SYNTAX ERROR:  ")
  (LOOP
    ((ATOM LEX1))
    (PRINT (POP LEX1))
    (SPACES 1) )
  (NEWLINE)
  (LOOP
    ( ((AND
          ECHO
          RDS ))
      (PRINT SCAN) )
    ((TERMINATOR))
    (READCHAR) )
  (: RDS FALSE)
  (NEWLINE) )

(MOVD (' LESSER) (' <))

(MOVD (' GREATER) (' >))

(MOVD (' PLUS) (' +))

(PUTPROP PREFIX + (PARSE EX2 130))

(FUNCTION - (EX1 EX2)
  ((EMPTY EX2)
    (MINUS EX1) )
  (DIFFERENCE EX1 EX2) )

(PUTPROP PREFIX - (COND
  ((: EX2 (PARSE EX2 130))
    ((INTEGER EX2)
      (MINUS EX2) )
    (LIST (' -) EX2) ) ))

(MOVD (' TIMES) (' *))

(FUNCTION / (EX1 EX2)
  (QUOTIENT EX1 EX2) )

(PUTPROP LBP : 180)
(PUTPROP LBP = 80)
(PUTPROP LBP EQ 80)
(PUTPROP LBP AND 60)
(PUTPROP LBP OR 50)
(PUTPROP LBP < 80)
(PUTPROP LBP > 80)
(PUTPROP LBP + 100)
(PUTPROP LBP - 100)
(PUTPROP LBP * 120)
(PUTPROP LBP / 120)
(PUTPROP LBP ^ 140)

(PUTPROP RBP = 80)
(PUTPROP RBP EQ 80)
(PUTPROP RBP NOT 70)
(PUTPROP RBP AND 60)
(PUTPROP RBP OR 50)
(PUTPROP RBP < 80)
(PUTPROP RBP > 80)
(PUTPROP RBP + 100)
(PUTPROP RBP - 100)
(PUTPROP RBP * 120)
(PUTPROP RBP / 120)
(PUTPROP RBP ^ 139)

(FUNCTION PRTSPACE ()
  ((NOT PRTSPACE))
  (SPACES 1) )

(FUNCTION PRTPAREN (EX1)
  ((GET (' RBP) LOP1)
    ((> LBP (GET (' RBP) LOP1))
      (PRINT EX1) )
    ((GET (' LBP) LOP1)
      ((< RBP (GET (' LBP) LOP1)))
      (PRINT EX1) ) )
  ((GET (' LBP) LOP1)
    ((< RBP (GET (' LBP) LOP1)))
    (PRINT EX1) ) )

(FUNCTION PRTLIST (LOP1 EX1 LEX1)
  (PRINT LOP1)
  (PRTSPACE)
  (PRINT LPAR)
  (LOOP
    (PRTMATH EX1 0 0)
    ((ATOM LEX1))
    (PRINT COMMA)
    (PRTSPACE)
    (: EX1 (POP LEX1)) )
  (PRINT RPAR) )

(FUNCTION PRTMATH (EX1 RBP LBP PRTSPACE LOP1 LEX1)
  ((ATOM EX1)
    ((AND
        (NEGATIVE EX1)
        (> LBP 130) )
      (PRINT LPAR)
      (PRINT EX1)
      (PRINT RPAR) )
    (PRINT EX1) )
  (: LOP1 (FIRST EX1))
  (: LEX1 (REST EX1))
  ((APPLY (GET (' PRTMATH) LOP1) (LIST LEX1)))
  ((ATOM LEX1)
    (PRINT LOP1)
    (PRTSPACE)
    (PRINT LPAR)
    (PRINT RPAR) )
  (: EX1 (POP LEX1))
  ((ATOM LEX1)
    (PRTPAREN LPAR)
    ((GET (' RBP) LOP1)
      (PRINT LOP1)
      (PRTSPACE)
      (PRTMATH EX1 (GET (' RBP) LOP1) 0)
      (PRTPAREN RPAR) )
    ((GET (' LBP) LOP1)
      (PRTMATH EX1 0 (GET (' LBP) LOP1))
      (PRTSPACE)
      (PRINT LOP1)
      (PRTPAREN RPAR) )
    (PRTLIST LOP1 EX1 LEX1) )
  ((AND
      (GET (' RBP) LOP1)
      (GET (' LBP) LOP1) )
    (PRTPAREN LPAR)
    (PRTMATH EX1 0 (GET (' LBP) LOP1))
    (LOOP
      (: EX1 (POP LEX1))
      (PRTSPACE)
      (PRINT LOP1)
      (PRTSPACE)
      ((ATOM LEX1))
      (PRTMATH EX1 (GET (' RBP) LOP1) (GET (' LBP) LOP1)) )
    (PRTMATH EX1 (GET (' RBP) LOP1) 0)
    (PRTPAREN RPAR) )
  (PRTLIST LOP1 EX1 LEX1) )

% -------------------------------------------- %

(FUNCTION ORDERED (EX1 EX2)
  ((ATOM EX2)
    (ORDERP EX1 EX2))
  ((ATOM EX1) TRUE)
  ((ORDERED (FIRST EX1) (FIRST EX2)) TRUE)
  ((= (FIRST EX1) (FIRST EX2))
    (ORDERED (REST EX1) (REST EX2)))
  FALSE )

% -------------------------------------------- %

(DRIVER)
