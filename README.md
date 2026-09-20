# crease (WIP Language)

### Current Grammar 
```ebnf
program    = { statement };
statement  = (assignment | print) SEMICOLON;
assignment = "set" IDENTIFIER "=" expr;
print      = ">>" expr;
expr       = term { ("+" | "-") term };
term       = factor { ("*" | "/") factor };
factor     = NUMBER | IDENTIFIER | "(" expr ")" | ("+" | "-") factor;

NUMBER     => [0-9]+(\.[0-9]+)?;
IDENTIFIER => [a-zA-Z_][a-zA-Z0-9_]*;
```

- TODO: Fix error col marker, add line numbers and additional data, standardize var names, error name revamp, crate import name fixes, code cleanup.
