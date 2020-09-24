#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn to_base_ten(number: &[u32], from_base: u32) -> Result<u32, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    number
        .iter()
        .enumerate()
        .map(|(index, &digit)| {
            if digit >= from_base {
                return Err(Error::InvalidDigit(digit));
            }

            Ok(digit * from_base.pow((number.len() - index - 1) as u32))
        })
        .collect::<Result<Vec<u32>, Error>>()
        .map(|l| l.iter().sum())
}

fn from_base_ten(number: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    let mut ret = vec![];
    let mut current = number;
    while current > 0 {
        ret.insert(0, current % to_base);
        current = current / to_base;
    }
    Ok(ret)
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    from_base_ten(to_base_ten(number, from_base)?, to_base).map(|ret| match ret.is_empty() {
        true => vec![0],
        false => ret,
    })
}
