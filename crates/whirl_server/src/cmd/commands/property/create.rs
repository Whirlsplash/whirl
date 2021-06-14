// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

// TODO: of2m-ify?

use whirl_config::Config;

use crate::{
  cmd::constants::Command,
  net::{
    constants::{
      VAR_APPNAME,
      VAR_CHANNEL,
      VAR_ERROR,
      VAR_EXTERNAL_HTTP_SERVER,
      VAR_MAIL_DOMAIN,
      VAR_PRIV,
      VAR_PROTOCOL,
      VAR_SCRIPT_SERVER,
      VAR_SERIAL,
      VAR_SERVERTYPE,
      VAR_SMTP_SERVER,
      VAR_UPDATETIME,
    },
    network_property::NetworkProperty,
    property_list::PropertyList,
  },
};

pub fn property_update_as_distributor() -> Vec<u8> {
  PropertyList(vec![
    NetworkProperty {
      prop_id: VAR_MAIL_DOMAIN,
      value:   "worlds3d.com".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_SMTP_SERVER,
      value:   "mail.worlds.net:25".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_SCRIPT_SERVER,
      value:   "http://www-dynamic.us.worlds.net/cgi-bin".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_EXTERNAL_HTTP_SERVER,
      value:   "http://www-static.us.worlds.net".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_SERVERTYPE,
      value:   "1".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_PROTOCOL,
      value:   "24".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_APPNAME,
      value:   Config::get().whirlsplash.worldsmaster_username,
    },
  ])
  .as_bytes(Command::PropUpd as i32, 0xFF)
}

pub fn property_update_as_hub() -> Vec<u8> {
  PropertyList(vec![
    NetworkProperty {
      prop_id: VAR_UPDATETIME,
      value:   "1000000".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_MAIL_DOMAIN,
      value:   "worlds3d.com".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_SMTP_SERVER,
      value:   "mail.worlds.net:25".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_SCRIPT_SERVER,
      value:   "http://www-dynamic.us.worlds.net/cgi-bin".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_EXTERNAL_HTTP_SERVER,
      value:   "http://www-static.us.worlds.net".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_SERVERTYPE,
      value:   "3".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_PROTOCOL,
      value:   "24".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_APPNAME,
      value:   Config::get().whirlsplash.worldsmaster_username,
    },
  ])
  .as_bytes(Command::PropUpd as i32, 0xFF)
}

pub fn property_request_as_distributor() -> Vec<u8> {
  PropertyList(vec![
    NetworkProperty {
      prop_id: VAR_ERROR,
      value:   "0".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_APPNAME,
      value:   Config::get().whirlsplash.worldsmaster_username,
    },
    NetworkProperty {
      prop_id: VAR_PROTOCOL,
      value:   "24".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_SERVERTYPE,
      value:   "1".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_SERIAL,
      value:   "DWLV000000000000".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_PRIV,
      value:   "0".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_CHANNEL,
      value:   "dimension-1".to_string(),
    },
  ])
  .as_bytes(Command::SessInit as i32, 0x01)
}

pub fn property_request_as_hub() -> Vec<u8> {
  PropertyList(vec![
    NetworkProperty {
      prop_id: VAR_ERROR,
      value:   "0".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_SERVERTYPE,
      value:   "3".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_UPDATETIME,
      value:   "1000000".to_string(),
    },
    NetworkProperty {
      prop_id: VAR_PROTOCOL,
      value:   "24".to_string(),
    },
  ])
  .as_bytes(Command::SessInit as i32, 0x01)
}
