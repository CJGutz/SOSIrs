use crate::file_rep::lines::DefinitionData;

pub fn value_for_key(line: &str) -> Option<DefinitionData> {
    let line = line.trim();
    let indentation = line.find(|c| c != '.').unwrap_or(0);
    let first_space = line[indentation..].find(' ').unwrap_or(line.len());
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
