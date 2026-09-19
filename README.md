# crease (WIP)

### Current Grammar (EBNF)
```ebnf
expr   = term { ("+" | "-") term };
term   = factor { ("*" | "/") factor };
factor = NUMBER | "(" expr ")" | ("+" | "-") factor;
```
