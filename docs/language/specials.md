# Specials

Specials are constants set by a containing element.

Examples of specials include the error caught by a [`<try>`](./errors.md#try) element, or the attributes and children passed to a [function](./functions.md).

## `<special>`

The `<special>` element is used to retrieve the value of a special.

If the special of the requested name, the ``Special `{name}` not found`` [error](./errors.md) is thrown.

### Attributes

- `name` ([string](./data_types/string.md), optional): The name of the special to retrieve.

### Children

**If the `name` attribute has been provided**, `<special>` does not accept any children. The value of the special with the given name is returned.

**If the `name` attribute has not been provided**, `<special>` must have a single child, which is evaluated and converted to a [string](./data_types/string.md). This child is used as the name of the special to retrieve.
