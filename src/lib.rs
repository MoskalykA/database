use acursor::write::WriteBytes;
use std::{
    collections::HashMap,
    io::Cursor,
};

pub enum Data {
    String(String),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
}

pub struct Collection {
    name: String,
    data: HashMap<String, Data>,
}

impl Collection {
    pub fn new(name: String) -> Self {
        Self {
            name,
            data: HashMap::default(),
        }
    }

    pub fn serialize(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_string::<u8>(self.name.clone()).unwrap();

        for (key, data) in self.data.iter() {
            cursor.write_string::<u8>(key.to_string()).unwrap();

            match data {
                Data::String(string) => cursor.write_string::<u8>(string.to_string()).unwrap(),
                Data::I8(number) => cursor.write_i8(*number).unwrap(),
                Data::U8(number) => cursor.write_u8(*number).unwrap(),
                Data::I16(number) => cursor.write_i16(*number).unwrap(),
                Data::U16(number) => cursor.write_u16(*number).unwrap(),
                Data::I32(number) => cursor.write_i32(*number).unwrap(),
                Data::U32(number) => cursor.write_u32(*number).unwrap(),
                Data::I64(number) => cursor.write_i64(*number).unwrap(),
                Data::U64(number) => cursor.write_u64(*number).unwrap(),
                Data::I128(number) => cursor.write_i128(*number).unwrap(),
                Data::U128(number) => cursor.write_u128(*number).unwrap(),
            }
        }
    }

    pub fn add_data(&mut self, key: String, data: Data) {
        self.data.insert(key, data);
    }

    pub fn delete_data(&mut self, key: String) {
        self.data.remove(&key);
    }

    pub fn modify_data(&mut self, key: String, data: Data) {
        self.delete_data(key.clone());
        self.add_data(key, data);
    }
}

pub struct Database {
    name: String,
    collections: Vec<Collection>,
}

impl Database {
    pub fn new(name: String) -> Self {
        Self {
            name,
            collections: Vec::default(),
        }
    }

    pub fn serialize(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_string::<u8>(self.name.clone()).unwrap();

        for collection in self.collections.iter() {
            collection.serialize(cursor);
        }
    }

    pub fn add_collection(&mut self, collection: Collection) {
        self.collections.push(collection);
    }
}

#[derive(Default)]
pub struct World(Vec<Database>);

impl World {
    pub fn serialize(&self, cursor: &mut Cursor<Vec<u8>>) {
        cursor.write_u8(self.0.len().try_into().unwrap()).unwrap();

        for database in self.0.iter() {
            database.serialize(cursor);
        }
    }

    pub fn add_database(&mut self, database: Database) {
        self.0.push(database);
    }
}
