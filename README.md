# crease 

### Current Grammar 
```ebnf
program     = { statement } EOF;
statement   = ( assign_stmt | delete_stmt | input_stmt | print_stmt | if_stmt | while_stmt ) ( NEWLINE | EOF );
assign_stmt = "set" IDENTIFIER "=" expr;
delete_stmt = "del" IDENTIFIER;
input_stmt  = "in" IDENTIFIER;
print_stmt  = "out" expr;
if_stmt     = "if" expr ":" NEWLINE { statement } [ "else" ":" NEWLINE { statement } ] "endif";
while_stmt  = "while" expr ":" NEWLINE { statement } "endwhile";
expr        = or_expr;
or_expr     = and_expr { "||" and_expr };
and_expr    = equality { "&&" equality };
equality    = comparison { ( "==" | "!=" ) comparison };
comparison  = arithmetic { ( "<" | ">" | "<=" | ">=" ) arithmetic };
arithmetic  = term { ( "+" | "-" ) term };
term        = factor { ( "*" | "/" | "%" ) factor };
factor      = NUMBER | BOOLEAN | STRING | IDENTIFIER | "(" expr ")" | ( "+" | "-" | "!" ) factor;

NUMBER      = [0-9]+(\.[0-9]+)?;
BOOLEAN     = "true" | "false";
STRING      = "[^\""]*";
IDENTIFIER  = [a-zA-Z_][a-zA-Z0-9_]*;
COMMENT     = "#" [^\n]*;
```

- TODO: Fix error col marker, add line numbers and additional data, standardize var names, error name revamp, crate import name fixes, token name to literal char name, code cleanup.
