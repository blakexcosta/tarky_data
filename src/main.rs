use reqwest::blocking::Client;
use std::io::Read;

// NOTE:
// Dioxus and/or leptos might be kinda cool. that's all.
// fn main() {
//     let body = r#"{"query": "{ items(name: \"m855a1\") {id name shortName } }"}"#;
//     let client = Client::new();
//     let mut response = client
//         .post("https://api.tarkov.dev/graphql")
//         .header("Content-Type", "application/json")
//         .header("Accept", "application/json")
//         .body(body)
//         .send()
//         .expect("Failed to send request");

//     let mut body = String::new();
//     response
//         .read_to_string(&mut body)
//         .expect("Failed to read response body");

//     println!("{}", body);
// }

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `GET /` route for tarkov data
        .route("/tarkov", get(tarkov))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, Blake!"
}

// TODO: Make this a post call with axum.
// attempt to move runtime worker
async fn tarkov_query() -> String {
    let body = r#"{"query": "{ items(name: \"m855a1\") {id name shortName } }"}"#;
    let client = Client::new();
    let mut response = client
        .post("https://api.tarkov.dev/graphql")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(body)
        .send()
        .expect("Failed to send request");
    let mut body = String::new();
    response
        .read_to_string(&mut body)
        .expect("Failed to read response body");
    println!("{}", body);
    return body.clone();
}

// basic handler that responds with a static string
async fn tarkov() -> &'static str {
    let query = tarkov_query();
    println!("{:?}", query);
    "Hello, Tarkov!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
