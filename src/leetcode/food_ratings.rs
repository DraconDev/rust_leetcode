use std::collections::HashMap;

struct FoodInfo {
    cuisine: String,
    rating: i32,
}

struct CuisineInfo {
    highest_rated: String,
    highest_rating: i32,
}
pub struct FoodRatings {
    food_map: HashMap<String, FoodInfo>,
    cuisine_map: HashMap<String, CuisineInfo>,
}

impl FoodRatings {
    pub fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_map: HashMap<String, FoodInfo> = HashMap::new();
        let mut cuisine_map: HashMap<String, CuisineInfo> = HashMap::new();
        for (i, food) in foods.into_iter().enumerate() {
            let cuisine = cuisines[i].clone();
            let food_clone = food.clone();
            let rating = ratings[i];

            if let Some(cuisine_info) = cuisine_map.get_mut(&cuisine) {
                if rating > cuisine_info.highest_rating {
                    cuisine_info.highest_rating = rating;
                    cuisine_info.highest_rated = food_clone;
                } else if rating == cuisine_info.highest_rating {
                    if food.to_lowercase() < cuisine_info.highest_rated.to_lowercase() {
                        cuisine_info.highest_rated = food_clone
                    }
                }
            } else {
                cuisine_map.insert(
                    cuisine.clone(),
                    CuisineInfo {
                        highest_rated: food_clone,
                        highest_rating: rating,
                    },
                );
            }

            let info = FoodInfo { cuisine, rating };

            food_map.insert(food, info);
        }
        FoodRatings {
            food_map,
            cuisine_map,
        }
    }

    pub fn change_rating(&mut self, food: String, new_rating: i32) {
        self.food_map.get_mut(&food).unwrap().rating = new_rating;
    }

    pub fn highest_rated(&self, cuisine: String) -> String {
        self.cuisine_map
            .get(&cuisine)
            .unwrap()
            .highest_rated
            .clone()
    }
}
