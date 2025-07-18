# `<rand />`

The `<rand />` element is used to generate a random [integer](./data_types/int.md) within a specified range.

## Attributes

- `min` ([int](./data_types/int.md), optional): The minimum value of the range (inclusive). Defaults to `0`.
- `max` ([int](./data_types/int.md), optional): The maximum value of the range (inclusive). Defaults to [Rust's `i64::MAX`](https://doc.rust-lang.org/std/primitive.i64.html#associatedconstant.MAX).

## Example

```xml
<program>
    <print>
        Random number between 1 and 10: <space /> <rand min="1" max="10" />
    </print>
</program>
```
