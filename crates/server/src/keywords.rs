pub(crate) type KeywordDocsMap = std::collections::HashMap<String, String>;

pub(crate) fn load_kw_docs() -> KeywordDocsMap {
    let mut map = KeywordDocsMap::new();
    map.insert("EXPLAIN".to_string(), include_str!("./md/explain.md").to_string());
    map.insert("FROM".to_string(), include_str!("./md/from.md").to_string());
    map.insert("GROUP BY".to_string(), include_str!("./md/group_by.md").to_string());
    map.insert("LIMIT".to_string(), include_str!("./md/limit.md").to_string());
    map.insert("ONLY".to_string(), include_str!("./md/only.md").to_string());
    map.insert("ORDER BY".to_string(), include_str!("./md/order_by.md").to_string());
    map.insert("PARALLEL".to_string(), include_str!("./md/parallel.md").to_string());
    map.insert("SELECT".to_string(), include_str!("./md/select.md").to_string());
    map.insert("SPLIT".to_string(), include_str!("./md/split.md").to_string());
    map.insert("TIMEOUT".to_string(), include_str!("./md/timeout.md").to_string());
    map.insert("VALUE".to_string(), include_str!("./md/value.md").to_string());
    map.insert("WHERE".to_string(), include_str!("./md/where.md").to_string());
    map.insert("WITH".to_string(), include_str!("./md/with.md").to_string());
    map
}
