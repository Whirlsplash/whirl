use crate::re_server::net::structure::NetworkProperty;

pub fn find_property_in_property_list(
  property_list: &[NetworkProperty],
  property: i32,
) -> &NetworkProperty {
  property_list
    .iter()
    .find(|i| i.prop_id == property)
    .unwrap()
}
