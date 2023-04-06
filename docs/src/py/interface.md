# Python interface

Python bindings are the main way for most users to interact with the **repeto** library. These bindings are designed to
work seamlessly with Python's memory system, and they include the following features:

* All objects and fields are treated as pointers, following Python's conventions.
* The bindings support pickling and unpickling, which allow users to serialize and deserialize objects.
* The bindings also support type hints, which provide additional information about the types of objects and functions in
  the library.

One thing to keep in mind when using `repeto.optimize` function is that all input inverted repeats are copied into
native Rust types and then transformed back into Python classes. This means that any additional links to input repeats
will no longer be valid.