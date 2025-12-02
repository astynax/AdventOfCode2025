use std::fs::read_to_string;

pub fn read_single_line(path: &str) -> Result<String, String> {
    let text = read_to_string(path)
        .map(|s| s.trim_end().to_string())
        .map_err(|e| e.to_string())?;
    if text.contains("\n") { return Err("Unexpected newline".to_string()) }
    Ok(text)
}

pub fn read_lines(path: &str) -> Result<Vec<String>, String> {
    let text = read_to_string(path)
        .map_err(|e| e.to_string())?;
    Ok(text
       .lines()
       .map(|l| l.to_string())
       .collect()
    )
}
