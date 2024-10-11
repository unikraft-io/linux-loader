// Copyright © 2020, Oracle and/or its affiliates.
//
// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//
// Portions Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE-BSD-3-Clause file.
//
// SPDX-License-Identifier: Apache-2.0 AND BSD-3-Clause

/*
 * automatically generated by rust-bindgen using:
 *
 * # bindgen --with-derive-default elf.h > elf.rs
 *
 * From upstream linux include/uapi/linux/elf.h at commit:
 * 48b1320a674e1ff5de2fad8606bee38f724594dc
 * and then edited to eliminate unnecessary definitions, add comments,
 * and relocate definitions and tests for clarity.
 */

pub const PT_LOAD: u32 = 1;
pub const PT_NOTE: u32 = 4;

pub const EI_MAG0: u32 = 0;
pub const EI_MAG1: u32 = 1;
pub const EI_MAG2: u32 = 2;
pub const EI_MAG3: u32 = 3;
pub const EI_DATA: u32 = 5;

pub const ELFMAG0: u32 = 127;

// The values for the following definitions have been edited
// to use their equivalent byte literal representations.
pub const ELFMAG1: u8 = b'E';
pub const ELFMAG2: u8 = b'L';
pub const ELFMAG3: u8 = b'F';

pub const ELFDATA2LSB: u32 = 1;

pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;

pub type Elf64_Addr = __u64;
pub type Elf64_Half = __u16;
pub type Elf64_Off = __u64;
pub type Elf64_Sword = __s32;
pub type Elf64_Word = __u32;
pub type Elf64_Xword = __u64;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct elf64_hdr {
    pub e_ident: [::std::os::raw::c_uchar; 16usize],
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}
pub type Elf64_Ehdr = elf64_hdr;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct elf64_phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}
pub type Elf64_Phdr = elf64_phdr;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct elf64_note {
    pub n_namesz: Elf64_Word,
    pub n_descsz: Elf64_Word,
    pub n_type: Elf64_Word,
}
pub type Elf64_Nhdr = elf64_note;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindgen_test_layout_elf64_hdr() {
        const UNINIT: ::std::mem::MaybeUninit<elf64_hdr> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<elf64_hdr>(),
            64usize,
            concat!("Size of: ", stringify!(elf64_hdr))
        );
        assert_eq!(
            ::std::mem::align_of::<elf64_hdr>(),
            8usize,
            concat!("Alignment of ", stringify!(elf64_hdr))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_ident) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_ident)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_type) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_type)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_machine) as usize - ptr as usize },
            18usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_machine)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_version) as usize - ptr as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_version)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_entry) as usize - ptr as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_entry)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_phoff) as usize - ptr as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_phoff)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_shoff) as usize - ptr as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_shoff)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_flags) as usize - ptr as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_flags)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_ehsize) as usize - ptr as usize },
            52usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_ehsize)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_phentsize) as usize - ptr as usize },
            54usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_phentsize)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_phnum) as usize - ptr as usize },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_phnum)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_shentsize) as usize - ptr as usize },
            58usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_shentsize)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_shnum) as usize - ptr as usize },
            60usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_shnum)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).e_shstrndx) as usize - ptr as usize },
            62usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_hdr),
                "::",
                stringify!(e_shstrndx)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_elf64_phdr() {
        const UNINIT: ::std::mem::MaybeUninit<elf64_phdr> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<elf64_phdr>(),
            56usize,
            concat!("Size of: ", stringify!(elf64_phdr))
        );
        assert_eq!(
            ::std::mem::align_of::<elf64_phdr>(),
            8usize,
            concat!("Alignment of ", stringify!(elf64_phdr))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).p_type) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_phdr),
                "::",
                stringify!(p_type)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).p_flags) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_phdr),
                "::",
                stringify!(p_flags)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).p_offset) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_phdr),
                "::",
                stringify!(p_offset)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).p_vaddr) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_phdr),
                "::",
                stringify!(p_vaddr)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).p_paddr) as usize - ptr as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_phdr),
                "::",
                stringify!(p_paddr)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).p_filesz) as usize - ptr as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_phdr),
                "::",
                stringify!(p_filesz)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).p_memsz) as usize - ptr as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_phdr),
                "::",
                stringify!(p_memsz)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).p_align) as usize - ptr as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_phdr),
                "::",
                stringify!(p_align)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_elf64_note() {
        const UNINIT: ::std::mem::MaybeUninit<elf64_note> = ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<elf64_note>(),
            12usize,
            concat!("Size of: ", stringify!(elf64_note))
        );
        assert_eq!(
            ::std::mem::align_of::<elf64_note>(),
            4usize,
            concat!("Alignment of ", stringify!(elf64_note))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).n_namesz) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_note),
                "::",
                stringify!(n_namesz)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).n_descsz) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_note),
                "::",
                stringify!(n_descsz)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).n_type) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(elf64_note),
                "::",
                stringify!(n_type)
            )
        );
    }
}
