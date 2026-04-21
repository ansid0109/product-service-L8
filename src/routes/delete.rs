use crate::model::ProductInfo;
use crate::startup::AppState;
use actix_web::{error, web, Error, HttpResponse};
use mongodb::bson::doc;

pub async fn delete_product(
    data: web::Data<AppState>,
    path: web::Path<ProductInfo>,
) -> Result<HttpResponse, Error> {
    let result = data
        .products_collection
        .delete_one(doc! { "id": path.product_id })
        .await
        .map_err(error::ErrorInternalServerError)?;

    if result.deleted_count == 0 {
        Ok(HttpResponse::NotFound().body("Product not found"))
    } else {
        Ok(HttpResponse::Ok().body(""))
    }
}