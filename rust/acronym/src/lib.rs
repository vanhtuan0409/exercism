fn word_process(w: String) -> String {}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace("-", "")
        .replace(",", "")
        .replace(":", "")
        .replace("_", "")
        .split(' ')
        .filter(|it| it.is_empty())
        .collect()
}
