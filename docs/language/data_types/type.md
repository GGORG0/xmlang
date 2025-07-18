# `<type>`

The `<type>` element is used to get the type name of a value as a [string](./string.md).
If multiple children are present, their types are concatenated with a space.

```xml
<type>42</type> <!-- int -->
<type>3.14</type> <!-- float -->
<type>true</type> <!-- bool -->
<type>hello</type> <!-- string -->

<type>
    <int>42</int>
    <float>3.14</float>
    <true />
    hello
</type> <!-- int float bool string -->
```
