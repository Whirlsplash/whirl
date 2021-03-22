use std::collections::HashMap;

pub static ROOM_IDS: phf::Map<&'static str, &'static i32> = phf::phf_map! {
"ChatElevator<dimension-1>" => &4,
"IconViewRoom1Enter<dimension-1>" => &3,
"ChatHall<dimension-1>" => &9,
"IconViewRoom1<dimension-1>" => &12,
"IconViewRoom1g<dimension-1>" => &27230,
"ReceptionView1<dimension-1>" => &6,
"ReceptionView2<dimension-1>" => &5,
"Reception<dimension-1>" => &1,
"staircase1<dimension-1>" => &11,
};
