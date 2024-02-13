use std::path::PathBuf;
macro_rules! multi_error {
    ($name:ident($($manual:ident),*); $($err:ident = $obj:ty);*) => {
        /// Each line represents a possible error type, acting as a union between all error types below.
        /// This is especially useful when making heavy use of the ? operator, as any Result type whose error type is listed below can be coerced into the defined type.
        #[derive(Debug)]
        pub enum $name {
            $($err($obj),)*
            $($manual),*
        }

        impl std::fmt::Display for Error { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(self, f) } }
        impl std::error::Error for Error {}

        $(impl From<$obj> for $name { fn from(value: $obj) -> Self { Self::$err(value) } })*

    }
}

multi_error! { Error();
    IoError = std::io::Error;
    TomlDeserialiseError = toml::de::Error;
    GlobError = globwalk::GlobError;
    WalkError = globwalk::WalkError;
    JoinError = tokio::task::JoinError;
    RegexError = regex::Error;
    BuildError = BuildError
}

#[derive(Debug)]
pub enum BuildError {
    MatchedDirectory(PathBuf),
    InvalidSyntax(String),
    NoTagName,

    NoClosingTag,
    NoSelectorList,
    BadSelectorList,
    NotATag,
    NotAnExpression,
    BracketMismatch,
    NotALiteral,
    ByteStringNotSupported,

    InvalidCharacterCode(String),
    UnexpectedEOF,
}

pub type Result<T> = core::result::Result<T, Error>;