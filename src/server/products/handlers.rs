use std::sync::Arc;

// Handler function from API
use axum::{
    response::{Json, IntoResponse}, // utilities for constructing HTTP responses
    extract::{Path, State}, // extracting data from HTTP requests
    http::{StatusCode},
};
use serde_json::json;
use tokio_postgres::Client;

use super::entities;
use super::usecases;

// Handler function that handle HTTP requests (API)
pub async fn find_one_product(Path(product_id): Path<i32>) -> impl IntoResponse {
    let product_id_int = product_id.abs();
    match usecases::find_one_product(product_id_int) {
        // OK condition
        entities::Result::Ok(product) => {
            (StatusCode::OK, Json(product).into_response())
        },
        // Error condition
        entities::Result::Err(e) => {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": 400,
                    "message": e,
                })).into_response()
            )
        },
    }
}

// ???
// pub async fn add_robot(Json(payload): Json<entities::Robot>) -> impl IntoResponse {
    pub async fn add_robot(state: State<Arc<Client>>, Json(payload): Json<entities::Robot>) -> Json<String> {
    println!(
        "Received robot: serial={}, name={}, organization={}, robot_type={}",
        payload.serial, payload.name, payload.organization, payload.robot_type
    ); 
    let client = state.as_ref();
    client.
        execute("INSERT INTO public.products
        (serial, name, organization, robot_type)
        VALUES ($1, $2, $3, $4)", 
        &[&payload.serial, &payload.name, 
        &payload.organization, &payload.robot_type])
        .await.unwrap();

    Json("User created successfully".to_string())
}

pub async fn get_robots(
    state: State<Arc<Client>>
) -> Json<Vec<entities::Robot>> {
    println!(
        "Received get_robots"
    );

    let client = state.as_ref();
    let rows = client
            .query("SELECT id, serial, name, organization, robot_type
            FROM public.products", &[])
            .await.unwrap();
    let robots = rows
        .iter()
        .map(|row| entities::Robot {
            serial: row.get(1),
            name: row.get(2),
            organization: row.get(3),
            robot_type: row.get(4)
        })
        .collect();
    Json(robots)
}

pub async fn get_robot() -> impl IntoResponse {
    println!(
        "Received get_1robots"
    );
    Json(json!({
        "message": "Received get_1robots"
    }))
}

// UPDATE Product SET name = 'Updated Product', price = 150 WHERE id = 1;
pub async fn update_robot(
    Path(robot_serial): Path<String>, 
    state: State<Arc<Client>>, 
    Json(payload): Json<entities::UpdateRobot>
) -> impl IntoResponse {
    println!("Received update_robot");

    let client = state.as_ref();
    let mut query = String::from("UPDATE public.products SET ");
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = 
        Vec::new();
    
    if let Some(name) = &payload.name {
        query.push_str("name = $1, ");
        params.push(name);
    }
    if let Some(org) = &payload.organization {
        query.push_str("organization = $2, ");
        params.push(org);
    }
    if let Some(robot_type) = &payload.robot_type {
        query.push_str("robot_type = $3, ");
        params.push(robot_type);
    }

    // Remove trailing comma and add WHERE clause
    query = query.trim_end_matches(", ").to_string();
    query.push_str(" WHERE serial = $4");
    params.push(&robot_serial);

    // Debug !!!
    println!("Incoming serial: {}", robot_serial);
    println!("SQL command: {}", query);
    println!("Incoming request: ");
    for i in &params {
        println!("{:?}", i);
    }

    // Execute the query
    let rows_affected = 
        client.execute(&query, &params)
        .await.unwrap();
    println!("Rows {}", rows_affected);
    if rows_affected > 0 {
        (StatusCode::OK, Json("Robot updated successfully!".to_string()))
    }
    else {
        (StatusCode::BAD_REQUEST, Json("Robot not found.".to_string()))
    }
    
}

pub async fn delete_robot(
    Path(robot_serial): Path<String>,
    state: State<Arc<Client>>
) -> impl IntoResponse {
    println!(
        "Received delete_robot"
    );

    let client = state.as_ref();
    let rows_affected = client
    .execute("DELETE FROM public.products WHERE serial = $1", 
        &[&robot_serial])
    .await.unwrap();

    if rows_affected > 0 {
        (StatusCode::OK, Json("Robot deleted successfully!".to_string()))
    }
    else {
        (StatusCode::BAD_REQUEST, Json("Serial not found.".to_string()))
    }
}