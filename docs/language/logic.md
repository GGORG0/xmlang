# Logical operations

## `<not>`

The `<not>` element is used to negate a boolean value.

### Children

It only accepts a single child, which is evaluated and converted to a [boolean](./data_types/bool.md).
After evaluation, the boolean value is negated (i.e. `true` becomes `false`, and `false` becomes `true`).

### Example

```xml
<program>
    <print>
        <join>
            <not><bool>true</bool></not> <!-- false -->
            <not><int>1</int></not> <!-- false -->
            <not><float>0.0</float></not> <!-- true -->
            <not>hello</not> <!-- false -->
            <not><null /></not> <!-- true -->
        </join>
    </print>
</program>
```

## `<and>`

The `<and>` element is used to perform a logical AND operation on at least two boolean values.

### Children

It accepts at least two children, which are evaluated and converted to [boolean](./data_types/bool.md)s.
The result is `true` if all children evaluate to `true`, and `false` otherwise.

### Example

```xml
<program>
    <print>
        <join>
            <and>
                <bool>true</bool>
                <bool>false</bool>
            </and> <!-- false -->
            <and>
                <int>1</int>
                <float>1.0</float>
            </and> <!-- true -->
            <and>
                <bool>true</bool>
                <not><null /></not>
                <string>hello</string>
            </and> <!-- true -->
        </join>
    </print>
</program>
```

## `<or>`

The `<or>` element is used to perform a logical OR operation on at least two boolean values.

### Children

It accepts at least two children, which are evaluated and converted to [boolean](./data_types/bool.md)s.
The result is `true` if at least one child evaluates to `true`, and `false` otherwise.

### Example

```xml
<program>
    <print>
        <join>
            <or>
                <bool>true</bool>
                <bool>false</bool>
            </or> <!-- true -->
            <or>
                <int>0</int>
                <float>0.0</float>
            </or> <!-- false -->
        </join>
    </print>
</program>
```

## `<eq>`

The `<eq>` element is used to check if all of at least two values are equal.

Two values are considered equal if they have the same type and value.

### Children

It accepts at least two children.
The result is `true` if all children evaluate to the same value, and `false` otherwise.

### Example

```xml
<program>
    <print>
        <join>
            <eq>
                <int>42</int>
                <int>42</int>
            </eq> <!-- true -->
            <eq>
                <int>3</int>
                <float>3.0</float>
            </eq> <!-- false -->
            <eq>
                <bool>true</bool>
                <not><null /></not>
                <bool><string>hello</string></bool>
            </eq> <!-- true -->
            <eq>
                <null />
                <null />
            </eq> <!-- true -->
        </join>
    </print>
</program>
```

## `<ne>`

The `<ne>` element is used to check if none of at least two values are equal.

### Children

It accepts at least two children.
The result is `true` if all children evaluate to different values, and `false` otherwise.

### Example

```xml
<program>
    <print>
        <join>
            <ne>
                <int>42</int>
                <int>42</int>
            </ne> <!-- false -->
            <ne>
                <int>3</int>
                <float>3.0</float>
            </ne> <!-- true -->
            <ne>
                <bool>true</bool>
                <not><null /></not>
                <bool><string>hello</string></bool>
            </ne> <!-- false -->
            <ne>
                <bool>true</bool>
                <string>true</string>
                <not><null /></not>
            </ne> <!-- true -->
        </join>
    </print>
</program>
```

## `<gt>` (>), `<ge>` (≥), `<lt>` (<), `<le>` (≤)

The `<gt>`, `<ge>`, `<lt>`, and `<le>` elements are used to compare at least two values.

### Children

These elements accept at least two children.
The result is `true` if all overlapping pairs of children evaluate to values that meet the specified comparison, and `false` otherwise.

For example, when using `<gt>` with 4 children (a, b, c, d), it returns `true` if (a > b AND b > c AND c > d).

The order of the children matters, as the comparisons are made sequentially.

### Uncomparable data types

Comparisons are done using the Rust [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) trait, which allows for uncomparable values.
This means that if you try to compare two incompatible types (e.g., an integer and a string), the result will be `false`, no matter the comparison operator used.

This can be counterintuitive, see the example (the second child of the comparison elements is a string, which is not comparable to an integer):

```xml
<program>
    <print>
        <join>
            <ge>
                <int>42</int>
                42
            </ge> <!-- false -->
            <eq>
                <int>42</int>
                42
            </eq> <!-- false -->
            <le>
                <int>42</int>
                42
            </le> <!-- false -->
        </join>
    </print>
</program>
```

#### Data type compatibility

This compatibility list is similar to the one used for [mathematical operations](./math.md).

- [null](./data_types/null.md) and [null](./data_types/null.md) are always considered equal.
- [null](./data_types/null.md) and *anything* are **always considered different**.
- [int](./data_types/int.md) and [int](./data_types/int.md) are always comparable.
- [float](./data_types/float.md) and [float](./data_types/float.md) are always comparable.
- [int](./data_types/int.md) and [float](./data_types/float.md) are always comparable (the int is [converted to a float](./data_types/int.md#conversion-to-other-types)).
- [bool](./data_types/bool.md) and [bool](./data_types/bool.md) are always comparable.
- [bool](./data_types/bool.md) and [int](./data_types/int.md)/[float](./data_types/float.md) are always comparable (the bool is [converted to an int/float](./data_types/bool.md#conversion-to-other-types)).
- [string](./data_types/string.md) and [string](./data_types/string.md) are always comparable (lexicographically).
- [string](./data_types/string.md) and *anything* are **always considered different**.
