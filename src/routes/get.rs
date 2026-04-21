use crate::model::ProductInfo;
use crate::startup::AppState;
use actix_web::{error, web, Error, HttpResponse};
use futures_util::TryStreamExt;
use mongodb::bson::doc;

pub async fn get_product(
    data: web::Data<AppState>,
    path: web::Path<ProductInfo>,
) -> Result<HttpResponse, Error> {
    let product = data
        .products_collection
        .find_one(doc! { "id": path.product_id })
        .await
        .map_err(error::ErrorInternalServerError)?;

    if let Some(product) = product {
        Ok(HttpResponse::Ok().json(product))
    } else {
        Ok(HttpResponse::NotFound().body("Product not found"))
    }
}

pub async fn get_products(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let cursor = data
        .products_collection
        .find(doc! {})
        .sort(doc! { "id": 1 })
        .await
        .map_err(error::ErrorInternalServerError)?;
    let products = cursor
        .try_collect::<Vec<_>>()
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(products))
}