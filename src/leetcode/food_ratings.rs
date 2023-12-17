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
            if !cuisine_map.contains_key(&cuisines[i]) {
                cuisine_map.insert(
                    cuisines[i].clone(),
                    CuisineInfo {
                        highest_rated: food.clone(),
                        highest_rating: ratings[i].clone(),
                    },
                );
            } else {
                if ratings[i] > cuisine_map.get(&cuisines[i]).unwrap().highest_rating {
                    cuisine_map.get_mut(&cuisines[i]).unwrap().highest_rated = food.clone();
                    cuisine_map.get_mut(&cuisines[i]).unwrap().highest_rating = ratings[i];
                } else if ratings[i] == cuisine_map.get(&cuisines[i]).unwrap().highest_rating
                    && food < cuisine_map.get(&cuisines[i]).unwrap().highest_rated
                {
                    cuisine_map.get_mut(&cuisines[i]).unwrap().highest_rated = food.clone();
                }
            }

            let info = FoodInfo {
                cuisine: cuisines[i].clone(),
                rating: ratings[i],
            };

            food_map.insert(food.clone(), info);
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
