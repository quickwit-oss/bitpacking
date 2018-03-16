const BLOCK_LEN: usize = 32;

type DataType = u32;


fn set1(el: i32) -> DataType {
    el as u32
}

fn right_shift_32(el: DataType, shift: i32) -> DataType {
    el >> shift
}

fn left_shift_32(el: DataType, shift: i32) -> DataType {
    el << shift
}

fn op_or(left: DataType, right: DataType) -> DataType {
    left | right
}

fn op_and(left: DataType, right: DataType) -> DataType {
    left & right
}

unsafe fn load_unaligned(addr: *const DataType) -> DataType {
    *addr
}

unsafe fn store_unaligned(addr: *mut DataType, data: DataType) {
    *addr = data;
}

fn delta(next: DataType, prev: DataType) -> DataType {
    next - prev
}

fn or_collapse_to_u32(accumulator: DataType) -> u32 {
    accumulator
}

declare_bitpacker!(ScalarBitPacker);
