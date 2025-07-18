# Integer

[**Type name:**](./type.md) `int`

**Rust type:** [`i64`](https://doc.rust-lang.org/std/primitive.i64.html)

An integer is a whole number, which can be positive, negative, or zero.
Under the hood, integers are represented as 64-bit signed integers.

```xml
<int>42</int>
<int>-7</int>
<int>0</int>
```

## Conversion to other types

When converting an `int` to other types, it behaves exactly as you would expect:

- [**float**](./float.md): A float with the same value and a fractional part of `.0`.
- [**bool**](./bool.md): `true` if the value is non-zero, `false` if it is zero.
- [**string**](./string.md): The string representation of the integer, e.g., the string `42`.
