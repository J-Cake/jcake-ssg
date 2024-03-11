Using Content Types, you can define how various formats get converted into a final HTML bundle. You can define arbitrarily many content types within your [site config](Config.md). Content types represent nothing more than a template which understands the given file's contents. Specifically, they are *translation layers* between your chosen format and the templating engine. This means that while you may not need to define a `<page>` element for a content-typed page, it will ultimately invoke one under-the-hood for you. This is how Markdown support is implemented. 
> [!INFO] The `md` content type is defined by default and generally does not need to be overridden. However, it is possible to do so, if you choose.
> ```toml
> [content-type]
> extensions = ["md"]
> handler = """
>     // Do something
> """
> ```

A content type is a table consisting of a list of file extensions and a handler script. You may provide this script via the `handler` key and entering your script here, or by indicating the script's location with `handler_path`. Like all scripts, it is a [Rune](./templates.md#scripting) script. It should define a function `pub fn content_type(origin: Origin, page: File) -> Result<PageElement, TemplateError>;`
```rust
use jcake_ssg::elements::PageElement;
use jcake_ssg::elements::TextElement;

pub fn content_type(origin, page) {
	Ok(PageElement {
		body: TextElement::new("My custom content type!"),
		origin
	})
}
```
It is common to place these scripts in a separate directory called `content-types`. Although this is recommended for cleanliness, it is not necessary.
Any file now located with the extension `.md` will be opened and passed to the content type. 