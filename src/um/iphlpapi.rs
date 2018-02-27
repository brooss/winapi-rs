// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Internet Protocol Helper Interface Prototypes and structure definitions
use ctypes::{c_char};
use shared::minwindef::{BOOL, BYTE, DWORD, PULONG, UINT};

pub const MAX_ADAPTER_ADDRESS_LENGTH: usize = 8;
pub const MAX_ADAPTER_DESCRIPTION_LENGTH: usize = 128;
pub const MAX_ADAPTER_NAME_LENGTH: usize = 256;

#[cfg(target_arch = "x86")]
pub type time_t = u32;
#[cfg(target_arch = "x86_64")]
pub type time_t = u64;


STRUCT!{struct IP_ADDRESS_STRING{
    String: [c_char; 4*4],
}}

pub type PIP_ADDRESS_STRING = *mut IP_ADDRESS_STRING;
pub type IP_MASK_STRING = IP_ADDRESS_STRING;
pub type PIP_MASK_STRING = *mut IP_MASK_STRING;

STRUCT!{struct IP_ADDR_STRING{
    Next: *mut IP_ADDR_STRING,
    IpAddress: IP_ADDRESS_STRING,
    IpMask: IP_MASK_STRING,
    Context: DWORD,
}}

pub type PIP_ADDR_STRING = *mut IP_ADDR_STRING;

STRUCT!{struct IP_ADAPTER_INFO {
    Next: *mut IP_ADAPTER_INFO,
    ComboIndex: DWORD,
    AdapterName: [c_char; MAX_ADAPTER_NAME_LENGTH+4],
    Description: [c_char; MAX_ADAPTER_DESCRIPTION_LENGTH + 4],
    AddressLength: UINT,
    Address: [BYTE; MAX_ADAPTER_ADDRESS_LENGTH],
    Index: DWORD,
    Type: UINT,
    DhcpEnabled: UINT,
    CurrentIpAddress: PIP_ADDR_STRING,
    IpAddressList: IP_ADDR_STRING,
    GatewayList: IP_ADDR_STRING,
    DhcpServer: IP_ADDR_STRING,
    HaveWins: BOOL,
    PrimaryWinsServer: IP_ADDR_STRING,
    SecondaryWinsServer: IP_ADDR_STRING,
    LeaseObtained: time_t,
    LeaseExpires: time_t,
}}

pub type PIP_ADAPTER_INFO = *mut IP_ADAPTER_INFO;

extern "system" {
	pub fn GetAdaptersInfo(
        pAdapterInfo: PIP_ADAPTER_INFO, pOutBufLen: PULONG
    )-> DWORD;
}
