use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
    container: HashMap<i32, i32>,
    ind_memo: HashMap<i32, BTreeSet<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {

    fn new() -> Self {
        Self {
            container: HashMap::new(),
            ind_memo: HashMap::new(),
        }
    }
    
    fn change(&mut self, index: i32, number: i32) {
        if let Some(&old_number) = self.container.get(&index) {
            if let Some(indicies) = self.ind_memo.get_mut(&old_number) {
                indicies.remove(&index);
                if indicies.is_empty() {
                    self.ind_memo.remove(&old_number);
                }
            }
        }

        self.container.insert(index, number);
        self.ind_memo.entry(number).or_insert_with(BTreeSet::new).insert(index);
    }
    
    fn find(&self, number: i32) -> i32 {
        self.ind_memo.get(&number)
            .and_then(|indices| indices.iter().next().cloned())
            .unwrap_or(-1)
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    // честно лень было писать тесты для такой задачи
}

fn main() {}