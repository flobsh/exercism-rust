#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (f_len, s_len) if f_len < s_len => {
            if is_sublist_of(_first_list, _second_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (f_len, s_len) if f_len > s_len => {
            if is_sublist_of(_second_list, _first_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (_, _) => {
            if _first_list == _second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}

fn is_sublist_of<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    _second_list
        .windows(_first_list.len())
        .any(|sublist| sublist == _first_list)
}
