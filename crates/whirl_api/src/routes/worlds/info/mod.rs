// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Serialize, Deserialize)]
pub struct Parameters {
  username: Option<String>,
}

#[allow(clippy::needless_pass_by_value, clippy::unused_async)]
pub async fn info(axum::extract::Query(req): axum::extract::Query<Parameters>) -> &'static str {
  let mut easy = curl::easy::Easy::new();

  easy
    .url(&format!(
      "http://www-dynamic.us.worlds.net/cgi-bin/profile.pl?{}",
      req.username.as_ref().unwrap_or(&"".to_string()),
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

  Box::leak(String::from_utf8(response).unwrap().into_boxed_str())
}
