// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2024 SUSE LLC
//
// Author: Roy Hopkins <roy.hopkins@suse.com>

use std::mem::size_of;

use bootlib::kernel_launch::Stage2LaunchInfo;
use igvm::IgvmDirectiveHeader;
use igvm_defs::{IgvmPageDataFlags, IgvmPageDataType, PAGE_SIZE_4K};
use zerocopy::AsBytes;

use crate::gpa_map::GpaMap;

pub struct Stage2Stack {
    stage2_stack: Stage2LaunchInfo,
}

const _: () = assert!((size_of::<Stage2Stack>() as u64) <= PAGE_SIZE_4K);

impl Stage2Stack {
    pub fn new(gpa_map: &GpaMap, vtom: u64) -> Self {
        let stage2_stack = Stage2LaunchInfo {
            kernel_elf_start: gpa_map.kernel_elf.get_start() as u32,
            kernel_elf_end: (gpa_map.kernel_elf.get_start() + gpa_map.kernel_elf.get_size()) as u32,
            kernel_fs_start: gpa_map.kernel_fs.get_start() as u32,
            kernel_fs_end: (gpa_map.kernel_fs.get_start() + gpa_map.kernel_fs.get_size()) as u32,
            igvm_params: gpa_map.igvm_param_block.get_start() as u32,
            vtom,
            padding: 0,
        };
        Self { stage2_stack }
    }

    pub fn add_directive(
        &self,
        gpa: u64,
        compatibility_mask: u32,
        directives: &mut Vec<IgvmDirectiveHeader>,
    ) {
        let stage2_stack_data = self.stage2_stack.as_bytes();
        let mut stage2_stack_page = vec![0u8; PAGE_SIZE_4K as usize - stage2_stack_data.len()];
        stage2_stack_page.extend_from_slice(stage2_stack_data);

        directives.push(IgvmDirectiveHeader::PageData {
            gpa,
            compatibility_mask,
            flags: IgvmPageDataFlags::new(),
            data_type: IgvmPageDataType::NORMAL,
            data: stage2_stack_page,
        });
    }
}
