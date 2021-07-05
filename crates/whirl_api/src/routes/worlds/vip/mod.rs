// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

mod structures;

use std::str::from_utf8;

use actix_web::{HttpRequest, HttpResponse};

use crate::routes::worlds::vip::structures::Vip;

// error: this argument is passed by value, but not consumed in the function
// body
#[allow(clippy::needless_pass_by_value)]
pub fn vip(req: HttpRequest) -> HttpResponse {
  let queries = qstring::QString::from(req.query_string());
  let mut easy = curl::easy::Easy::new();
  let mut error = String::new();

  let username = queries.get("username");
  if username.is_none() || username.map_or(false, str::is_empty) {
    error = "no username query parameter provided, defaulting to 'null'".to_string();
  }

  easy
    .url(&format!(
      "http://www-dynamic.us.worlds.net/cgi-bin/vip.pl?Username={}",
      username.unwrap_or("null"),
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

  HttpResponse::Ok().json(Vip {
    vip:   from_utf8(&response)
      .unwrap()
      .to_string()
      .contains("You're already a VIP!"),
    error: if error.is_empty() { None } else { Some(error) },
  })
}
