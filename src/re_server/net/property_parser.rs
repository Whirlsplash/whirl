use std::str::from_utf8;

use crate::re_server::net::structure::NetworkProperty;

/// Iterate over a network property in the form of bytes and return a list of
/// human-readable properties.
pub fn parse_network_property(mut data: Vec<u8>) -> Vec<NetworkProperty> {
  let mut property_list = vec![];

  // Iterate over all network properties
  loop {
    // Check if any commands are present
    if data.len() <= 2 {
      break;
    }
    debug!("iteration: {:?}", data);
    // if data[0] == 0 {
    //   break;
    // }

    let property_length = data[1] + 2;
    property_list.push(NetworkProperty {
      prop_id: data[0] as i32,
      value:   from_utf8(&data[2..data[1] as usize + 2])
        .unwrap()
        .to_string(),
    });

    // Remove current property from the network property
    data = data[property_length as usize..].to_vec();
  }

  // Return the human-readable network property
  property_list
}
