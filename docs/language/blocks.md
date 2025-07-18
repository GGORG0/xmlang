# Blocks

Blocks are elements, that execute their children in the order they appear, and return the result of the **last child** as their own result.

Blocks **do not** have their own scope, meaning that [variables](./variables.md) set inside a block are accessible outside of it, and variables set outside of a block are accessible inside it.

Many elements in XMLang are blocks, such as [`<program>`](./program.md), [`<block>`](#block), [`<loop>`](./loop.md), ;[`<if>`](./if.md)'s children, [`<try>`](./errors.md#try)'s children and [functions](./functions.md).

## `<block>`

The `<block>` element is used to create a block of code that can be executed.

This is the simplest way to create a block, and it can be useful when you want to calculate a value inside another element's children, while returning a single value.

If no children are provided, the result of the block is [null](./data_types/null.md).

### Example

```xml
<program>
    <print>
        The result of the block is:
        <space />
        <block>
            <set var="x"><int><readline /></int></set>
            <set var="x"><add><get var="x" /><int>1</int></add></set>
            <get var="x" /> <!-- Only the result of this child will be returned -->
        </block>
    </print>
</program>
```

## `<return>`

The `<return>` element is used to return a value from a block or a function.

It stops the execution of the **nearest** block or function and returns a value to the caller.

No further elements after `<return>` are executed.

### Children

It accepts at most a single child, which is evaluated and returned as the result of the block or function.

If no child is provided, the result of the block or function is [null](./data_types/null.md).

### Example

```xml
<program>
    <print>
        The result of the block is:
        <space />
        <block>
            <set var="x"><int><readline /></int></set>
            <set var="x"><add><get var="x" /><int>1</int></add></set>
            <return><get var="x" /></return>
            <print>This will never be printed.</print>
        </block>
    </print>
</program>
```
