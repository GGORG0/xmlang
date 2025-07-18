# Data types

XMLang has several built-in data types.

All text content in XMLang is considered a [string](./string.md), and can be converted to other types as needed.
For example, these are different:

```xml
<set var="n"><int>42</int></set>

<print>
    <add>
        <get var="n" />
        1
    </add>
</print> <!-- prints 421 -->

<print>
    <add>
        <get var="n" />
        <int>1</int>
    </add>
</print> <!-- prints 43 -->
```

XMLang supports the following data types:

- [null](./null.md): Represents the absence of a value.
- [int](./int.md): Represents an integer value.
- [float](./float.md): Represents a floating-point number.
- [bool](./bool.md): Represents a boolean value, either `true` or `false`.
- [string](./string.md): Represents a UTF-8 encoded string of text.

There are no lists, arrays, dictionaries, objects, classes, or other complex data types in XMLang.
