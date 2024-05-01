use std::{collections::BTreeMap, result};
pub fn transform(i: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut map=BTreeMap::new();
    for (key,chars) in i{
        for c in chars.iter().flat_map(|x| x.to_lowercase()) {
            map.insert(c,*key);
        }

        }
    map
    }
