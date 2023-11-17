use crate::file_rep::lines::DefinitionData;

use super::comments::remove_comment;

/// Parses a line of text and returns the key-value pair as a `DefinitionData` struct.
/// Expects at least one punctuation character (.) before the key.
///
/// # Arguments
///
/// * `line` - A string slice that holds the line of text to be parsed.
///
/// # Returns
///
/// An `Option` that contains a `DefinitionData` struct if the line was successfully parsed,
/// or `None` if the line did not contain a valid key-value pair or
///  did not start with punctuation character (.).
pub fn parse_definition_key(line: &str) -> Option<DefinitionData> {
    let line = line.trim();

    let line = remove_comment(line);

    let indentation = line.find(|c| c != '.');
    let first_space = line.find(' ').unwrap_or(line.len());

    if indentation.is_none() {
        return None;
    }
    let indentation = indentation.unwrap();

    let key = &line[indentation..first_space];

    if key.len() == 0 {
        return None;
    }

    let value = line[first_space..].trim();

    return Some(DefinitionData {
        indentation: indentation.try_into().unwrap(),
        key: key.to_string(),
        value: if value.len() > 0 {
            Some(value.to_string())
        } else {
            None
        },
    });
}
