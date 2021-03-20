# limp
Limp is a Lisp-like programming language implemented in Rust. Made for [PackHacks 2021](https://ncsupackhacks.org/).

## Goals
* CLI program
  * [ ] With parameter: runs parses and evaluates given Limp script.
  * [ ] Without parameter: functions as REPL.
* PL construction: grammar
  * Value types
    * [ ] Integers
    * [ ] Floats
    * [ ] Strings
    * [ ] Boolean
    * [ ] Lists
  * Procedures
    * Built-in:
      * [ ] `+`
      * [ ] `-`
      * [ ] `*`
      * [ ] `/`
      * [ ] `print` (maybe separated by type)
      * Many more, obviously
    * [ ] Custom
* PL construction: evaluation
  * All of the above
  * Environment structure (passed by value)
* Support
  * [ ] Example Limp files
  * [ ] Unit tests