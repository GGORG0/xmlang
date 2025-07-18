# `<delay>`

The `<delay>` element is used to pause the execution of the program for a specified duration.

## Attributes

- `duration` ([int](./data_types/int.md), optional): The duration in milliseconds for which the program should be paused.

## Children

**If the `duration` attribute has been provided**, it does not accept any children. The program will pause for the specified duration.

**If the `duration` attribute has not been provided**, it must have a single child, which is evaluated and converted to an [int](./data_types/int.md). This value is used as the duration for which the program should be paused.

## Example

```xml
<program>
    <print>Waiting for 1 second...</print>
    <delay duration="1000" />
    <print>Waiting for 2 seconds...</print>
    <delay>
        <mul>
            <int>2</int>
            <int>1000</int>
        </mul>
    </delay>
    <print>Hello, world!</print>
</program>
```
