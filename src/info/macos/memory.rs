use super::*;
use std::{ffi::c_int, mem::size_of, ptr::null};

use libc::{
    c_void, host_info_t, host_statistics64, mach_host_self, mach_msg_type_number_t, sysctl,
    vm_statistics64_data_t, CTL_HW, HOST_VM_INFO64, HOST_VM_INFO64_COUNT, HW_MEMSIZE, HW_PAGESIZE,
    KERN_SUCCESS,
};

pub fn memory_info() -> Result<Memory, InfoError> {
    // Get the size of each page
    let mut pagesize: usize = 0;
    let mut mib: [c_int; 2] = [CTL_HW, HW_PAGESIZE];
    if unsafe {
        sysctl(
            &mut mib as *mut c_int,
            2,
            &mut pagesize as *mut usize as *mut c_void,
            &mut size_of::<usize>(),
            null::<usize>() as *mut c_void,
            0,
        )
    } < 0
    {
        return Err(InfoError::Sysctl {
            name: "hw.pagesize".to_string(),
        });
    }

    // Get the total pysical memory in bytes
    let mut memsize: u64 = 0;
    let mut mib: [c_int; 2] = [CTL_HW, HW_MEMSIZE];
    if unsafe {
        sysctl(
            &mut mib as *mut c_int,
            2,
            &mut memsize as *mut u64 as *mut c_void,
            &mut size_of::<u64>(),
            null::<usize>() as *mut c_void,
            0,
        )
    } < 0
    {
        return Err(InfoError::Sysctl {
            name: "hw.memsize".to_string(),
        });
    }

    // Get free pages
    let mut count: mach_msg_type_number_t = HOST_VM_INFO64_COUNT;
    let mut vm_stat: vm_statistics64_data_t = vm_statistics64_data_t {
        free_count: 0,
        active_count: 0,
        inactive_count: 0,
        wire_count: 0,
        zero_fill_count: 0,
        reactivations: 0,
        pageins: 0,
        pageouts: 0,
        faults: 0,
        cow_faults: 0,
        lookups: 0,
        hits: 0,
        purges: 0,
        purgeable_count: 0,
        speculative_count: 0,
        decompressions: 0,
        compressions: 0,
        swapins: 0,
        swapouts: 0,
        compressor_page_count: 0,
        throttled_count: 0,
        external_page_count: 0,
        internal_page_count: 0,
        total_uncompressed_pages_in_compressor: 0,
    };

    if unsafe {
        host_statistics64(
            mach_host_self(),
            HOST_VM_INFO64,
            &mut vm_stat as *mut vm_statistics64_data_t as host_info_t,
            &mut count,
        ) != KERN_SUCCESS
    } {
        return Err(InfoError::General("Failed to get VM stats".to_string()));
    }

    let free_pages = vm_stat.free_count as u64;
    let available = free_pages * pagesize as u64;
    let total = memsize;

    Ok(Memory {
        total,
        available,
        used: total - available,
    })
}
