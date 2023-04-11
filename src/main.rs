use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use database::{Collection, Data, Database, World};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

type SharedState = Arc<RwLock<World>>;

#[tokio::main]
async fn main() {
    let mut collection = Collection::new("test".to_string());
    collection.add_data("a".to_string(), Data::String("b".to_string()));

    let mut database = Database::new("hello".to_string());
    database.add_collection(collection);

    let mut world = World::default();
    world.add_database(database);

    let app = Router::new()
        .route("/:database/:collection/get/:key", get(get_data))
        .route("/:database/:collection/set/:key/:value", get(set_data))
        .with_state(Arc::new(RwLock::new(world)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_data(
    Path((database, collection, key)): Path<(u8, u8, String)>,
    State(state): State<SharedState>,
) -> String {
    let world = state.read().unwrap();
    let database = match world.0.get(database as usize) {
        Some(database) => database,
        None => return "This database does not exist".to_string(),
    };

    let collection = match database.collections.get(collection as usize) {
        Some(collection) => collection,
        None => return "This collection does not exist".to_string(),
    };

    let data = match collection.data.get(&key) {
        Some(data) => data,
        None => return "This data does not exist".to_string(),
    };

    match data {
        Data::String(b) => b.to_string(),
        _ => "This data is not supported".to_string(),
    }
}

async fn set_data(
    Path((database, collection, key, value)): Path<(u8, u8, String, String)>,
    State(state): State<SharedState>,
) -> String {
    let mut world = state.write().unwrap();
    let database = match world.0.get_mut(database as usize) {
        Some(database) => database,
        None => return "This database does not exist".to_string(),
    };

    let collection = match database.collections.get_mut(collection as usize) {
        Some(collection) => collection,
        None => return "This collection does not exist".to_string(),
    };

    match collection.data.get(&key) {
        Some(_) => {
            collection.modify_data(key, Data::String(value));
        }
        None => {
            collection.add_data(key, Data::String(value));
        }
    };

    "The data was defined".to_string()
}
