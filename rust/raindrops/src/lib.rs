pub fn raindrops(n: u32) -> String {
    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        return format!("{}", n);
    }
    let mut ret = String::new();
    if n % 3 == 0 {
        ret.push_str("Pling")
    }
    if n % 5 == 0 {
        ret.push_str("Plang")
    }
    if n % 7 == 0 {
        ret.push_str("Plong")
    }
    ret
}
