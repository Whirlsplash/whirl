// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use crate::server::net::structure::NetworkProperty;

pub fn find_property_in_property_list(
  property_list: &[NetworkProperty],
  property: i32,
) -> &NetworkProperty {
  property_list
    .iter()
    .find(|i| i.prop_id == property)
    .unwrap()
}
