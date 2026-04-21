# product-service

This is a Rust app that simulates a product catalog. It is meant to be used in conjunction with the store-front and store-admin apps.

This app is a simple REST API that allows you to get a list of products, get a single product, update a product, and add a product.

Products are persisted in MongoDB. On startup, the service seeds the original product catalog if those products (matched by `id`) do not already exist.

## Running the app locally

The app relies on MongoDB for persistence.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [MongoDB](https://www.mongodb.com/docs/manual/installation/)

### Environment variables

- `MONGODB_URI` (default: `mongodb://127.0.0.1:27017`)
- `MONGODB_DATABASE` (default: `product_service`)
- `MONGODB_COLLECTION` (default: `products`)

### Running the app

To run the app, clone the repo, open a terminal, and navigate to the `product-service` directory. Then run the following command:

```bash
cargo run
```

When the app is running, you should see output similar to the following:

```text
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/product-service`
Listening on http://0.0.0.0:3002
[2023-06-28T02:44:47Z INFO  actix_server::builder] starting 16 workers
[2023-06-28T02:44:47Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
```

Using the [`test-product-service.http`](./test-product-service.http) file in the root of the repo, you can test the API. However, you will need to use VS Code and have the [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) extension installed.