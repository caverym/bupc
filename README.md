# Bunny Unit Processing Central **DEEP!**

(\ /)<br>
( . .)<br>
C(")(")<br>

An assembly-like interpreted language written using Rust.

The interpreter is currently extremely inefficient, as it is in early development and "whatever works" stage. If you would like to contribute, feel free to.

## Features
* Functions: **NOT IMPLEMENTED IN DEEP REWRITE**
functions are possible, and required. The main function must be called `main:`

* Conditionals:
using `jump_eq` and `jump_neq` you can compare the value of two *compatible* registers and jump to a line.

* Registers:
there are eight operation registers (four signed, four unsigned), process register to set exit code, and IP register, used for loops and jumps.

* You can use STDIN to run code.

## Registers
### visible:
The interpreter has eight operation registers:

`uia, uib, uic, uid` are unsigned 8bit registers which can be used in your program.

`sia, sib, sic, sid` are signed 8bit registers which can be used in your program.

`proc` is the process register used to set the process exit code.

### invisible:
`counter` is the [Program Counter](https://en.wikipedia.org/wiki/Program_counter), this can be manually set using the `jump` or `goto` instructions.  everything before `,` is one counter, this also means that the program will ignore empty lines.

`past` is the past value of the instruction pointer, used on `return` instruction

## Instructions
`set` sets a value in a register.

`view` views the value of a register.

`add` adds the value of a register to the other.

`sub` subtracts the value of a register to the other.

`del` deletes the value of a register.

`jump` jumps to a given point in the program.

`jump_eq` jumps to given point if two registers are equal.

`jump_neq` jumps to given point if two registers aren't equal.

`goto` goes to given function.

`print` prints given text (max of 4 words).

`printl` prints empty line of one line of text.

`return` goes back to function call.

## [Examples](examples/)
hello world:
```
main:,
printl hello world!,
set proc 0,
exit,
```

loops:
```
add:,
add uia uib,
return,

view:,
view uib,
return,

set:,
set uia 1,
set uib 0,
set uic 255,
return,

main:,
goto set:,
goto view:,
goto add:,
jump_neq uib uic 14,
set proc 0,
exit,
```

# License
Copyright 2021 Avery Murray

Licensed under the Apache License, Version 2.0 (the \"License\");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an \"AS IS\" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.