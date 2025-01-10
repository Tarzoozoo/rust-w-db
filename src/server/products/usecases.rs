use tokio_postgres::Client;

// Business logic for interating with the data or services 
use super::entities;

pub fn find_one_product(product_id: i32) -> entities::Result<entities::Product, String> {
    let mut products: Vec<entities::Product> = Vec::new();

    products.push(entities::Product {
        id: 1,
        title: String::from("Tom Yum Kung"),
        description: String::from("Thai's food"),
    });
    products.push(entities::Product {
        id: 2,
        title: String::from("Shushi"),
        description: String::from("Japanese's food"),
    });
    products.push(entities::Product {
        id: 3,
        title: String::from("Roti"),
        description: String::from("Indian's food"),
    });

    for product in products {
        if product.id == product_id {
            return entities::Result::Ok(product)
        };
    }
    entities::Result::Err(format!("product_id: {} not found", product_id))
}


// pub async fn fetch_all_products(client: &Client) -> Vec<entities::Robot> {
//     let rows = client
//             .query("SELECT id, serial, name, organization, robot_type
//             FROM public.products", &[])
//             .await.unwrap();
//     rows
//         .iter()
//         .map(|row| entities::Robot {
//             serial: row.get(1),
//             name: row.get(2),
//             organization: row.get(3),
//             robot_type: row.get(4)
//         })
//         .collect();
// }