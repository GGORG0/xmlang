# Language introduction

## Elements

Everything in XMLang is an **XML element**.

An element is a tag in the XML document, which can have **attributes**, **children**, and **text content**.

```xml
<element attribute1="value1" attribute2="value2">
    <child1>Child 1 content</child1>
    <child2>Child 2 content</child2>
</element>
```

### Attributes

Attributes are defined in the opening tag of an element, and are used to provide constant, static values to the element. They can't be dynamically generated or changed at runtime.

Attribute values are always strings by default, but certain built-in elements parse them as other types.

### Children

Children are elements that are nested inside another element.

Some elements accept 0, 1, a predefined number, or an arbitrary number of children.

Every child element of an expression is considered an **argument** to that expression, and the order of the children matters.
The children are evaluated in the order they appear in the XML document, and the result of each child is passed to the parent element as an argument.

An element can also be self-closing, as defined by the XML standard:

```xml
<element attribute1="value1" attribute2="value2" />
```

That is equivalent to:

```xml
<element attribute1="value1" attribute2="value2"></element>
```

### Text content

Text content is the text that appears between the opening and closing tags of an element.

It is always considered a string, and has to be converted to other types if needed.

You can mix text content with other child elements, but due to parser limitations, the whitespace between the text and the child elements is not preserved, and will be ignored.

```xml
<element attribute1="value1" attribute2="value2">
    Text content
    <child1>Child 1 content</child1>
    <child2>Child 2 content</child2>
    Other text content
</element>
```

If you want to preserve whitespace, you can use tricks like the `<space />` element or the `<join>` element, which will be explained later.

```xml
<program>
    <set var="myVar">wonderful</set>
    <print>Hello <space /> <get var="myVar" /> <space /> world!</print>
</program>
```

When mixing text content with child elements, each text node and child element is considered a separate child of the parent element, and the order of the children matters. If you want to concatenate text content with child elements, keeping them as 1 string child, you can use the `<string>` element to wrap the text content and child elements together.

```xml
<element attribute1="value1" attribute2="value2">
    <string>Text content <child1>Child 1 content</child1> Other text content</string>
</element>
```

## Expressions and statements

There are 2 main types of elements in XMLang: **expressions** and **statements**.
Almost everything is an expression.

The 2 are distinguished by the fact that statements contain children that aren't valid elements anywhere else in the language, for example, the `<if>` statement contains `<condition>`, `<then>`, and optionally `<else>` elements, which are not valid outside an `<if>` statement.

Every element returns a value, which can be used by its parent element.
If an element does not return a value, it is considered to return `null`.
