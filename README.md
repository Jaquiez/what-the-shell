# What The Shell

A simple shell written in Rust using a tree walk intepreter for the scripting langauge.

## Barebones Grammar

```
    expression      → "(" expression ")"
                    | expression binary_operator expression
                    | expression unary_operator
                    | command;

    command         → WORD arguments;

    arguments       → WORD*
                    | flag*  arguments;

    flag            → SHORT_FLAG
                    | LONG_FLAG;

    SHORT_FLAG      → "-" FLAG_NAME;
    LONG_FLAG       → "--" FLAG_NAME;

    FLAG_NAME       → WORD;

    STRING          → /".*"/;
    WORD            → /[A-Za-z0-9_-_.]+/;

    unary_operator  → "&"
                    | ";";
    
    binary_operator → "|"
                    | ">";
                    | ">>";
                    | "<";
                    | "<<";
```