use super::{Deserializable, LabelError, RpoDigest, SliceReader, ToString, Vec, MAX_LABEL_LEN};

// LABEL PARSERS
// ================================================================================================

/// Constant label parser.
pub const CONSTANT_LABEL_PARSER: LabelParser = LabelParser {
    caps: true,
    max_len: MAX_LABEL_LEN,
    numbers_letters_underscore: true,
    start_with_letter: true,
};

/// Library namespace label parser.
pub const NAMESPACE_LABEL_PARSER: LabelParser = LabelParser {
    caps: false,
    max_len: MAX_LABEL_LEN,
    numbers_letters_underscore: true,
    start_with_letter: true,
};

/// Procedure label parser.
pub const PROCEDURE_LABEL_PARSER: LabelParser = LabelParser {
    caps: false,
    max_len: MAX_LABEL_LEN,
    numbers_letters_underscore: true,
    start_with_letter: true,
};

// LABEL PARSER IMPLEMENTATION
// ================================================================================================

/// Struct that specifies the rules for parsing labels.
pub struct LabelParser {
    pub caps: bool,
    pub max_len: usize,
    pub numbers_letters_underscore: bool,
    pub start_with_letter: bool,
}

impl LabelParser {
    /// Parses a label and verifies that is passes label conventions.
    /// This is used for procedures and constants.
    ///
    /// Returns an error if label violates the rules.
    pub fn parse_label<'a>(&'a self, label: &'a str) -> Result<&str, LabelError> {
        if label.is_empty() {
            // label cannot be empty
            return Err(LabelError::empty_label());
        } else if label.len() > self.max_len {
            // label cannot be more than `max_len` characters long
            return Err(LabelError::label_too_long(label, self.max_len));
        } else if self.start_with_letter && !label.chars().next().unwrap().is_ascii_alphabetic() {
            // label must start with a letter
            return Err(LabelError::invalid_fist_letter(label));
        } else if self.numbers_letters_underscore
            && !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            // label can consists only of numbers, letters, and underscores
            return Err(LabelError::invalid_label(label));
        } else if self.caps
            && !label
                .chars()
                .all(|c| !c.is_alphabetic() || (c.is_alphabetic() && c.is_uppercase()))
        {
            // all letters must be uppercase
            return Err(LabelError::must_be_uppercase(label));
        }
        Ok(label)
    }
}

// HEX LABEL PARSER
// ================================================================================================.
/// Parses an [RpoDigest] from a hex representation. Verifies that the hex string is 66 characters
/// long, contains only valid hex characters, and that the resulting [RpoDigest] is valid.
pub fn decode_hex_rpo_digest_label(s: &str) -> Result<RpoDigest, LabelError> {
    debug_assert!(s.starts_with("0x"), "hex label must start with 0x");
    if s.len() != 66 {
        Err(LabelError::rpo_digest_hex_label_incorrect_length(s.len()))
    } else {
        let data: Vec<u8> = (2..s.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&s[i..i + 2], 16)
                    .map_err(|_| LabelError::InvalidHexCharacters(s.to_string()))
            })
            .collect::<Result<Vec<_>, LabelError>>()?;
        let mut slice_reader = SliceReader::new(&data);
        RpoDigest::read_from(&mut slice_reader)
            .map_err(|_| LabelError::InvalidHexRpoDigestLabel(s.to_string()))
    }
}
