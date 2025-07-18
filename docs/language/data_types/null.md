# Null

[**Type name:**](./type.md) `null`

**Rust type:** [`()`](https://doc.rust-lang.org/std/primitive.unit.html)

This is the most basic data type, representing the absence of a value.

```xml
<null />
```

## Conversion to other types

When converting `null` to other types, it converts to the following values:

- [**int**](./int.md): `0`
- [**float**](./float.md): `0.0`
- [**bool**](./bool.md): `false`
- [**string**](./string.md): the string `null`

## `<unwrap>`

The `<unwrap>` element is used to throw an error if its child is `null`. If the child is not `null`, it returns the child value.

It optionally receives the `message` attribute, which is used as the error message if the child is `null`.

```xml
<unwrap><null /></unwrap> <!-- Error: "Unwrapped value is null" -->
<unwrap message="Custom error message"><null /></unwrap> <!-- Error: "Custom error message" -->
<unwrap><int>42</int></unwrap> <!-- 42 -->
```
