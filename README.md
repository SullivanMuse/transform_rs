# Transform

Transform is a minimal language for exploring automatic program transformation. "Automatic program transformation" includes things like macros, static analysis, proof assistants, compilers, interpreters, transpilers.

## Dev Rules

- Do not commit if it doesn't run.

## Goals

1. Facilitate staged analysis
2. Facilitate automatic transformation
3. Be able to write every algorithm/data structure under the sun in a minimal sub-language with statically checked assumptions
4. Assumptions is a broad term which covers
    - Type annotations
    - Trait implementations
    - Pre-conditions
    - Discipline (sub-language)
        - Throwing - whether or not the procedure throws exceptions
        - Purity - giving the same result with the same arguments
        - Mutation - whether or not the procedure mutates its argument(s)
        - N-Linearity, affinity, owernship - what the procedure does with its arguments

## Features

### Planned

- Minimal pure, immutable interpreted homoiconic language
    - Data
        - Lists
        - Atoms
        - Machine types
            - u8 - u64
            - i8 - i64
            - pointers
    - `let symbol value body`: define a symbol
    - `expand`: evaluate a quasi-quote into the ast where it stands
    - `quote`: quasi-quoted syntax literal
    - `interp`: evaluate expressions inside a quasi-quoted syntax literal
    - Homoiconicity + Staged compilation are the minimal set
