/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let normalized = isbn.replace("-", "");
    if normalized.len() != 10 {
        return false;
    }
    normalized
        .chars()
        .enumerate()
        .map(|(index, it)| match (index, it) {
            (9, 'X') => Ok(10),
            (_, 'X') => Err(()),
            (_, digit) => digit.to_string().parse::<usize>().or(Err(())),
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|it| {
            it.iter()
                .enumerate()
                .map(|(index, number)| number * (10 - index))
                .sum::<usize>()
                % 11
                == 0
        })
        .unwrap_or(false)
}
