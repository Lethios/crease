# crease (WIP Language)

### Current Grammar (EBNF)
```ebnf
program    = { statement };
statement  = (assignment | print) SEMICOLON;
assignment = "var" IDENTIFIER "=" expr;
print      = ">>" expr;
expr       = term { ("+" | "-") term };
term       = factor { ("*" | "/") factor };
factor     = NUMBER | IDENTIFIER | "(" expr ")" | ("+" | "-") factor;
```
