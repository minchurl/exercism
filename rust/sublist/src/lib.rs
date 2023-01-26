
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn kmp_generate_failure_function<T: PartialEq>(needle: &[T]) -> Vec<usize> {
    let mut j = 0;
    let mut res: Vec<usize> = Vec::new();

    for (i, v) in needle.iter().enumerate() {
        if i == 0 {
            res.push(j);
            continue;
        }
        loop {
            if needle[j].eq(v) {
                j += 1;
                break;
            }
            if j == 0 {
                break;
            }
            j = res[j - 1];
        }
        res.push(j);
    }
    res
}

fn kmp_find<T: PartialEq>(needle: &[T], haystack: &[T]) -> Option<usize>{
    if needle.len() == 0 {
        return Some(0);
    }

    let f = kmp_generate_failure_function(needle);
    let mut j = 0;

    for (i, v) in haystack.iter().enumerate() {
        loop {
            if needle[j].eq(v) {
                j += 1;
                break;
            }
            if j == 0 {
                break;
            }
            j = f[j - 1];
        }
        if j == needle.len(){
            return Some(i - (needle.len() - 1));
        }
    }
    None
}

fn a_is_in_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    match kmp_find(a, b) {
        Some(_) => true, 
        _ => false, 
    }
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    return match (a_is_in_b(_first_list, _second_list), a_is_in_b(_second_list, _first_list)) {
        (true, true) => Comparison::Equal, 
        (true, false) => Comparison::Sublist, 
        (false, true) => Comparison::Superlist, 
        _ => Comparison::Unequal, 
    }
    // unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
}
