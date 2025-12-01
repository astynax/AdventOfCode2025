use std::fs::read_to_string;

pub fn read_lines(path: &str) -> Result<Vec<String>, String> {
    let text = read_to_string(path)
        .map_err(|e| e.to_string())?;
    Ok(text
       .lines()
       .map(|l| l.to_string())
       .collect()
    )
}

pub fn parse_each_with<T, R, E>(parser: T, source: Vec<String>) -> Result<Vec<R>, E>
where T: FnMut(&String) -> Result<R, E> {
    source.iter()
        .map(parser)
        .collect()
}
