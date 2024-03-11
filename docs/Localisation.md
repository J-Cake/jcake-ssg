> *Localisation* is the act of making the same content available in various *locales* (languages / regional formats) to broaden the possible audience of this content. 

## Localisation methods
There are multiple methods to localising a page, each with various pros and cons. Each localisation strategy requires that pages be discovered in slightly different ways, but it essentially boils down to looking up which languages are required from the [site config](./config.md) and applying the desired strategy for each language. 
### Language extensions
By far the simplest approach to language switching is by simply rewriting the textual content of a page in a different file and translating each version separately. This produces translated content which is clean, easy to understand and easy to manage. 
```
#/pages/about.en.md
	# About Us
	We're a simple company.
#/pages/about.de.md
	# Über Uns
	Wir sind ein einfaches Unternehmen.
```

Here, it is important to note that depending on the filesystem used, similar page names may behave differently. For instance on Windows (NTFS) systems, where filenames are case-insensitive, `#pages/ABOUT.DE.MD` is treated as the German version of `#pages/about.en.md`, while on case-sensitive filesystems it is not. It is therefore recommended to eliminate this variable by keeping page naming consistent. It is generally recommended, that since pages define their title within them, files should always follow kebab-case. 
### Delegated translation
Should one localised copy of a page be restructured or have non-textual content altered in a breaking way, all other copies need to be updated accordingly, in order to prevent imbalances in the content. While this may be perfectly fine for some applications, circumstances may arise where this is simply infeasible to maintain. This is where delegated translation comes in. 
The name is a fancy way of saying the textual content exists in an external map of keys which is *expanded* into each language at build time. This approach tends to be the best when there are a lot of languages with little content each. For instance a home page or a navigation bar. To achieve a delegated translation, you must use a component capable of translating content. There are several which can do this via the `translation` attribute; 
1. The [`<page>`](./pages#translation) tag
2. The `<translate>` tag
3. Doing it yourself in a script by 
	1. Fetching the translation file
	2. Binding it to the page
	3. Querying the translation file
### Language switch
Language switches are simplest when working with a handful of languages at the most. They are analogous to conditional compilation, in that they only emit the content for the specified language. However the switch is enumerated to find a list of languages to be switched and called for each language. This is achieved with the `<langswitch>` tag.
```html
<langswitch lang="en">
	English Content
</langswitch>
<langswitch lang="de">
	Deutscher Inhalt
</langswitch>
```
> [!INFO] A brief note on DX
> As of the present, there is no mechanism to detect whether a language switch will emit any content at all, as they currently exist as yes/no structures. Ideally, a system is defined which enumerates all languages, behaving similarly to Rust's `match` statements, however it is unclear how to cleanly define a system which does this. 
> Therefore, if your language switch doesn't cover all languages of your website, the page will emit with missing content.
### Machine translation
Recommended only as a last-resort, pages can be machine-translated at build time. There are various providers which do this, but require manual setup to work properly. 
```html
<machinetranslation config="deepl">
	<page title="Machine Translation">
		<markdown>r#"
			# This page is machine translated
			
			Is is defined in English but visible in any language. 
		"#</markdown>
	</page>
</machinetranslation>
```
You will need to define the various machine-translation profiles in your [[Config]] under the `machine-translation` key. An example configuration using [the DeepL API](https://www.deepl.com/en/docs-api) would look something like this:
```toml
[machine-translation]
[[machine-translation.profile]]
name = "deepl"
handler_path = "deepl.rn"
arguments = {  }
```
You must declare a function following the type `pub fn translate(page: File, language: String, args: HashMap<String, String>) -> Result<PageElement, TemplateError>`
```rust
// deepl.rn
pub fn translate(page, language) {
	todo!("Call DeepL API")
}
```
As with any scripted part, you may of course enter your script inline in the `handler` key. This will also cause an error if both are defined.
Once you have defined your translation profiles, you may pass a list of languages to the `<machinetranslation>` tag via the `languages` attribute. This will call the profile for each language once. If the argument is omitted, it is assumed that the profile should be called for all languages defined in the site config. 
### Runtime translation
> [!WARNING] This approach is not recommended as it (among other things) requires two roundtrips to the server, dramatically impacting UX.

You can also define your page to fetch the translation file from the server and swap the content with the translated version on the client. This approach tends to be common with very large applications and allows for faster language switching, often avoiding refetching the page. 
This approach is deliberately not supported by JCake-SSG due to the nature of how it is used, as well as being vastly more complex to implement and results in poor UX if implemented improperly. 
## Recommended Approach
The best solution to any problem is of course dependent on the problem itself. The same applies here too - which method works best for you is based on your requirements. The most common approaches are language extensions when there are few languages and more static content, while delegated translation work best for SPAs 
It should also be noted that localisation strategies are only mutually exclusive for the same page. Otherwise, according to necessity can be employed simultaneously.