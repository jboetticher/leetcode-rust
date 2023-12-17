use std::collections::{HashMap, BTreeSet};

/*

Originally I just used vectors, which I immediately thought was strange because it was an inefficient data type.
It was too slow, but the logic worked.

Then I tried two HashMaps, which I found out afterwards was the suggested solution. This was also too slow, but I 
was proud of my "rusty" use of iterators.

Finally, I had to use a BTreeSet as was suggested by some of the comments. Never used a binary tree in Rust before!

*/

struct FoodRatings {
    f2r: HashMap<String, i32>,  // food -> rating
    c2fr: HashMap<String, BTreeSet<(i32, String)>>,   // Cuisines -> BTreeSet(rating, food)
    f2c: HashMap<String, String>   // food -> cuisine
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {

    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut f2r: HashMap<String, i32> = HashMap::new();
        let mut c2fr: HashMap<String, BTreeSet<(i32, String)>> = HashMap::new();
        let mut f2c: HashMap<String, String> = HashMap::new();

        for i in 0..foods.len() {
            f2r.insert(foods[i].clone(), ratings[i].clone());
            c2fr.entry(cuisines[i].clone())
                // insert negative, as we want the highest at the top of the tree
                .and_modify(|mut btree| { btree.insert((-ratings[i], foods[i].clone())); }) 
                .or_insert(BTreeSet::from([(-ratings[i], foods[i].clone())]));
            f2c.insert(foods[i].clone(), cuisines[i].clone());
        }

        FoodRatings {
            f2r,
            c2fr,
            f2c
        }       
    }
    
    fn change_rating(&mut self, food: String, new_rating: i32) {
        let cur_rating = self.f2r.get(&food).unwrap();
        self.c2fr.entry(self.f2c.get(&food).unwrap().to_string())
            .and_modify(|mut btree| {
                btree.remove(&(-cur_rating, food.clone())); // Remove original entry
                btree.insert((-new_rating, food.clone()));  // Insert new entry
            });
        self.f2r.entry(food).and_modify(|mut v| *v = new_rating);
    }
    
    // Gets the first entry because we ordered it negative
    fn highest_rated(&self, cuisine: String) -> String {
        self.c2fr.get(&cuisine).unwrap()
                .iter().next().unwrap()
                .1.clone()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */
