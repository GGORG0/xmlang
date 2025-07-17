# `<print>`

## Attributes

- `newline` (bool): Whether to print a newline after the output. Defaults to `true`. Useful for reading input from the user or printing something in chunks.

## Children

All children will be converted to strings and concatenated together (without any separators), in the order they appear in the XML document.

## Example

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
