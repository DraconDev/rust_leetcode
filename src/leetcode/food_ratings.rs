use std::collections::HashMap;

struct FoodInfo {
    cuisine: String,
    rating: i32,
}
pub struct FoodRatings {
    food_map: HashMap<String, FoodInfo>,
}

impl FoodRatings {
    pub fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_map = HashMap::new();
        for (i, food) in foods.into_iter().enumerate() {
            // Create a FoodInfo object for each food item.
            let info = FoodInfo {
                cuisine: cuisines[i].clone(),
                rating: ratings[i],
            };
            // Insert the food key and FoodInfo object into the HashMap.
            food_map.insert(food, info);
        }
        FoodRatings { food_map }
    }

    pub fn change_rating(&mut self, food: String, new_rating: i32) {
        self.food_map.get_mut(&food).unwrap().rating = new_rating;
    }

    fn highest_rated(&self, cuisine: String) -> String {
        let mut max_rating = 0;
        let mut max_food = String::new();
        for (food, info) in self.food_map.iter() {
            if info.cuisine == cuisine {
                if info.rating > max_rating {
                    max_rating = info.rating;
                    max_food = food.to_string();
                }
            }
        }
        max_food
    }
}
