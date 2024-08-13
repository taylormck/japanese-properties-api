//! An API for parsing CSV files containing Japanese real estate data

use core::str;

use axum::{
    debug_handler,
    extract::{Json, Multipart, Path, State},
    routing::{get, post},
    Router,
};

use tokio::sync::RwLock;

use std::{collections::HashMap, sync::Arc};

use japanese_properties_api::property::Property;

/// Our app uses a HashMap as a lazy implementation
/// of an in-memory database
#[derive(Clone, Default)]
struct AppState {
    db: HashMap<usize, Property>,
}

// We need to wrap our state in a RwLock so that we can
// allow an arbitrary number of readers, but lock when
// we have a writer.
// We then wrap in an Arc to make it thread safe
type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() {
    let state = SharedState::default();

    let app = Router::new()
        .route("/up", get(up))
        .route("/properties", get(list_properties))
        .route("/properties/upload", post(upload_csv))
        .route("/properties/:id", get(get_property))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// A simple route just to check if we're up
async fn up() -> &'static str {
    "200 OK"
}

/// The route to upload the CSV file
#[debug_handler]
async fn upload_csv(State(state): State<SharedState>, mut multipart: Multipart) {
    let db = &mut state.write().await.db;

    // The spec isn't completely clear about how long to preserve the property
    // data, so for now we wipe it out whenever a user uploads a new CSV file.
    db.clear();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        if name != "file" {
            continue;
        }

        let data = field.bytes().await.unwrap();
        let rows: Vec<&str> = str::from_utf8(&data).unwrap().lines().collect();

        // We slice off the first row, because that's the header
        rows[1..]
            .iter()
            // Split each row into columns
            .map(|row| row.split(','))
            .enumerate()
            // Map those columns into properties
            .map(|(i, mut columns)| Property {
                // We increment the index to start from 1.
                // This way, we can match the rows in the CSV file
                id: i + 1,
                // str::split returns an iterator, so we can pull each value
                // out one-by-one here and convert them all to owned strings.
                // NOTE: It's important that we do this in the order that matches the CSV:
                // prefecture, city, town, chome, banchi, go, building, price, nearest_station, property_type, land_area
                // TODO: We should ensure the CSV is valid and correctly formed before parsing
                prefecture: columns.next().unwrap().to_owned(),
                city: columns.next().unwrap().to_owned(),
                town: columns.next().unwrap().to_owned(),
                chome: columns.next().unwrap().to_owned(),
                banchi: columns.next().unwrap().to_owned(),
                go: columns.next().unwrap().to_owned(),
                building: columns.next().unwrap().to_owned(),
                price: columns.next().unwrap().to_owned(),
                nearest_station: columns.next().unwrap().to_owned(),
                property_type: columns.next().unwrap().to_owned(),
                land_area: columns.next().unwrap().to_owned(),
            })
            .for_each(|property| {
                // Add each property into the db
                db.insert(property.id, property);
            });
    }
}

/// This route returns all the property data in JSON format
#[debug_handler]
async fn list_properties(State(state): State<SharedState>) -> Json<Vec<Property>> {
    let db = &state.read().await.db;

    // Serde can stringify the whole list for us, but we need to
    // collect the values into a vector first
    let properties: Vec<Property> = db.values().cloned().collect();

    Json(properties)
}

#[debug_handler]
async fn get_property(Path(id): Path<usize>, State(state): State<SharedState>) -> Json<Property> {
    let db = &state.read().await.db;
    let property = db.get(&id).unwrap().clone();

    Json(property)
}
