# String (and string operations)

[**Type name:**](./type.md) `string`

**Rust type:** [`String`](https://doc.rust-lang.org/std/string/struct.String.html)

A string is a UTF-8 encoded sequence of characters.

All [text content](../README.md#text-content) in XMLang is considered a string, and wrapping it in a `<string>` element is optional, although sometimes it's necessary for, for example, concatenation - it ensures that the value is treated as a single string, rather than multiple children.

All children of a `<string>` element are evaluated, converted to strings and then concatenated into a single string (without any separators). [Null](./null.md) values are ignored.

```xml
<string>Hello, world!</string>
<string>42</string>
<string>3.14</string>
<string>true</string>
<string>false</string>
<string />
```

## Conversion to other types

When converting a `string` to other types, it behaves as follows:

- [**int**](./int.md): Parses the string as an integer. If the string is not a valid integer, it will throw an error.
- [**float**](./float.md): Parses the string as a float. If the string is not a valid float, it will throw an error.
- [**bool**](./bool.md): Converts the string to a boolean value, where all values except for the following are considered `true`:
    - `false`
    - `0`
    - `off`
    - `no`
    - empty string

## `<space />`

The `<space />` element returns a string with a single space character.

### Attributes

- `count` ([int](./int.md), optional): Specifies the number of space characters to return. Defaults to `1`.

### Example

```xml
<space /> <!-- " " -->
<space count="3" /> <!-- "   " -->

<print>
    <string>Hello,</string>
    <space />
    <string>world!</string>
</print> <!-- prints "Hello, world!" -->
```

## `<join>`

The `<join>` element concatenates multiple strings into a single string, with a separator.

### Attributes

- `separator` (string, optional): A string to insert between each child string (default is a single space).
- `start` (string, optional): A string to prefix the result with (default is an empty string).
- `end` (string, optional): A string to suffix the result with (default is an empty string).

### Children

All its children get evaluated, and then converted to strings before concatenation.

### Example

```xml
<join end="!">
    <int>1</int>
    <string>Hello</string>
    <string>world</string>
</join> <!-- "1 Hello world!" -->
```

## `<trim>`

The `<trim>` element removes leading and trailing whitespace from a string.

### Attributes

- `start` ([bool](./bool.md), optional): Whether to trim leading whitespace. Defaults to `true`.
- `end` ([bool](./bool.md), optional): Whether to trim trailing whitespace. Defaults to `true`.

### Children

It accepts exactly 1 child, which is the string to trim. The child is evaluated and converted to a string before trimming.

### Example

```xml
<trim>   Hello, world!   </trim> <!-- "Hello, world!" -->
<trim start="true">   Hello, world!   </trim> <!-- "Hello, world!   " -->
<trim end="true">   Hello, world!   </trim> <!-- "   Hello, world!" -->
<trim start="true" end="true">   Hello, world!   </trim> <!-- "Hello, world!" (default) -->
<trim start="false" end="false">   Hello, world!   </trim> <!-- "   Hello, world!   " (why?) -->
```

## `<starts-with>`

The `<starts-with>` element checks if a string starts with a given prefix, and returns a [boolean](./bool.md) value.

### Children

It accepts exactly 2 children - the first is the string to check, and the second is the prefix to check against.

### Example

```xml
<starts-with>
    <string>Hello, world!</string>
    <string>Hello</string>
</starts-with> <!-- true -->
<starts-with>
    <string>Hello, world!</string>
    <string>world</string>
</starts-with> <!-- false -->
<starts-with>
    <string>Hello, world!</string>
    <string>hello</string>
</starts-with> <!-- false (case-sensitive) -->
```

## `<ends-with>`

The `<ends-with>` element checks if a string ends with a given suffix, and returns a [boolean](./bool.md) value.

It is similar to `<starts-with>`, but checks the end of the string instead.

### Children

It also accepts exactly 2 children - the first is the string to check, and the second is the suffix to check against.

### Example

```xml
<ends-with>
    <string>Hello, world!</string>
    <string>world!</string>
</ends-with> <!-- true -->
<ends-with>
    <string>Hello, world!</string>
    <string>Hello</string>
</ends-with> <!-- false -->
<ends-with>
    <string>Hello, world!</string>
    <string>WORLD!</string>
</ends-with> <!-- false (case-sensitive) -->
```

## `<contains>`

The `<contains>` element checks if a string contains a given substring, and returns a [boolean](./bool.md) value.

It is similar to `<starts-with>` and `<ends-with>`, but checks if the string contains the substring anywhere within it.

### Children

It accepts exactly 2 children - the first is the string to check, and the second is the substring to check against.

### Example

```xml
<contains>
    <string>Hello, world!</string>
    <string>world</string>
</contains> <!-- true -->
<contains>
    <string>Hello, world!</string>
    <string>hello</string>
</contains> <!-- false (case-sensitive) -->
<contains>
    <string>Hello, world!</string>
    <string>!</string>
</contains> <!-- true -->
```

## TODO: `<replace>`
