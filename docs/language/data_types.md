# Data types

## Null

This is the most basic data type, representing the absence of a value.

```xml
<null />
```

## Integer

An integer is a whole number, which can be positive, negative, or zero.
Under the hood, integers are represented as 64-bit signed integers.

```xml
<int>42</int>
<int>-7</int>
<int>0</int>
```

## Float

A float is a number that can have a fractional part, represented in decimal notation.
Under the hood, floats are represented as 64-bit double-precision floating-point numbers.

**Note:** When converting a float to an integer, the fractional part is discarded.

```xml
<float>1.0</float>
<float>3.14</float>
<float>-2.71828</float>
<float>0.0</float>
```

## Boolean

A boolean is a data type that can have one of two values: `true` or `false`.

```xml
<bool>true</bool>
<bool>false</bool>
```

When converting from a string, all values except for the following are considered `true`:

- `false`
- `0`
- `off`
- `no`
- empty string

## String

A string is a sequence of characters.

All text content in XMLang is considered a string, and wrapping it in a `<string>` element is optional, although sometimes it's necessary for, for example, concatenation - it ensures that the value is treated as a single string, rather than multiple children.

```xml
<string>Hello, world!</string>
<string>42</string>
<string>3.14</string>
<string>true</string>
<string>false</string>
<string />
```
