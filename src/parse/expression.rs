use std::path::Path;
use crate::{BuildError, Error};
use crate::parse::{Expression, ParsingContext};

impl<Source: AsRef<str> + 'static, File: AsRef<Path> + 'static> ParsingContext<Source, File> {
    pub(super) fn parse_expr(&mut self, depth: usize) -> crate::Result<Expression> {
        return if self.starts_with("{") {
            let mut bracket_count = 0;

            for (offset, char) in self.chars().enumerate() {
                if char == '{' {
                    bracket_count += 1;
                } else if char == '}' {
                    bracket_count -= 1;
                }

                if bracket_count == 0 {
                    let body = self.take(offset + 1);
                    return Ok(Expression {
                        body: body[1..body.len() - 1].to_owned(),
                        origin: self.origin(depth),
                    });
                }
            }

            Err(Error::BuildError(BuildError::BracketMismatch))
        } else {
            Err(Error::BuildError(BuildError::NotAnExpression))
        };
    }

    pub(super) fn parse_hex<Hex: AsRef<str>>(&self, hex_asm: Hex) -> Vec<u8> {
        let mut hex_bytes = hex_asm
            .as_ref()
            .as_bytes()
            .iter()
            .filter_map(|b| match b {
                b'0'..=b'9' => Some(b - b'0'),
                b'a'..=b'f' => Some(b - b'a' + 10),
                b'A'..=b'F' => Some(b - b'A' + 10),
                _ => None,
            })
            .fuse();

        let mut bytes = Vec::new();

        while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
            bytes.push(h << 4 | l)
        }

        return bytes;
    }
}