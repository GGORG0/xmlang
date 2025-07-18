# Mathematical operations

The compatibility tables below represent the actions taken by the interpreter when the operation is applied to two values of the given types. *anything* means that the operation can be applied to any data type that wasn't explicitly listed above in the list (the list is evaluated from top to bottom, so the first matching type is used).

## Folding operations

**[Addidion](#add), [subtraction](#sub), [multiplication](#mul), [division](#div), and [modulo](#mod)** in XMLang accept any number of children.

They are evaluated and the result is calculated by using the popular `fold` iterator method.

The first child's value is used as the initial value and stored in an accumulator.
Then, each subsequent child's value is used to update the accumulator by applying the operation.
The final value of the accumulator is returned as the result.

If no children are provided, the result is [null](./data_types/null.md).

If the two [data types](./data_types/README.md) of an operation are incompatible, an [error](./errors.md) (`Can't {operation} incompatible types: {type1} and {type2}`) is thrown.

### Example

```xml
<program>
    <print>
        <add>
            <int>1</int>
            <float>2.1</float> <!-- 3.1 -->
            <int>3</int> <!-- 6.1 -->
            <string>a</string> <!-- "6.1a" -->
        </add>
    </print> <!-- prints 6.1a -->
</program>
```

### `<add>`

The order of the children **does not matter**, as addition is commutative.

**Compatible types:**

- [null](./data_types/null.md) + *anything* = the other value
- [int](./data_types/int.md) + [int](./data_types/int.md) = the sum of the two integers ([int](./data_types/int.md))
- [float](./data_types/float.md) + [float](./data_types/float.md) = the sum of the two floats ([float](./data_types/float.md))
- [int](./data_types/int.md) + [float](./data_types/float.md) = the sum of the two numbers ([float](./data_types/float.md))
- [bool](./data_types/bool.md) + [bool](./data_types/bool.md) = the logical OR of the two booleans ([bool](./data_types/bool.md))
- [bool](./data_types/bool.md) + *anything* = the boolean [converted to the *other type*](./data_types/bool.md#conversion-to-other-types) + the other value
- [string](./data_types/string.md) + [string](./data_types/string.md) = the concatenation of the two strings ([string](./data_types/string.md))
- [string](./data_types/string.md) + [int](./data_types/int.md)/[float](./data_types/float.md) = the concatenation of the string and the numer ([string](./data_types/string.md))

### `<sub>`

The order of the children **matters**, as subtraction is not commutative.

**Incompatible types:**

- *anything* - [string](./data_types/string.md)
- [string](./data_types/string.md) - *anything*

**Compatible types:**

- *anything* - [null](./data_types/null.md) = the other value
- [null](./data_types/null.md) - *anything* = the [negation](#neg) of the other value
- [int](./data_types/int.md) - [int](./data_types/int.md) = the difference of the two integers ([int](./data_types/int.md))
- [float](./data_types/float.md) - [float](./data_types/float.md) = the difference of the two floats ([float](./data_types/float.md))
- [int](./data_types/int.md) - [float](./data_types/float.md) = the difference of the two numbers ([float](./data_types/float.md))
- [float](./data_types/float.md) - [int](./data_types/int.md) = the difference of the two numbers ([float](./data_types/float.md))
- [bool](./data_types/bool.md) - [bool](./data_types/bool.md) = the difference of the two booleans [converted to int](./data_types/bool.md#conversion-to-other-types) ([int](./data_types/int.md))
- [bool](./data_types/bool.md) - *anything* = the boolean [converted to the *other type*](./data_types/bool.md#conversion-to-other-types) - the other value
- *anything* - [bool](./data_types/bool.md) = the other value - the boolean [converted to the *other type*](./data_types/bool.md#conversion-to-other-types)

### `<mul>`

The order of the children **does not matter**, as multiplication is commutative.

**Incompabile types:**

- [string](./data_types/string.md) * [string](./data_types/string.md)

**Compatible types:**

- [null](./data_types/null.md) * *anything* = [null](./data_types/null.md)
- [int](./data_types/int.md) * [int](./data_types/int.md) = the product of the two integers ([int](./data_types/int.md))
- [float](./data_types/float.md) * [float](./data_types/float.md) = the product of the two floats ([float](./data_types/float.md))
- [int](./data_types/int.md) * [float](./data_types/float.md) = the product of the two numbers ([float](./data_types/float.md))
- [bool](./data_types/bool.md) * [bool](./data_types/bool.md) = the logical AND of the two booleans ([bool](./data_types/bool.md))
- [bool](./data_types/bool.md) * *anything* = if the boolean is `true`, the other value is returned, otherwise [null](./data_types/null.md) is returned
- [string](./data_types/string.md) * [int](./data_types/int.md) (positive) = the string repeated int times ([string](./data_types/string.md))
- [string](./data_types/string.md) * [int](./data_types/int.md) (negative) = the string reversed, and then repeated int times ([string](./data_types/string.md))
- [string](./data_types/string.md) * [float](./data_types/float.md) (positive) = the string repeated [float [converted to int](./data_types/float.md#conversion-to-other-types)] times ([string](./data_types/string.md))
- [string](./data_types/string.md) * [float](./data_types/float.md) (negative) = the string reversed, and then repeated [float [converted to int](./data_types/float.md#conversion-to-other-types)] times ([string](./data_types/string.md))

### `<div>`

The order of the children **matters**, as division is not commutative.

Division by zero will throw the [error](./errors.md) `Division by zero is not allowed`.

**Incompatible types:**

- [null](./data_types/null.md) / *anything* (but [null](./data_types/null.md) / [null](./data_types/null.md) is allowed)
- *anything* / [null](./data_types/null.md)
- [string](./data_types/string.md) / *anything*
- *anything* / [string](./data_types/string.md)

**Compatible types:**

- [null](./data_types/null.md) / [null](./data_types/null.md) = [null](./data_types/null.md)
- [int](./data_types/int.md) / [int](./data_types/int.md) = the quotient of the two integers ([int](./data_types/int.md))
- [float](./data_types/float.md) / [float](./data_types/float.md) = the quotient of the two floats ([float](./data_types/float.md))
- [int](./data_types/int.md) / [float](./data_types/float.md) = the quotient of the two numbers ([float](./data_types/float.md))
- [float](./data_types/float.md) / [int](./data_types/int.md) = the quotient of the two numbers ([float](./data_types/float.md))
- [bool](./data_types/bool.md) / [bool](./data_types/bool.md) = the quotient of the two booleans [converted to int](./data_types/bool.md#conversion-to-other-types) ([int](./data_types/int.md))
- [bool](./data_types/bool.md) / *anything* = the boolean [converted to the *other type*](./data_types/bool.md#conversion-to_other_types) / the other value
- *anything* / [bool](./data_types/bool.md) = the other value / the boolean [converted to the *other type*](./data_types/bool.md#conversion-to-other_types)

### `<mod>`

The order of the children **matters**, as modulo is not commutative.

Modulo by zero will throw the [error](./errors.md) `Division by zero is not allowed`.

**Incompatible types:**

- [null](./data_types/null.md) % *anything* (but [null](./data_types/null.md) % [null](./data_types/null.md) is allowed)
- *anything* % [null](./data_types/null.md)
- [string](./data_types/string.md) % *anything*
- *anything* % [string](./data_types/string.md)

**Compatible types:**

- [null](./data_types/null.md) % [null](./data_types/null.md) = [null](./data_types/null.md)
- [int](./data_types/int.md) % [int](./data_types/int.md) = the remainder of the two integers ([int](./data_types/int.md))
- [float](./data_types/float.md) % [float](./data_types/float.md) = the remainder of the two floats ([float](./data_types/float.md))
- [int](./data_types/int.md) % [float](./data_types/float.md) = the remainder of the two numbers ([float](./data_types/float.md))
- [float](./data_types/float.md) % [int](./data_types/int.md) = the remainder of the two numbers ([float](./data_types/float.md))
- [bool](./data_types/bool.md) % [bool](./data_types/bool.md) = the remainder of the two booleans [converted to int](./data_types/bool.md#conversion-to_other_types) ([int](./data_types/int.md))
- [bool](./data_types/bool.md) % *anything* = the boolean [converted to the *other type*](./data_types/bool.md#conversion-to_other_types) % the other value
- *anything* % [bool](./data_types/bool.md) = the other value % the boolean [converted to the *other type*](./data_types/bool.md#conversion_to_other_types)

## Unary operations

**[Arithmetic negation](#neg) and [absolute value](#abs)** in XMLang accept a single child.

If the [data type](./data_types/README.md) of an operation is incompatible, an [error](./errors.md) (`Can't {operation} incompatible type: {type}`) is thrown.

### `<neg>`

The `<neg>` element is used to compute the [additive inverse](https://en.wikipedia.org/wiki/Additive_inverse) of a value.

**Incompatible types:**

- [bool](./data_types/bool.md)
- [string](./data_types/string.md)

**Compatible types:**

- [null](./data_types/null.md) = [null](./data_types/null.md)
- [int](./data_types/int.md) = the integer multiplied by -1 ([int](./data_types/int.md))
- [float](./data_types/float.md) = the float multiplied by -1.0 ([float](./data_types/float.md))

### `<abs>`

The `<abs>` element is used to calculate the absolute value of a number.

**Incompatible types:**

- [bool](./data_types/bool.md)
- [string](./data_types/string.md)

**Compatible types:**

- [null](./data_types/null.md) = [null](./data_types/null.md)
- [int](./data_types/int.md) = the absolute value of the integer ([int](./data_types/int.md))
- [float](./data_types/float.md) = the absolute value of the float ([float](./data_types/float.md))
