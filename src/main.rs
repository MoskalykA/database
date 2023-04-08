use acursor::WriteBytes;
use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Write},
};

pub enum Data {
    String(String),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

pub struct Collection {
    name: String,
    data: HashMap<String, Data>,
}

impl Collection {
    fn new(name: String) -> Self {
        Self {
            name,
            data: HashMap::default(),
        }
    }

    fn serialize(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_string::<u8>(self.name.clone()).unwrap();

        for (key, data) in self.data.iter() {
            cursor.write_string::<u8>(key.to_string()).unwrap();

            match data {
                Data::String(string) => cursor.write_string::<u8>(string.to_string()).unwrap(),
                Data::U8(number) => cursor.write_u8(*number).unwrap(),
                Data::U16(number) => cursor.write_u16(*number).unwrap(),
                Data::U32(number) => cursor.write_u32(*number).unwrap(),
                Data::U64(number) => cursor.write_u64(*number).unwrap(),
                Data::U128(number) => cursor.write_u128(*number).unwrap(),
            }
        }
    }

    fn add_data(&mut self, key: String, data: Data) {
        self.data.insert(key, data);
    }

    fn delete_data(&mut self, key: String) {
        self.data.remove(&key);
    }

    fn modify_data(&mut self, key: String, data: Data) {
        self.delete_data(key.clone());
        self.add_data(key, data);
    }
}

pub struct Database {
    name: String,
    collections: Vec<Collection>,
}

impl Database {
    fn new(name: String) -> Self {
        Self {
            name,
            collections: Vec::default(),
        }
    }

    fn serialize(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_string::<u8>(self.name.clone()).unwrap();

        for collection in self.collections.iter() {
            collection.serialize(cursor);
        }
    }

    fn add_collection(&mut self, collection: Collection) {
        self.collections.push(collection);
    }
}

#[derive(Default)]
pub struct World(Vec<Database>);

impl World {
    fn serialize(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_u8(self.0.len().try_into().unwrap()).unwrap();

        for database in self.0.iter() {
            database.serialize(cursor);
        }
    }

    fn add_database(&mut self, database: Database) {
        self.0.push(database);
    }
}

fn main() {
    let mut collection = Collection::new("test".to_string());
    collection.add_data("a".to_string(), Data::String("Hello".to_string()));
    collection.add_data("b".to_string(), Data::U8(11));
    collection.add_data("c".to_string(), Data::String("Gitchub".to_string()));

    collection.delete_data("b".to_string());

    collection.modify_data("c".to_string(), Data::String("Github".to_string()));

    let mut database = Database::new("hello".to_string());
    database.add_collection(collection);

    let mut world = World::default();
    world.add_database(database);

    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    world.serialize(&mut cursor);

    let mut file = File::create("test.database").unwrap();
    file.write_all(cursor.get_ref()).unwrap();
    file.flush().unwrap();
}
