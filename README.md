## Description

A toy REPL for playing with boolean expressions.

## Usage

Build and run the project with [cargo](rustup.rs/) to enter the REPL, you can use `CTRL` + `C` to exit.

### Expressions

- `a-z` Variable
- `!` Not
- `->` Implies
- `==` Equals
- `&` And
- `|` Or

### Commands

- `table` Generates truth table
- `clear` Clears the screen
- `help` Shows available commands
- `exit` Exits the program

### Example

```
> table (p -> !q) & q
  p  | q  | ¬q  | (p ⭢ ¬q)  | ((p ⭢ ¬q) ∧ q)
  0  | 0  | 1   | 1         | 0
  0  | 1  | 0   | 1         | 1
  1  | 0  | 1   | 1         | 0
  1  | 1  | 0   | 0         | 0
```
