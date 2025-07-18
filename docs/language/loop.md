# `<loop>`

The `<loop>` element is used to create loops in the program. It allows you to repeat a block of code multiple times.

It's similar to the `for` loop in many other programming languages.

## Attributes

- `start` ([int](./data_types/int.md), optional): The starting value of the loop counter. If not provided, it defaults to `0`.
- `end` ([int](./data_types/int.md), optional): The ending value of the loop counter. If not provided, the loop will run indefinitely until a `<return>` or `<exit />` element is encountered, or an unhandled [error](./errors.md) is thrown.

The loop will run from `start` to `end - 1`, incrementing the loop counter by `1` on each iteration.

If the loop terminates due to the counter reaching `end - 1`, the result of the loop is [null](./data_types/null.md).

## Children

This is a [block](./blocks.md).

Its children will be executed on each iteration of the loop.

### Specials

The `<loop>` element can access the current loop counter using the [`<special>`](./specials.md) element with the `name` attribute set to `iteration`. This will return the current value of the loop counter as an [int](./data_types/int.md).

### `<continue />`

The `<continue />` element is used to skip the rest of the current iteration and move to the next iteration of the loop.

It is only valid inside a `<loop>` block.
When used anywhere else, it will throw an [error](./errors.md) (`Tried to continue outside of a loop`).

### `<return>`

Similarly to other [blocks](./blocks.md), the `<return>` element is used to exit the loop and return a value to the caller.

## Example

```xml
<program>
    <set var="sum"><int>0</int></set>
    <loop start="1" end="6">
        <set var="sum">
            <add>
                <get var="sum" />
                <special name="iteration" />
            </add>
        </set>
        <print>Current iteration: <space /> <special name="iteration" /></print>
    </loop>
    <print>Total sum: <space /> <get var="sum" /></print> <!-- Prints: "Total sum: 15" -->
</program>
```
