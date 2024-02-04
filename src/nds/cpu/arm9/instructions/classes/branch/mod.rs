mod instructions;
mod lookup;

pub use lookup::lookup;

fn sign_extend_24_to_32(value: u32) -> i32 {
    let sign_bit = value & (1 << 23);

    let extended_value = if sign_bit != 0 {
        value | 0xFF000000
    } else {
        value & 0x00FFFFFF
    };

    extended_value as i32
}
