# crease 

### Current Grammar 
```ebnf
program      = { statement } EOF;
statement    = ( declare_stmt | assign_stmt | delete_stmt | input_stmt | print_stmt | if_stmt | while_stmt ) ( NEWLINE | EOF );
declare_stmt = "set" IDENTIFIER "=" expr;
assign_stmt  = IDENTIFIER "=" expr | IDENTIFIER "[" expr "]" "=" expr;
delete_stmt  = "del" IDENTIFIER;
input_stmt   = "in" IDENTIFIER;
print_stmt   = "out" expr;
if_stmt      = "if" expr ":" NEWLINE { statement } [ "else" ":" NEWLINE { statement } ] "endif";
while_stmt   = "while" expr ":" NEWLINE { statement } "endwhile";
expr         = or_expr;
or_expr      = and_expr { "||" and_expr };
and_expr     = equality { "&&" equality };
equality     = comparison { ( "==" | "!=" ) comparison };
comparison   = arithmetic { ( "<" | ">" | "<=" | ">=" ) arithmetic };
arithmetic   = term { ( "+" | "-" ) term };
term         = factor { ( "*" | "/" | "%" ) factor };
factor       = INT | FLOAT | BOOLEAN | STRING | IDENTIFIER | IDENTIFIER "[" expr "]" | array | "(" expr ")" | ( "+" | "-" | "!" ) factor | ( "int" | "float" | "bool" | "str" ) factor;
array = "[" [ expr { "," expr } ] "]";

COMMENT      = "#" [^\n]*;
IDENTIFIER   = [a-zA-Z_][a-zA-Z0-9_]*;
INT          = [0-9]+;
FLOAT        = [0-9]+\.[0-9]+;
BOOLEAN      = "true" | "false";
STRING       = '"[^"]*"';
```

- TODO: end stmt error fix, Standardize var names, error name revamp.
