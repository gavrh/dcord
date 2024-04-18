/// ### Manager Struct
/// The **Manager** struct is used to cache collections of mutable discord data references such as `guilds`, `channels`, `users` and `more...`
/// 
/// It holds a `HashMap<String, &'a mut T>` and is designed similar to a hash map without the ability to add, remove and obtain mutable references outside of the crate.
///
/// The reason for this is to keep the cached discord data correct and unchanged outside of the crate.
#[derive(Debug)]
pub struct Manager<'a, T> {
    pub(crate) collection: std::collections::HashMap<&'a str, &'a mut T>
}

impl<'a, T> Manager<'a, T> {
    
    pub(crate) fn init() -> Manager<'a, T> {
        return Manager {
            collection: std::collections::HashMap::new()
        }
    }

    pub fn len(&self) -> usize {
        return self.collection.len();
    }

    pub fn get(&self, id: &'a str) -> Option<&T> {
        if let Some(item) = self.collection.get(id) {
            return Some(item);
        }
        return None;
    }

}