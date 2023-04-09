use database::{Collection, Data, Database, World};
use std::{
    fs::File,
    io::{Cursor, Write},
};

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
