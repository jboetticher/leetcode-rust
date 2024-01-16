use std::collections::HashMap;
use rand::Rng;

struct RandomizedSet {
    // Maps item to index
    map: HashMap<i32, usize>,

    // Maps index to item
    set: Vec<i32>
}

impl RandomizedSet {

    // Generate & store seed
    fn new() -> Self {
        RandomizedSet {
            map: HashMap::<i32, usize>::new(),
            set: Vec::new()
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        let new_index = self.set.len();
        if self.map.get(&val) == None {
            let res = self.map.insert(val, new_index);
            self.set.push(val);
            return true;
        }
        false
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(index) = self.map.remove(&val) {
            let last_item = self.set.pop().unwrap();
            if index != self.set.len() {
                self.set[index] = last_item;
                self.map.entry(last_item).and_modify(|x| *x = index);
            }
            return true;
        }
        false
    }
    
    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        self.set[rng.gen::<usize>() % self.set.len()]
    }
}
