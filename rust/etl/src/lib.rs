use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut m = BTreeMap::new();

    for (n, xs) in h {
        for x in xs {
            m.insert(x.to_lowercase().next().unwrap(), *n);
        }
    }

    m
}
