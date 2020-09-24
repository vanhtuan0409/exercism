use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

impl Comparison {
    fn reverse(self) -> Self {
        match self {
            Self::Superlist => Self::Sublist,
            Self::Sublist => Self::Superlist,
            other => other,
        }
    }
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal => match first_list == second_list {
            true => Comparison::Equal,
            false => Comparison::Unequal,
        },
        Ordering::Less => sublist(second_list, first_list).reverse(),
        Ordering::Greater => {
            for i in 0..=first_list.len() - second_list.len() {
                if &first_list[i..i + second_list.len()] == second_list {
                    return Comparison::Superlist;
                }
            }
            return Comparison::Unequal;
        }
    }
}
