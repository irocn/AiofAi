use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use sled::{Db, IVec, Iter};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HistoryChat {
    userid: String,
    chats: Vec<(String,String)>
}

impl HistoryChat {

    pub fn new(_userid:&str) -> Self {
        HistoryChat {
            userid: _userid.to_owned(),
            chats: Vec::new(),
        }
    }

    // Method to add a key-value pair to the struct
    pub fn add_chat(&mut self, db: &sled::Db, conversation_id: &str, content: &str) {
        let tree = db.open_tree(self.userid.clone()).unwrap();
        tree.insert(conversation_id, content).unwrap();
        let _ = tree.flush_async();
    }

    // Method to get the value associated with a given key
    pub fn get_chat(&mut self, db: &sled::Db) -> Vec<(String, String)>{
        let tree = db.open_tree(self.userid.clone()).unwrap();
        
        tree.iter().for_each(|x|{
            let binding = x.unwrap();
            let _key = String::from_utf8_lossy(&binding.0.as_ref());
            let _value = String::from_utf8_lossy(&binding.1.as_ref());
            self.chats.push( ( _key.to_string() , _value.to_string() ) )
        });

        self.chats.clone()
    }
}

#[cfg(test)]
mod tests {
    use sled::Db;

    use super::HistoryChat;


    #[test]
    fn insert(){
    }

    #[test]
    fn test_db_tree(){
            // Open or create a Sled database
    let db = Db::open("my_database").unwrap();

    // Open a tree named "users"
    let users_tree = db.open_tree("users").unwrap();

    // Insert some data into the "users" tree
    users_tree.insert(b"1", b"Alice").unwrap();
    users_tree.insert(b"2", b"Bob").unwrap();

    // Open another tree named "products"
    let products_tree = db.open_tree("products").unwrap();

    // Insert some data into the "products" tree
    products_tree.insert(b"101", b"Book").unwrap();
    products_tree.insert(b"102", b"Laptop").unwrap();


    products_tree.iter().all(|x|{
        println!("===Product with key 101: {:?}", String::from_utf8_lossy(&x.clone().unwrap().0.as_ref()));
        println!("===Product with key 101: {:?}", String::from_utf8_lossy(&x.unwrap().1.as_ref()));
        true
    });
    }
}