#[derive(Debug)]
pub struct Manager<'a, T> {
    pub collection: std::collections::HashMap<String, &'a mut T>
}

impl<'a, T> Manager<'a, T> {
    
    pub fn init() -> Manager<'a, T> {
        return Manager {
            collection: std::collections::HashMap::new()
        }
    }

    pub fn len(&self) -> usize {
        return self.collection.len();
    }

    pub fn add(&mut self, id: String, item: &'a mut T) {
        self.collection.insert(id, item);
    }

    pub fn get(&self, id: &'a str) -> Option<&T> {
        if let Some(item) = self.collection.get(id) {
            return Some(item);
        }
        return None;
    }

}