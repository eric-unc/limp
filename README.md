# Limp
**Limp** is a Lisp-like programming language implemented in Rust. Made for [PackHacks 2021](https://ncsupackhacks.org/).

<img src="limp_logo.png" width="200" alt="The Limp logo.">

## Installation
Limp can be installed by doing the following:
1. Cloning this repo (`git clone https://github.com/eric-unc/limp.git`).
2. Entering the directory (`cd limp`) and installing through Cargo (`cargo install --path .`).

## Running
Limp's command line interface has two modes: REPL and file loading:
* REPL mode: With no argument, Limp will take continually read input from the user and evaluate it. Note that the REPL will not automatically print what is evaluated, so it is not really a true REPL.
* Load mode: With a single argument, it will load the file and run it.

## Language
Limp is a pretty standard Lisp, with a few twists. You can probably figure out what's going on in the example below.

Limp has two major constructions. The first is the "atom", which is a singular unit of value (such as an integer, or a name). The second is the "invocation," which is the activation of a procedure (must be a name type) and a series of values. The procedure being activated is called the "rator" (operator), and the values being passed are called the "rands" (operands).

### Functioning example
```limp
(print (+ 10 5) 1)

(if true (print (/ 8 (- 10 4 4))) (print false))

(if (== 5.0 (+ 3 2)) (exit) (/ 5 0))
```

#### Output
```
15
1
4
```

### Built-in procedures
| Name | Description
| :------ | :------
| `+` | Adds all rands given. Requires at least two rands (int/float).
| `-` | Subtracts the first rand from the remaining rands. Requires at least two rands (int/float).
| `*` | Multiplies all rands given. Requires at least two rands (int/float).
| `/` | Divides the first rand from the remaining rands. Requires at least two rands (int/float).
| `and` | Ands all rands given. Requires at least two rands (boolean).
| `or` | Ors all rands given. Requires at least two rands (boolean).
| `xor` | Xors all rands given. Requires at least two rands (boolean).
| `not` | Inverts the rand given. Requires just one rand (boolean).
| `==` | Compares the rands given for equality. Require two rands (any type).
| `!=` | Compares the rands given for inequality. Require two rands (any type).
| `print` | Prints (on new lines) each rand. Requires at least one rand (int/float/boolean).
| `exit` | Exits the program with a 0 status. With an optional rand, exits with that status  (int/float).
| `if` | Returns one expression if the given condition is true, the other if false. It should be noted that if is a special form, and that the expression within will not be evaluated. Requires three rands (one boolean, two of any type).

## Technologies used
* [Rust](https://github.com/rust-lang/rust)
    * [Cargo](https://github.com/rust-lang/cargo)
    * [pest](https://github.com/pest-parser/pest)
    
## Authors
1. Eric Schneider
2. Chongyi Zheng
