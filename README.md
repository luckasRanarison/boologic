## Description

A toy REPL for getting truth tables from boolean expressions.

## Usage

Build and run the project with [cargo](rustup.rs/) to enter the REPL, you can use use `CTRL` + `C` to exit.

### Expressions

- `a-z` Variable
- `!` Not
- `->` Implies
- `==` Equals
- `&` And
- `|` Or

### Example

```
> (p -> q) & q
  p  | q  | ¬q  | (p ⭢ ¬q)  | ((p ⭢ ¬q) ∧ q)
  0  | 0  | 1   | 1         | 0
  0  | 1  | 0   | 1         | 1
  1  | 0  | 1   | 1         | 0
  1  | 1  | 0   | 0         | 0
```
