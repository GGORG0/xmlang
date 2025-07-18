# `<exit />`

The `<exit />` element is used to terminate the program immediately.

## Attributes

- `code` ([int](./data_types/int.md), optional): The exit code of the program. If not provided, the default exit code is `0`.

## Example

```xml
<program>
    <print>Exiting the program...</print>
    <exit code="1" />
    <print>This line will not be executed.</print>
</program>
```
