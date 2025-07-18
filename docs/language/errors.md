# Error handling

Errors in XMLang work similarly to errors in many other programming languages.

They can be thrown (either manually or by a built-in operation) and caught using the `<try>` and `<catch>` elements.

When an error is thrown, the program execution stops and control is transferred to the nearest `<catch>` element that can handle the error.
If no `<catch>` element is found, the program execution stops and the error is printed to the standard output.

## `<throw>`

The `<throw>` element is used to throw an error.

### Attributes

- `message` ([string](./data_types/string.md), optional): The error message to throw.

### Children

**If the `message` attribute has been provided**, `<throw>` does not accept any children. The provided message is used as the error message.

**If the `message` attribute has not been provided**, `<throw>` can optionally have children, which are evaluated, converted to [string](./data_types/string.md)s and concatenated, just like in the [`<print>`](./io.md#print) element.

- If at least 1 child are provided, the resulting string is used as the error message.
- If no children are provided or the constructed error message is empty, the error message is set to `An error occurred, but no message was provided.`.

## `<try>`

The `<try>` element is a statement that allows you to execute a block of code that may throw an error.
If an error is thrown, control is transferred to the **nearest** `<try>` statement's `<catch>` element.

### Children

This is a [statement](./README.md#expressions-and-statements).
The only elements that can be direct children of `<try>` are the `<do>` and `<catch>` elements, which aren't valid elements anywhere else.

#### `<do>`

The `<do>` element is used to define a block of fallible code that will be executed when the `<try>` statement is reached.
It is a [block](./blocks.md).

If an error is thrown during the execution of the `<do>` block, control is transferred to the nearest `<catch>` element.

#### `<catch>`

The `<catch>` element is used to handle errors thrown by the `<do>` block.
It is a [block](./blocks.md).

##### Specials

The `<catch>` element can access the error that was thrown by the `<do>` block using the [`<special>`](./specials.md) element with the `name` attribute set to `error`. This will return the error as a [string](./data_types/string.md).

### Example

```xml
<program>
    <try>
        <do>
            <print>
                The meaning of life is
                <space />
                <div>
                    <int>42</int>
                    <int>0</int> <!-- Oh no! Division by zero! -->
                </div>
            </print>
        </do>
        <catch>
            <print>
                An error occurred:
                <space />
                <special name="error" />
            </print> <!-- Prints: "An error occurred: Division by zero is not allowed" -->
        </catch>
    </try>
</program>
```
