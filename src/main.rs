use serde::{Serialize, Deserialize};
use tide::prelude::*;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
struct State {
    jobqueue: Arc<RwLock<Vec<Order>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Order {
    name: String,
    number: u8,
}

#[async_std::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    let state = State {
        jobqueue: Arc::new(RwLock::new(Vec::new())),
    };
    tide::log::start();
    let mut app = tide::with_state(state);
    app.at("/").get(process_get);
    app.at("/").post(process_post);
    app.at("/list").get(process_list);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn process_post(mut req: tide::Request<State>) -> tide::Result {
    let order: Order = req.body_json().await?;
    req.state().jobqueue.write().unwrap().push(order);
    Ok(tide::Response::new(200))
}

async fn process_get(mut req: tide::Request<State>) -> tide::Result {
    let orderqueue = &mut *req.state().jobqueue.write().unwrap();
    let order = orderqueue.pop();
    match order {
        Some(order) => Ok(tide::Response::builder(200)
                            .body(tide::Body::from_json(&order)?)
                            .build()),
        None => Ok(tide::Response::builder(204)
                .build()),
    }
}

async fn process_list(req: tide::Request<State>) -> tide::Result {
    let orderqueue = &*req.state().jobqueue.read().unwrap();
    Ok(tide::Response::builder(200)
       .body(tide::Body::from_json(&orderqueue)?)
       .build())
}
