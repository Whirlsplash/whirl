// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

// Have to use this until https://github.com/rust-num/num-derive/issues/47 gets
// fixed.
#![allow(clippy::use_self)]

#[derive(num_derive::FromPrimitive)]
pub enum Command {
  LongLoc  = 1,
  State,
  Prop,
  ShortLoc,
  RoomChng,
  SessInit,
  SessExit,
  AppInit,
  PropReq  = 10,
  Disappr,
  ApprActr,
  RegObjId,
  Text,
  PropSet,
  PropUpd,
  Whisper,
  Teleport,
  RoomIdRq = 20,
  RoomId,
  Subscrib,
  Unsubscr,
  SubDist,
  Redirect,
  RedirId,
  FingReq,
  FingRep,
  BuddyListUpdate,
  BuddyListNotify,
  Channel,
}
