pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut map: std::collections::HashMap<&String, &String> = std::collections::HashMap::new();

    for path in paths.iter() {
        map.insert(&path[0], &path[1]);
    }

    for (_, value) in map.iter() {
        if !map.contains_key(value) {
            return value.to_string();
        }
    }
    paths[0][1].clone()
}
