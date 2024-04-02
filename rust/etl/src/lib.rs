use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {

    let res = h
        .iter()
        .flat_map(|(&point, char_list)| 
            char_list.iter().map(move |&c| (c.to_ascii_lowercase(), point))
        )
        .collect::<BTreeMap<char, i32>>();

    res

    // todo!("How will you transform the tree {h:?}?")
}
