use std::path::Path;
use crate::{
    parse::Origin,
    parse::Literal,
    Error,
    BuildError,
    parse::ParsingContext
};

impl<Source: AsRef<str> + 'static, File: AsRef<Path> + 'static> ParsingContext<Source, File> {
    pub(super) fn parse_literal(&mut self, depth: usize) -> crate::Result<Literal> {
        let allowed_quotes = ['"', '\'', '´'];
        let (modifier, string) = self.find(&allowed_quotes)
            .map(|i| self.split_at(i + 1))
            .ok_or(Error::BuildError(BuildError::NotALiteral))?;

        let modifier_length = modifier.len();
        let modifier = self.legal_string_modifier.captures(modifier)
            .ok_or(Error::BuildError(BuildError::NotALiteral))?;

        let r#mod = modifier.name("mod").unwrap().as_str();
        let hash = modifier.name("hash").unwrap().as_str();
        let quot = modifier.name("quot").unwrap().as_str();

        let literal = if r#mod.contains('r') {
            if let Some(end) = string.find(&format!("{}{}", quot, "#".repeat(hash.len()))) {
                Literal {
                    body: string[..end].to_owned().into_bytes(),
                    is_byte_string: r#mod.contains('b'),
                    origin: Origin {
                        source: self.origin.as_ref().to_path_buf(),
                        offset: self.range().start + modifier_length,
                        depth,
                        token_length: modifier_length + end // TODO: Test
                    },
                }
            } else {
                return Err(Error::BuildError(BuildError::InvalidSyntax(string.to_string())))
            }
        } else if hash.len() > 0 {
            return Err(Error::BuildError(BuildError::NotALiteral))
        } else {
            let mut body = Vec::new();
            let mut iter = string.chars();

            let is_byte = r#mod.contains('b');

            while let Some(next) = iter.next() {
                if next == '\\' {
                    match iter.next() {
                        Some('t') => body.push(b'\t'),
                        Some('n') => body.push(b'\n'),
                        Some('r') => body.push(b'\r'),
                        Some('0') => body.push(b'\0'),
                        Some('b') => { body.pop(); }
                        Some('x') if is_byte => {
                            let mut hex = String::new();
                            let allowed = "0123456789abcdefABCDEF";
                            while let Some(char) = iter.next() {
                                if allowed.contains(char) {
                                    hex.push(char);
                                } else {
                                    body.extend(self.parse_hex(hex));
                                    break;
                                }
                            }
                        }
                        Some('u') if !is_byte => {
                            let char = iter
                                .by_ref()
                                .take_while(|i| *i != '}')
                                .collect::<String>()[1..]
                                .to_owned();

                            let bytes = self.parse_hex(format!("{:0>8}", &char));
                            let char = char::from_u32(u32::from_be_bytes(bytes[0..4].try_into().unwrap()))
                                .ok_or(Error::BuildError(BuildError::InvalidCharacterCode(char)))?;

                            body.extend(char.to_string().bytes());
                        }
                        Some(char) => body.extend(char.to_string().bytes()),
                        _ => return Err(Error::BuildError(BuildError::UnexpectedEOF))
                    }
                } else if next.to_string().eq(quot) { break; } else {
                    body.extend(next.to_string().bytes())
                }
            }

            Literal {
                is_byte_string: is_byte,
                origin: Origin {
                    source: self.origin.as_ref().to_path_buf(),
                    offset: self.range().start + modifier_length,
                    depth,
                    token_length: modifier_length + body.len() + 1
                },
                body,
            }
        };

        return Ok(literal);
    }

}