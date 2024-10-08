use crate::cte::cte_t;
use sel4_common::structures::exception_t;

use super::arch::cap_t;

/// This struct is used when finaliseSlot return a value,
///
/// Arguments:
///
/// Status: exit value
///
/// success: Whether the finalising process is successfully
///
/// cleanupInfo: When finalise tcb_cap or cnode_cap, cleanupInfo is zombie_cap, otherwise cleanupInfo is null_cap
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct finaliseSlot_ret {
    pub status: exception_t,
    pub success: bool,
    pub cleanupInfo: cap_t,
}

impl Default for finaliseSlot_ret {
    fn default() -> Self {
        finaliseSlot_ret {
            status: exception_t::EXCEPTION_NONE,
            success: true,
            cleanupInfo: cap_t::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct finaliseCap_ret {
    pub remainder: cap_t,
    pub cleanupInfo: cap_t,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct resolveAddressBits_ret_t {
    pub status: exception_t,
    pub slot: *mut cte_t,
    pub bitsRemaining: usize,
}

impl Default for resolveAddressBits_ret_t {
    #[inline]
    fn default() -> Self {
        resolveAddressBits_ret_t {
            status: exception_t::EXCEPTION_NONE,
            slot: core::ptr::null_mut::<cte_t>(),
            bitsRemaining: 0,
        }
    }
}
