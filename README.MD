# Rust Cache


This is a simple cache implementation written in Rust.


## Building and Testing

To build the project, run the following command in the project root directory:


 
```shell
cargo build --release --lib 

```

This will build the project in release mode and generate a shared library file (librust_cache.dylib on macOS and librust_cache.so on Linux).

To run the tests, run the following command in the project root directory:

```shell
cargo test
```

# Usage

To use the library in a Python project, first build the shared library file using the command mentioned above. Then, use the following code to create a Cache object in Python:

```python
python3 example.py
```