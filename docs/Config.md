The Site Config defines the structure of your website and informs the templating engine where it can find the information it needs to build the final HTML bundle. It is a TOML file containing the following keys:
## `Config`

| Key                | Set With           | Value Type         | Description                                                                                                                                                                                       |
| ------------------ | ------------------ | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`              | `url`              | `String`           | Used in various locations to build URLs linking to pages within the current website, as well as provide optimisations to testing and building                                                     |
| `default_language` | `default_language` | `String`           | Which of the languages defined in `Config::languages.name` should be assumed to be be the default, if no language marker is detected                                                              |
| `languages`        | `language`         | `LanguageConfig`   | A table of [languages](#LanguageConfig)                                                                                                                                                           |
| `roots`            | `roots`            | `Vec<PathBuf>`     | List of directories to search for content in                                                                                                                                                      |
| `build`            | `default_build`    | `PathBuf`          | The final build directory. Static resources are copied here too. A best-effort is made to replicate the structure of the source tree, but the functionality of all pages and links is guaranteed. |
| `content_types`    | `content_type`     | `Vec<ContentType>` | A table of [content types](#ContentType).                                                                                                                                                         |
## `LanguageConfig`

| Key      | Set With       | Value Type | Description                                                                                                                                                                                                                                                                                         |
| -------- | -------------- | ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`   | `abbreviation` | `String`   | How the language is identified in source. This tends to be a two-character abbreviated code of the English name of the language for non-latin languages. For instance:<br><ul><li><code>de</code> - German (Deutsch)</li><li><code>ru</code> - Russian</li><li><code>zh</code> - Mandarin</li></ul> |
| `native` | `full-name`    | `String`   | How the language is referred to natively. For instance: <br><ul><li>German - Deutsch</li><li>Russian - Русский</li><li>Mandarin - 中文</li></ul>                                                                                                                                                      |

## `ContentType`
| Key            | Set With       | Value Type    | Description                                                                         |
| -------------- | -------------- | ------------- | ----------------------------------------------------------------------------------- |
| `extensions`   | `extensions`   | `Vec<String>` | Which file name extensions should trigger this content type                         |
| `handler`      | `handler`      | `String`      | The script source to handle the file - Mutually exclusive with `handler_path`       |
| `handler_path` | `handler_path` | `PathBuf`     | A path containing a scrip to to handle the file - Mutually exclusive with `handler` |
