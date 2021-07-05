// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::str::from_utf8;

use actix_web::{HttpRequest, HttpResponse};

// error: this argument is passed by value, but not consumed in the function
// body
#[allow(clippy::needless_pass_by_value)]
pub fn info(req: HttpRequest) -> HttpResponse {
  let mut easy = curl::easy::Easy::new();

  easy
    .url(&format!(
      "http://www-dynamic.us.worlds.net/cgi-bin/profile.pl?{}",
      qstring::QString::from(req.query_string())
        .get("username")
        .unwrap_or("null"),
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

  HttpResponse::Ok().body(from_utf8(&response).unwrap().to_string())
}
