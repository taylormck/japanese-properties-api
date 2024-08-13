//! An API for parsing CSV files containing Japanese real estate data

use core::str;

use axum::{
    debug_handler,
    extract::{Json, Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use tokio::sync::RwLock;

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

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
        .with_state(state)
        .fallback(not_found);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(3000);

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("Listening on http://{}", address);
    axum::serve(listener, app).await.unwrap();
}

/// A simple route just to check if we're up
async fn up() -> &'static str {
    "200 OK"
}

/// The route to upload the CSV file
#[debug_handler]
async fn upload_csv(
    State(state): State<SharedState>,
    mut multipart: Multipart,
) -> Json<Vec<Property>> {
    let db = &mut state.write().await.db;

    // The spec isn't completely clear about how long to preserve the property
    // data, so for now we wipe it out whenever a user uploads a new CSV file.
    // TODO: We should be backing this data up somehow so that we can restore it
    // in the event that this update fails.
    // If we use a proper database, we can wrap these changes in a transaction
    // and simply drop it on error, or commit on success.
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
            .flat_map(|(i, mut columns)| {
                Some(Property {
                    // We increment the index to start from 1.
                    // This way, we can match the rows in the CSV file
                    id: i + 1,
                    // str::split returns an iterator, so we can pull each value
                    // out one-by-one here and convert them all to owned strings.
                    // If there should be an error, we return None.
                    // None values get filtered out by the `flat_map` call.
                    // NOTE: It's important that we do this in the order that matches the CSV:
                    // prefecture, city, town, chome, banchi, go, building, price, nearest_station, property_type, land_area
                    prefecture: columns.next()?.to_owned(),
                    city: columns.next()?.to_owned(),
                    town: columns.next()?.to_owned(),
                    chome: columns.next()?.to_owned(),
                    banchi: columns.next()?.to_owned(),
                    go: columns.next()?.to_owned(),
                    building: columns.next()?.to_owned(),
                    price: columns.next()?.to_owned(),
                    nearest_station: columns.next()?.to_owned(),
                    property_type: columns.next()?.to_owned(),
                    land_area: columns.next()?.to_owned(),
                })
            })
            .for_each(|property| {
                // Add each property into the db
                db.insert(property.id, property);
            });
    }

    // TODO: report if there were any failed rows

    match db.len() {
        0 => Json(vec![]),
        // Serde can stringify the whole list for us, but we need to
        // collect the values into a vector first
        _ => Json(db.values().cloned().collect()),
    }
}

/// This route returns all the property data in JSON format
#[debug_handler]
async fn list_properties(State(state): State<SharedState>) -> Json<Vec<Property>> {
    let db = &state.read().await.db;

    match db.len() {
        0 => Json(vec![]),
        // Serde can stringify the whole list for us, but we need to
        // collect the values into a vector first
        _ => Json(db.values().cloned().collect()),
    }
}

#[debug_handler]
async fn get_property(
    Path(id): Path<usize>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let db = &state.read().await.db;

    match db.get(&id) {
        Some(value) => Json(value.clone()).into_response(),
        None => (StatusCode::NOT_FOUND, "Property not found").into_response(),
    }
}

#[debug_handler]
async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The page you're looking for doesn't exist",
    )
        .into_response()
}
