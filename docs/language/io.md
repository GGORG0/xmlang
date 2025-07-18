# Input/output

## `<print>`

Outputs text to the standard output.

### Attributes

- `newline` ([bool](./data_types/bool.md), optional): Whether to print a newline after the output. Defaults to `true`. Useful for reading input from the user or printing something in chunks. If set to `false`, the next output will continue on the same line.

### Children

All its children are evaluated, converted to [string](./data_types/string.md)s and concatenated together (without any separators), in the order they appear in the XML document.

### Example

```xml
<program>
    <print>Hello, world!</print>
    <print>Sum of 2 and 3 is: <space /> <add><int>2</int><int>3</int></add></print>
    <print newline="false">This is printed without a newline. <space/></print>
    <print>And this is printed on the same line.</print>
</program>
```

This will output:

```
Hello, world!
Sum of 2 and 3 is: 5
This is printed without a newline. And this is printed on the same line.
```

## `<readline />`

Reads a line of input from the user (until the `0xA` newline character is reached) and returns it as a [string](./data_types/string.md).

The returned value is trimmed of trailing CR (carriage return) and LF (line feed) characters, so it can be used directly without further processing. Further trimming has to be done manually with [`<trim>`](./data_types/string.md#trim) if needed.

It doesn't accept any attributes or children.
