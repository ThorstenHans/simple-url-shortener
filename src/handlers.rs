use std::collections::HashMap;
use anyhow::Result;
use http::{StatusCode};
use rand::{thread_rng, distributions::Alphanumeric, Rng};
use spin_sdk::{key_value::Store, http::{Request, Response, Params}};

use crate::models::CreateLinkModel;

pub(crate) fn get_all_links(_req: Request, _params: Params) -> Result<Response> {
    let mut all: HashMap<String, String> = HashMap::new();

    let store = Store::open_default()?;
    let keys = store.get_keys()?;
    for key in keys {
        let value = store.get(&key)?;
        all.insert(key, String::from_utf8(value).unwrap_or(String::new()));
    }
    let payload = serde_json::to_string(&all)?;
    Ok(http::Response::builder()
        .status(StatusCode::OK)
        .body(Some(payload.into()))?)
}

pub(crate) fn open_link(req: Request, params: Params) -> Result<Response> {
    match params.get("term") {
        Some(term) => {
            let store = Store::open_default()?;
            if !store.exists(term).unwrap_or(false) {
                return not_found(req, params);
            }
            let value = store.get(term)?;
            Ok(http::Response::builder()
            .status(StatusCode::PERMANENT_REDIRECT)
            .header("Location", value)
            .body(None)?)
        },
        None => not_found(req, params)
    }

}

pub(crate) fn create_short_url(req: Request, _params: Params) -> Result<Response> {
    let Some(body) = &req.body().clone() else {
        return Ok(http::Response::builder().status(StatusCode::BAD_REQUEST).body(None)?);
    };

    let model : CreateLinkModel = serde_json::from_slice(body)?;
    let short = get_short();
    println!("Will store {} as {}", model.url, short);
    let store = Store::open_default()?;

    store.set(&short, model.url.as_bytes())?;
    Ok(http::Response::builder()
        .status(StatusCode::CREATED)
        .header("Location", format!("/{}", short))
        .body(None)?)
}

pub(crate) fn not_found(_req: Request, _params: Params) -> Result<Response> {
    Ok(http::Response::builder().status(StatusCode::NOT_FOUND).body(None)?)
}

/// returns a random string of 5 alphanumeric characters
fn get_short() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}
