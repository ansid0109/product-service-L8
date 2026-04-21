use actix_web::{error, web, Error, HttpResponse};
use crate::model::Product;
use crate::startup::AppState;
use futures_util::StreamExt;
use crate::localwasmtime::validate_product;
use mongodb::bson::doc;

pub async fn add_product(
    data: web::Data<AppState>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > data.settings.max_size {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let mut product = serde_json::from_slice::<Product>(&body)?;

    let max_product = data
        .products_collection
        .find_one(doc! {})
        .sort(doc! { "id": -1 })
        .await
        .map_err(error::ErrorInternalServerError)?;
    let new_id = max_product.map_or(1, |p| p.id + 1);

    // update product id
    product.id = new_id;

    // Add rules engine evaluation here
    match validate_product(&data.settings, &product) {
        Ok(validated_product) => {
            data.products_collection
                .insert_one(&validated_product)
                .await
                .map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().json(validated_product))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().body(e.to_string()))
        }   
    }
}