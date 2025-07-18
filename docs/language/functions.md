# Functions

XMLang supports functions, which are reusable blocks of code.

Functions can take attributes and children, which are used as parameters. They can return a value, which can be used in the calling code.
They can be called from anywhere in the program, including inside other functions, provided they are defined before the call.

Unlike anywhere else in the program, functions have their own local scope.
They can't access or modify variables in the global scope, and values of the local variables defined in the function do not persist between function calls.

## `<function>`

The `<function>` element is used to define a function.

### Attributes

- `name` ([string](./data_types/string.md)): The name of the function.

### Children

This is a [block](./blocks.md).
Its children are saved and will be executed when the function is called.

## `<call>`

The `<call>` element is used to call a function.

### Attributes

- `name` ([string](./data_types/string.md)): The name of the function to call. If the function does not exist, an [error](./errors.md) will be thrown (``Function `{name}` not found``).
- Any other attributes: [see *specials* below](#specials).

### Children

It accepts any number of children, which are evaluated and passed as parameters to the function. [See *specials* below](#specials).

## Specials

The body of the function (children of `<function>`) can access the [attributes](#attributes-1) and [children](#children-1) passed to the function using the [`<special>`](./specials.md) element with the `name` attribute set to:

- The name of the [attribute passed to the `<call>` element](#attributes-1) to retrieve the value of that attribute as a [string](./data_types/string.md).
- `child_count` to retrieve the number of children passed to the `<call>` element as an [int](./data_types/int.md).
- `child:{index}` to retrieve the value of the child at the specified `index` ([int](./data_types/int.md)). The `index` is zero-based, so the first child is `child:0`, the second child is `child:1`, and so on.

## Example

```xml
<program>
    <function name="greet">
        <set var="person">
            <if>
                <condition>
                    <eq>
                        <special name="child_count" />
                        <int>0</int>
                    </eq>
                </condition>
                <then>
                    <special name="person" />
                </then>
                <else>
                    <special name="child:0" />
                </else>
            </if>
        </set>
        <print>Hello, <space /> <get var="person" />!</print>
    </function>

    <call name="greet" person="Alice" />
    <call name="greet">Bob</call>
</program>
```
