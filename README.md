# C FFI on an stm32f3 discovery (Cortex-M4)

In library/, there is a simple C source file. It is compiled to an object file with the given makefile.

In build.rs, rustc is told to link the object file to the final executable.

In main.rs, the extern "C" block declares the C function in a Rust-compatible way.
