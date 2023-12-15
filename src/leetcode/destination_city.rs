// pub fn dest_city(paths: Vec<Vec<String>>) -> String {
//     let mut map: std::collections::HashMap<&String, &String> = std::collections::HashMap::new();

//     for path in paths.iter() {
//         map.insert(&path[0], &path[1]);
//     }

//     for (_, value) in map.iter() {
//         if !map.contains_key(value) {
//             return value.to_string();
//         }
//     }
//     paths[0][1].clone()
// }

// pub fn dest_city(paths: Vec<Vec<String>>) -> String {
//     let mut departures = std::collections::HashSet::new();

//     // Collect all departure cities in a HashSet
//     for path in &paths {
//         departures.insert(path[0].clone());
//     }

//     for path in paths.iter() {
//         if !departures.contains(&path[1]) {
//             return path[1].clone();
//         }
//     }

//     paths[0][1].clone()
// }

pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut departures = std::collections::HashSet::new();

    // Collect all departure cities in a HashSet
    for path in &paths {
        departures.insert(&path[0]);
    }

    for path in &paths {
        if !departures.contains(&path[1]) {
            return path[1].clone(); // Clone only the final result
        }
    }

    paths[0][1].clone()
}
