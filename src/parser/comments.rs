pub fn remove_comment(line: &str) -> &str {
    let comment_index = line.find('!').unwrap_or(line.len());
    &line[..comment_index]
}
