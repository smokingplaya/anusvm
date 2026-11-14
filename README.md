![anusVM](./assets/logo.png)

anus Virtual Machine™
---

This repository contains `REPL` for interacting with `anusVM™ bytecode`, and anusVM implementation\
Just a pet project

You probbly love to see [tests](./anusvm_core/src/tests.rs) (they're shows how to work with anusVM™)

## Features
- [ ] Functions
  - [ ] Arguments
  - [ ] Frames
  - [ ] Return values
  - [x] C Functions
- [x] Basic math operations `Add`, `Sub`, `AddF`, `SubF`

## Usage
```sh
# anusVM REPL is only way to interact with anusVM (for a while)
cd anusvm_repl
# building & running REPL
cargo run
```

## REPL examples
### Defining new function & calling it
```py
# registering new function in VM's storage
def test_add add 60 60
# pushing new function (function pointer actually) on VM's stack
getfn test_add
# calling function from stack
call
```

### Basic math operations
```py
add 5 6
sub 4 2

# float operations
addf 34.55555 0.91
subf 0.6666 0.3
```

![important thing](./assets/javavm.png)