Simple example to create C bindings for a Rust lib.

`cargo build --release` creates two files:

* `mylib.h` is a header file to be included in the C file.
* `target/release/libmylib.so` is an object file to be linked against.

After those files are created, the example could be compiled and executed:

```sh
clang example.c target/release/libmylib.so -o example
./example
```

Hopefully, it will output:
```
[1, 3, 5]
[1, 3, 5, 8]
```

