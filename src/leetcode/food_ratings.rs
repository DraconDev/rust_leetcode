// use std::collections::HashMap;

// struct FoodInfo {
//     cuisine: String,
//     rating: &'a i32,
// }

// struct CuisineInfo {
//     food: String,
//     rating: &'a i32,
// }
// pub struct FoodRatings {
//     food_map: HashMap<String, FoodInfo>,
//     cuisine_map: HashMap<String, CuisineInfo>,
// }

// impl FoodRatings {
//     pub fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
//         let mut food_map: HashMap<String, FoodInfo> = HashMap::new();
//         let mut cuisine_map: HashMap<String, CuisineInfo> = HashMap::new();
//         for (i, food) in foods.iter().enumerate() {
//             if let Some(cuisine_info) = cuisine_map.get_mut(&cuisines[i]) {
//                 cuisine_map[&cuisines[i]] = CuisineInfo {
//                     food: food.to_string(),
//                     rating: &ratings[i],
//                 }
//             } else {
//                 cuisine_map.insert(&cuisines[i], CuisineInfo {&food, &ratings[i]});
//             }

//             let info = FoodInfo { cuisine, rating };

//             food_map.insert(food, info);
//         }
//         FoodRatings {
//             food_map,
//             cuisine_map,
//         }
//     }

//     pub fn change_rating(&mut self, food: String, new_rating: i32) {
//         self.food_map.get_mut(&food).unwrap().rating = new_rating;
//     }

//     pub fn highest_rated(&self, cuisine: String) -> String {
//         self.cuisine_map
//             .get(&cuisine)
//             .unwrap()
//             .highest_rated
//             .clone()
//     }
// }
