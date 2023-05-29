// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;
use base::{CFAllocatorRef, CFTypeID, CFIndex, CFRange, CFOptionFlags};

#[repr(C)]
pub struct __CFData(c_void);

pub type CFDataRef = *const __CFData;
pub type CFDataSearchFlags = CFOptionFlags;

// typedef CF_OPTIONS(CFOptionFlags, CFDataSearchFlags)
pub const kCFDataSearchBackwards: CFDataSearchFlags = 1usize << 0;
pub const kCFDataSearchAnchored: CFDataSearchFlags = 1usize << 1;

extern {
    /*
     * CFData.h
     */

    /* Creating a CFData Object */
    pub fn CFDataCreate(allocator: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFDataRef;
    pub fn CFDataCreateCopy(allocator: CFAllocatorRef, theData: CFDataRef) -> CFDataRef;
    pub fn CFDataCreateWithBytesNoCopy(allocator: CFAllocatorRef, bytes: *const u8, length: CFIndex, bytesDeallocator: CFAllocatorRef) -> CFDataRef;

    /* Examining a CFData Object */
    pub fn CFDataGetBytePtr(theData: CFDataRef) -> *const u8;
    pub fn CFDataGetBytes(theData: CFDataRef, range: CFRange, buffer: *mut u8);
    pub fn CFDataGetLength(theData: CFDataRef) -> CFIndex;
    pub fn CFDataFind(theData: CFDataRef, dataToFind: CFDataRef, searchRange: CFRange, compareOptions: CFDataSearchFlags) -> CFRange;

    /* Getting the CFData Type ID */
    pub fn CFDataGetTypeID() -> CFTypeID;
}
