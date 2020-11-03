/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits: Option<Vec<u32>> = code
        .chars()
        .filter(|&c| c != ' ')
        .map(|c| c.to_digit(10))
        .collect();
    let digits = match digits {
        Some(val) => val,
        None => return false,
    };

    let count = digits.len();
    // validate code length
    match (digits.get(0), digits.len()) {
        (_, 0) => return false,
        (Some(0), 1) => return false,
        (_, _) => (),
    };

    let sum: u32 = digits
        .into_iter()
        .enumerate()
        .map(|(index, num)| match (count - index - 1) % 2 {
            0 => num,
            _ => {
                let double = num * 2;
                if double > 9 {
                    double - 9
                } else {
                    double
                }
            }
        })
        .sum();

    sum % 10 == 0
}
