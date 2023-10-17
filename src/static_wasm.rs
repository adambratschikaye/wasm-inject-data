#![feature(asm_experimental_arch)]

fn main() {}

/// Exported function can be called to check we have read the expected data from
/// the injected data segment.
#[no_mangle]
extern "C" fn foo() {
    let a = get_passive_data_segment();

    assert_eq!(a, vec![1, 2, 3, 4, 5]);
}

/// This method will be modified by the tooling when injecting the passive data
/// segment to return the correct size.
#[inline(never)]
#[no_mangle]
extern "C" fn passive_data_size() -> usize {
    0
}

#[cfg(target_arch = "wasm32")]
fn get_passive_data_segment() -> Vec<u8> {
    // Create a vector of the required size.
    let size = passive_data_size();
    let v = vec![0; size];
    let addr = v.as_ptr();
    // Execute a `memory.init` instruction which copies data from the passive
    // data segment to main memory at the address of our vector.
    unsafe {
        std::arch::asm!(
            "local.get {addr}",
            "i32.const 0",
            "local.get {size}",
            "memory.init 1, 0",
            addr = in(local) addr,
            size = in(local) size,
        );
    }
    v
}

#[cfg(not(target_arch = "wasm32"))]
fn get_passive_data_segment() -> Vec<u8> {
    vec![0; passive_data_size()]
}
