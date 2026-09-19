# crease (WIP Language)

### Current Grammar (EBNF)
```ebnf
program    = { statement };
statement  = (assignment | print) SEMICOLON;
assignment = "set" IDENTIFIER "=" expr;
print      = ">>" expr;
expr       = term { ("+" | "-") term };
term       = factor { ("*" | "/") factor };
factor     = NUMBER | IDENTIFIER | "(" expr ")" | ("+" | "-") factor;
```

- TODO: Fix error col marker, add line numbers and additional data, standardize var names, code cleanup.
