use actix_web::{error, web, Error, HttpResponse};
use crate::model::Product;
use crate::startup::AppState;
use futures_util::StreamExt;
use crate::localwasmtime::validate_product;
use mongodb::bson::doc;

pub async fn update_product(
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
    let product = serde_json::from_slice::<Product>(&body)?;
    
    match validate_product(&data.settings, &product) {
        Ok(validated_product) => {
            let result = data
                .products_collection
                .replace_one(doc! { "id": validated_product.id }, &validated_product)
                .await
                .map_err(error::ErrorInternalServerError)?;

            if result.matched_count == 0 {
                Ok(HttpResponse::NotFound().body("Product not found"))
            } else {
                Ok(HttpResponse::Ok().json(validated_product))
            }
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().body(e.to_string()))
        }
    }  
}