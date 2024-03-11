# Templates
Templates are reusable structures that enable you to reuse components and frameworks defined outside of your content. It is worth considering templates to behave like any other page, with the added difference that it pulls content from other pages at build-time. This means that all features you find in page content work in templates too. 
It is common to define a directory for including templates as this allows you to keep your templates separate from content, and avoid leaking templates into your final build. Although the following approach is highly recommended, it is not necessary;
1. Create a folder holding templates in your site-root. Typically called something similar `#/include`
2. Ensure this directory isn't marked as a page root in your [site config](./config.md#page-roots)
All templates must be placed inside of a `<template>` tag to make them easily identifiable to content and developers. This also allows you to define multiple templates in a single file. This may or may not be useful depending on how you choose to organise your site. 
`<template>` tags require the following parameters:
* `name` - A name used to identify the template throughout your codebase
* `bind` - When the template is invoked, which variable name should be used to retrieve the body
## Variables
Variables allow you to insert content into the template based on certain conditions or originating from various sources. Most commonly, variables are used to interact with templates and their contents. For instance, when loading templates, you will be able to define which variable the page is "bound to", meaning by which name will this content be available to me. 
To call on content within a template, invoke an expression requiring this variable between curly braces: `{variable}`.
## Scripting
A very important feature of a templating engine is the ability to modify or generate the content being templated. We have chosen to use the [Rune](https://github.com/rune-rs/rune) language for its close integration and similarity with Rust, it's ease of integration and in support of its developers. Syntactically, Rune is a subset of Rust - it lacks types and traits. 
Often, when requiring you to provide customised behaviour, you will do so by providing a Rune script either in form of a string or as a path to a script file. We use a naming convention to simplify understanding and using scripts - the `handler` key is used to pass the script directly to the interpreter, while the `handler_path` key indicates it should be read from a file. There is no noticeable difference between these approaches and you should chose the one you prefer. 
```toml
[some-key]
name = "Do something"
handler = """
	// Handle something
"""
handler_path = "path/to/rune/script.rn"
```
The `handler` and `handler_path` keys will always be mutually exclusive in the context of scripting and will throw an error when attempted. 
## Out-of-Template Content
Any content which is interpreted, but not emitted is said to be *out-of-template*. This content is exclusively used to provide metadata to the templating engine or the final template. For instance, when invoking a template, you may choose to do so conditionally. This allows you to create effects such as disruption-of-service banners or to switch between templates according to seasonal variables etc. 