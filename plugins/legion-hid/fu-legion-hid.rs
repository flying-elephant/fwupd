// Copyright 2024 Mario Limonciello <superm1@gmail.com>
// SPDX-License-Identifier: LGPL-2.1-or-later

#[derive(Getters, New)]
struct FuStructLegionHidReqDeviceVersion {
    command1: u8 == 0x08,
    command2: u8 == 0x25,
    ver: [u8; 11],
    _reserved: [u8; 498] = 0xFF,
    end: u8 == 0x00,
}

#[derive(Getters, New, Validate)]
struct FuStructLegionHidResDeviceVersion {
    command1: u8 == 0x09,
    command2: u8 == 0x25,
    ver: [u8; 11],
    _reserved: [u8; 498] = 0xFF,
    end: u8 == 0x00,
}

#[repr(u8)]
enum FuLegionHidReportId {
    Input = 0x4,
    Output = 0x5,
}

#[repr(u32)]
enum FuLegionHidCommand {
    IapStart = 0x50,
    IapGetMaxSize = 0x51,
    IapWriteRsp = 0x52,
    IapVerify = 0x53,
}

[derive(New, Setters, Getters)]
struct FuStructLegionIapHeader {
    report_id: u8,
    protocol_len: u8,
    protocol_id: u8,
    update_sub_id: u8,
    update_dev: u8,
    update_param: u8,
    update_cmd: u8,
}

[derive(New, Setters, Getters)]
struct FuStructLegionIapStartCmd {
    header: FuStructLegionIapHeader,
    update_cmd: u8 == 0x50,
    reserved: u8 == 0,
    crc16: u16,
    size: u32,
    update_data: [u8; 50],
}

[derive(New, Setters, Getters)]
struct FuStructLegionIapGetMaxSizeCmd {
    header: FuStructLegionIapHeader,
    update_cmd: u8 == 0x51,
    reserved: u8 == 0,
    crc16: u16,
    size: u32,
    update_data: [u8; 50],
}