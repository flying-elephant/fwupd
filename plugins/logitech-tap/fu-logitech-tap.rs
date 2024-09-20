/*
 * Copyright 1999-2023 Logitech, Inc.
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 */

 #[repr(u8)]
 enum FuStructLogitechTapSensorHidSetCmd {
     // put device into suspend/operational mode
     Tde = 0x1A,
     Reboot = 0x1A,
     Version = 0x1B,
     SerialNumber = 0x1C,
 }
 
 #[repr(u8)]
 enum FuStructLogitechTapSensorHidGetCmd {
     Version = 0x19,
     SerialNumber = 0x1D,
 }
 
 #[derive(New)]
 struct FuStructLogitechTapSensorHidReq {
     cmd: FuStructLogitechTapSensorHidSetCmd,
      //payload goes here
      payload_byte1: u8,
      payload_byte2: u8,
      payload_byte3: u8 == 0x,
      payload_byte4: u8 == 0x,
 }
 
 #[derive(New)]
 struct FuStructLogitechTapSensorHidRes {
     cmd: FuStructLogitechTapSensorHidGetCmd,
      //payload goes here
      payload_byte1: u8,
      payload_byte2: u8,
      payload_byte3: u8 == 0x,
      payload_byte4: u8 == 0x,
 }
 