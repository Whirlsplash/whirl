use crate::{
  config::get_config,
  re_server::{
    cmd::constants::{PROPUPD, SESSINIT},
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
      converter::property_list_to_bytes,
      structure::NetworkProperty,
    },
  },
};

pub fn create_property_update_as_distributor() -> Vec<u8> {
  property_list_to_bytes(
    PROPUPD,
    0xFF,
    vec![
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
        value:   get_config().unwrap().worldsmaster_username,
      },
    ],
  )
}

pub fn create_property_update_as_hub() -> Vec<u8> {
  property_list_to_bytes(
    PROPUPD,
    0xFF,
    vec![
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
        value:   get_config().unwrap().worldsmaster_username,
      },
    ],
  )
}

pub fn create_property_request_as_distributor() -> Vec<u8> {
  property_list_to_bytes(
    SESSINIT as i32,
    0x01,
    vec![
      NetworkProperty {
        prop_id: VAR_ERROR,
        value:   "0".to_string(),
      },
      NetworkProperty {
        prop_id: VAR_APPNAME,
        value:   get_config().unwrap().worldsmaster_username,
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
    ],
  )
}

pub fn create_property_request_as_hub() -> Vec<u8> {
  property_list_to_bytes(
    SESSINIT as i32,
    0x01,
    vec![
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
    ],
  )
}