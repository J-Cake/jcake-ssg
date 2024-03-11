A page is a construct representing any HTML-resultant URL. They tend to map almost perfectly to source files. For instance, you may choose to define a *home* page, an *about* page and a page for each article in a blog. Each of these will be stored as a single file (or multiple - [see templating](Templates.md)) within your source tree. 
* `#www/home.html` will be visible under `http://yourwebsite.com/`
* `#www/about.html` under `http://yourwebsite.com/about.html`
## Page content
The page's content is of course anything you want it to be. As long as it is within a `<page>` tag, it'll be visible. 
### `title`
The `title` attribute is bound to a [template variable](./templates.md#variables) which yourself or the engine can use to include more descriptive or even [out-of-template](./templates.md#out-of-template-content) headings to improve DX. This attribute's value is often used in combination with a [localisation system](./localisation.md), as it allows you to define how the page appears in other languages.
```html
<!--English-->
<page title="About Us">
	<h1>We're a nonexistent company meant for demonstration purposes</h1>
</page>
<!--German-->
<page title="Über Uns">
	<h1>Wir sind eine nicht-existierende Firma die als Beispiel verwendet wird</h1>
</page>
```
### `translation`
This attribute specifies a translation file containing translated versions of snippets used within the page. It is a TOML file with a table for each language. The table names are the abbreviations defined in your [site config](./config.md). The translation map is exposed under `page.translation` unless [`translation-bound`](#translation-bound) is specified.
```html
<page translation="./about.toml" title={page.translation.title}>
	<h1>{page.translation.heading}</h1>
</page>
```
```toml
[en]
title = "About Us"
heading = "We're a nonexistent company meant for demonstration purposes"

[de]
title = "Über Uns"
heading = "Wir sind eine nicht-existierende Firma die als Beispiel verwendet wird"
```
### `translation-bound`
Defines which variable name the translations are exposed under. 
```html
<page translation="./about.toml" translation-bound="lang" title={page.lang.title}>
	<h1>{page.lang.heading}</h1>
</page>
```
