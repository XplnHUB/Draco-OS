pub enum Intent {
    Chat,
    Command(String),
}

pub fn classify(input: &str) -> Intent {
    if input.starts_with('/') || input.to_lowercase().contains("run") {
        Intent::Command(input.to_string())
    } else {
        Intent::Chat
    }
}
