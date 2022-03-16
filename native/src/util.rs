pub fn split_file_name(file_name: &str) -> (Vec<String>, String) {
    let mut file_parts = file_name.split('/').peekable();
    let mut file_path = vec![];
    let mut file_name: Option<String> = None;
    while let Some(path) = file_parts.next() {
        if path.is_empty() {
            continue;
        }
        if file_parts.peek().is_some() {
            file_path.push(path.to_string());
        } else {
            file_name = Some(path.to_string());
        }
    }
    let file_name = file_name.unwrap_or_default();

    (file_path, file_name)
}
