// 另一种没有所有权概念的就是slice。
// slice允许你引用集合中一段连续的元素序号，而不是整个集合。

pub fn get_slice(s: &str) -> &str {
    let s_bytes = s.as_bytes();

    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        };
    }

    &s[..]
}
