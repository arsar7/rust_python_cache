# Rust Cache


The code defines a Rust module called rust_cache that exposes a Python class called Cache using the `pyo3 crate`.
The `Cache` struct contains an instance of `StringCache`, which is defined in the cache module. The `StringCache` struct is a simple implementation of an LRU cache for string keys and values.



The **Cache** class has several methods defined using the pymethods attribute:


- ``new(capacity: usize)`` : a constructor that creates a new Cache instance with the specified capacity.

- ``get(key: &str) -> Option<String>`` : a method that retrieves the value associated with a given key. If the key is not in the cache, None is returned.

- ``put(key: String, value: String)`` : a method that inserts a key-value pair into the cache. If the cache is full, the least recently used key-value pair is removed to make room for the new entry.

- ``len() -> usize`` : a method that returns the number of key-value pairs currently in the cache.

- ``capacity() -> usize`` : a method that returns the maximum number of key-value pairs that can be stored in the cache.

Finally, the rust_cache module is defined using the pymodule attribute, which allows it to be imported and used as a Python module. The Cache class is added to the module using the add_class method.


## Usage


### To use Rust Cache, follow these steps:

1.Create a virtual environment by running the following commands:
  ```shell
  python3 -m venv .env
  source .env/bin/activate
  pip install maturin>=0.14.15
  ```

2.Build Rust Cache with Maturin by running:
  ```shell
  maturin build
  ```

  - To build for Silicon Chips, use:
    ```shell
    maturin build --target=x86_64-apple-darwin
    ```

3.Install the .whl package by running:
  ```shell
  pip install target/wheels/*whl --force-reinstall
  ```


4.Test Rust Cache by running the following command:
  ```shell
  cargo test
  ```

Clean up target file with ``cargo clean`` command

5.Clean up the target file by running:
  ```shell
  cargo clean
  ```


After following these steps, you can test the package by opening a Python interactive shell and running import rust_cache.
Like this :) 
![img.png](img.png)