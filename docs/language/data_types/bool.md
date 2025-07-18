# Boolean

[**Type name:**](./type.md) `bool`

**Rust type:** [`bool`](https://doc.rust-lang.org/std/primitive.bool.html)

A boolean is a data type that can have one of two values: `true` or `false`.

```xml
<bool>true</bool>
<bool>yes</bool>
<bool>anything</bool>
<true />

<bool>false</bool>
<bool>no</bool>
<false />
```

## Conversion to other types

When converting a `bool` to other types, it converts to the following values:

- [**int**](./int.md): `1` for `true`, `0` for `false`.
- [**float**](./float.md): `1.0` for `true`, `0.0` for `false`.
- [**string**](./string.md): The string representation, which is always `true` or `false`.
