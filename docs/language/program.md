# `<program>`

Every XMLang program has to be a valid XML document.
The root element of the document is `<program>`, which contains the program's code.

This is the simplest XMLang program, which prints `Hello, world!` to the standard output:

```xml
<program>
    <print>Hello, world!</print>
</program>
```

The `<program>` element can contain any number of elements, which are executed in the order they appear in the document.
It is a [block](./blocks.md) - [`<return>`](./blocks.md#return) can be used to stop its execution. The returned value of the `<program>` element is discarded.

The `<program>` element is **only** valid as the root element of the document.
It cannot be used anywhere else in the document.
