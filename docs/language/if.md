# `<if>`

The `<if>` element is a conditional statement that allows you to execute a block of code based on whether a condition is true or false.

## Children

This is a [statement](./README.md#expressions-and-statements).
The only elements that can be direct children of `<if>` are the `<condition>`, `<then>`, `<elif>`, and `<else>` elements, which aren't valid elements anywhere else.

### `<condition>`

The `<condition>` element is used to define the condition that will be evaluated to determine whether the `<then>` block should be executed.

There must be exactly 1 `<condition>` element as a child of `<if>`/`<elif>`.

It must have a single child, which is evaluated and converted to a [boolean](./data_types/boolean.md). If the result is `true`, the `<then>` block will be executed.

### `<then>`

The `<then>` element is used to define a block of code that will be executed if the `<condition>` evaluates to `true`.
It is a [block](./blocks.md).

There must be exactly 1 `<then>` element as a child of `<if>`/`<elif>`.

#### Specials

The `<then>` element can access the result of the `<condition>` evaluation using the [`<special>`](./specials.md) element with the `name` attribute set to `condition`. This will return the result as the type of the value returned by the child of `<condition>` (but before being converted to a [boolean](./data_types/bool.md)).

### `<elif>`

The `<elif>` element is used to define an additional condition that will be evaluated if the `<condition>` evaluates to `false` and all previous `<elif>` conditions also evaluated to `false`.
It is a [statement](./README.md#expressions-and-statements).

There can be any number of `<elif>` elements as children of `<if>`.

Each `<elif>` must have exactly 1 `<condition>` and 1 `<then>` element as children.

### `<else>`

The `<else>` element is used to define a block of code that will be executed if the `<condition>` evaluates to `false` and all `<elif>` conditions also evaluated to `false`.
It is a [block](./blocks.md).

There can be at most 1 `<else>` element as a child of `<if>`.
If present, it must be the last child of `<if>`.

## Example

```xml
<program>
    <set var="secret"><int>42</int></set>
    <set var="guess"><int><readline /></int></set>
    <if>
        <condition>
            <eq>
                <get var="guess" />
                <get var="secret" />
            </eq>
        </condition>
        <then>
            <print>Congratulations! You guessed the secret number!</print>
        </then>
        <elif>
            <condition>
                <lt>
                    <get var="guess" />
                    <get var="secret" />
                </lt>
            </condition>
            <then>
                <print>Your guess is too low. Try again!</print>
            </then>
        </elif>
        <else>
            <print>Your guess is too high. Try again!</print>
        </else>
    </if>
</program>
```
