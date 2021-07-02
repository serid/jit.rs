fn main() {
    // Machine code that returns int value 13
    let bytes = [0xb8, 0x0d, 0x00, 0x00, 0x00, 0xc3];

    // Execute code stored in `bytes`
    let r = unsafe { jit_rs::execute_bytes::<i32>(&bytes) };

    // Prints 13
    println!("Hello, world! {}", r);
}