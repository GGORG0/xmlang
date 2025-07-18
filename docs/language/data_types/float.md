# Float

[**Type name:**](./type.md) `float`

A float is a number that can have a fractional part, represented in decimal notation.
Under the hood, floats are represented as 64-bit double-precision floating-point numbers (Rust's `f64`).

```xml
<float>1.0</float>
<float>3.14</float>
<float>-2.71828</float>
<float>0.0</float>
```

## Conversion to other types

When converting a `float` to other types, it behaves exactly as you would expect:

- [**int**](./int.md): An integer with the same value, discarding the fractional part (always rounding down).
- [**bool**](./bool.md): `true` if the value is non-zero, `false` if it is zero (`0.0`).
- [**string**](./string.md): The string representation of the float, e.g., the string `42.5`.
