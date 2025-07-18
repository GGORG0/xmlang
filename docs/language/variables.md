# Variables

XMLang supports variables, which are used to store data that can be referenced and manipulated throughout the program.

Except for functions, there is 1 global scope in the entire program. Each function execution has its own local scope, which can't access or modify variables in the global scope.

Variables can have any string name, with no restrictions on characters (even spaces are allowed!) or length. However, it's highly recommended to use a proper casing style, with only letters and numbers.

Variables can store values of any [data type](./data_types/README.md). The value and type of a variable can be changed at any time, and the new value will be used in subsequent operations.

## Example

```xml
<program>
    <set var="x"><int>5</int></set>
    <set var="y"><float>3.14</float></set>
    <set var="message">Hello, world!</set>
    <print>Value of x: <space /> <get var="x" /></print>
    <print>Value of y: <space /> <get>y</get></print>
    <print>Message: <space /> <get var="message" /></print>
    <print>This variable does not exist: <space /> <get var="non_existent" /></print>
    <print>And neither does this one: <space /> <get var="also_non_existent"><int>42</int></get></print>
</program>
```

## `<set>`

The `<set>` element is used to assign a value to a variable.

### Attributes

- `var` ([string](./data_types/string.md)): The name of the variable to set. If the variable does not exist, it will be created.

### Children

It accepts a single child, which is evaluated and the resulting value is assigned to the variable.

## `<get>`

The `<get>` element is used to retrieve the value of a variable.

### Attributes

- `var` ([string](./data_types/string.md), optional): The name of the variable to retrieve.

### Children

**If the `var` attribute has been provided**, `<get>` optionally accepts a single child, which is used if the variable does not exist. If the variable exists, this child is ignored. If the variable does not exist and no child is provided, [null](./data_types/null.md) is returned.

**If the `var` attribute has not been provided**, `<get>` must have a single child, which is evaluated and converted to a [string](./data_types/string.md). This child is used as the variable name to retrieve. If the variable does not exist, [null](./data_types/null.md) is returned.
