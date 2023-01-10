// Copyright 2022-2023, Offchain Labs, Inc.
// For license information, see https://github.com/nitro/blob/master/LICENSE

extern "C" {
    fn wavm_caller_load8(ptr: usize) -> u8;
    fn wavm_caller_load32(ptr: usize) -> u32;
    fn wavm_caller_store8(ptr: usize, val: u8);
    fn wavm_caller_store32(ptr: usize, val: u32);
}

pub unsafe fn caller_load8(ptr: usize) -> u8 {
    wavm_caller_load8(ptr)
}

pub unsafe fn caller_load32(ptr: usize) -> u32 {
    wavm_caller_load32(ptr)
}

pub unsafe fn caller_store8(ptr: usize, val: u8) {
    wavm_caller_store8(ptr, val)
}

pub unsafe fn caller_store32(ptr: usize, val: u32) {
    wavm_caller_store32(ptr, val)
}

pub unsafe fn caller_load64(ptr: usize) -> u64 {
    let lower = caller_load32(ptr);
    let upper = caller_load32(ptr + 4);
    lower as u64 | ((upper as u64) << 32)
}

pub unsafe fn caller_store64(ptr: usize, val: u64) {
    caller_store32(ptr, val as u32);
    caller_store32(ptr + 4, (val >> 32) as u32);
}

pub unsafe fn write_slice(src: &[u8], ptr: u64) {
    let ptr = usize::try_from(ptr).expect("pointer doesn't fit in usize");
    write_slice_usize(src, ptr)
}

pub unsafe fn write_slice_usize(mut src: &[u8], mut ptr: usize) {
    while src.len() >= 4 {
        let mut arr = [0u8; 4];
        arr.copy_from_slice(&src[..4]);
        caller_store32(ptr, u32::from_le_bytes(arr));
        ptr += 4;
        src = &src[4..];
    }
    for &byte in src {
        caller_store8(ptr, byte);
        ptr += 1;
    }
}

pub unsafe fn read_slice(ptr: u64, len: u64) -> Vec<u8> {
    let ptr = usize::try_from(ptr).expect("pointer doesn't fit in usize");
    let len = usize::try_from(len).expect("length doesn't fit in usize");
    read_slice_usize(ptr, len)
}

pub unsafe fn read_slice_usize(mut ptr: usize, mut len: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(len);
    if len == 0 {
        return data;
    }
    while len >= 4 {
        data.extend(caller_load32(ptr).to_le_bytes());
        ptr += 4;
        len -= 4;
    }
    for _ in 0..len {
        data.push(caller_load8(ptr));
        ptr += 1;
    }
    data
}