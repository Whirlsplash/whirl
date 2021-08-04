// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

mod structures;

use std::str::from_utf8;

use axum::response;

use crate::routes::worlds::vip::structures::Vip;

#[derive(Serialize, Deserialize)]
pub struct Parameters {
  username: Option<String>,
}

#[allow(clippy::needless_pass_by_value, clippy::unused_async)]
pub async fn vip(
  axum::extract::Query(req): axum::extract::Query<Parameters>,
) -> impl response::IntoResponse {
  let mut easy = curl::easy::Easy::new();
  let mut error = String::new();

  let username = req.username;
  if username.is_none()
    || username
      .as_ref()
      .map_or(false, std::string::String::is_empty)
  {
    error = "no username query parameter provided, defaulting to 'null'".to_string();
  }

  easy
    .url(&format!(
      "http://www-dynamic.us.worlds.net/cgi-bin/vip.pl?Username={}",
      username.unwrap_or_else(|| "null".to_string()),
    ))
    .unwrap();

  let mut response = Vec::new();

  // https://docs.rs/curl/0.4.6/curl/easy/struct.Easy.html
  {
    let mut transfer = easy.transfer();
    transfer
      .write_function(|data| {
        response.extend_from_slice(data);
        Ok(data.len())
      })
      .unwrap();
    transfer.perform().unwrap();
  }

  (
    hyper::StatusCode::OK,
    response::Json(Vip {
      vip:   from_utf8(&response)
        .unwrap()
        .to_string()
        .contains("You're already a VIP!"),
      error: if error.is_empty() { None } else { Some(error) },
    }),
  )
}
