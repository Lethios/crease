# crease (WIP Language)

### Current Grammar 
```ebnf
program    = { statement } EOF;
statement  = ( assignment | print | if ) ( NEWLINE | EOF );
assignment = "set" IDENTIFIER "=" expr;
print      = "out" expr;
if         = "if" expr ":" NEWLINE { statement } [ "else" ":" NEWLINE { statement } ] "endif";
expr       = or_expr;
or_expr    = and_expr { "||" and_expr };
and_expr   = equality { "&&" equality };
equality   = comparison { ( "==" | "!=" ) comparison };
comparison = arithmetic { ( "<" | ">" | "<=" | ">=" ) arithmetic };
arithmetic = term { ( "+" | "-" ) term };
term       = factor { ( "*" | "/" | "%" ) factor };
factor     = NUMBER | BOOLEAN | IDENTIFIER | "(" expr ")" | ( "+" | "-" | "!" ) factor;

NUMBER     = [0-9]+(\.[0-9]+)?;
BOOLEAN    = "true" | "false";
IDENTIFIER = [a-zA-Z_][a-zA-Z0-9_]*;
COMMENT    = "#" [^\n]*;
```

- TODO: Fix error col marker, add line numbers and additional data, standardize var names, error name revamp, crate import name fixes, token name to literal char name, code cleanup.
