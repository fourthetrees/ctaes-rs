// This module contains wrappers for 128-bit aes
// encryption.

use super::Aes;
use super::wrappers::*;

// Rustic handle for the underlying
// `AES128_ctx` C struct.
pub struct Aes128 {
    context: AES128_ctx
}

// Basic public interface.
impl Aes for Aes128 {
    // fixed-size secret key.
    type Key = [u8;16];

    // Construct an Aes context with the supplied key.
    #[inline] // Politely suggest that this operation be inlined.
    fn init(key: &Self::Key) -> Self {
        // generate a blank aes context.
        let mut ctx = AES128_ctx::new();
        // initialize the context with the supplied key.
        unsafe { AES128_init(&mut ctx, key.as_ptr()); }
        // return public handle to the context.
        Aes128 { context: ctx }
    }

    // Raw encryption function.  If called with the wrong block-size,
    // this can lead to out of bounds reads/writes.  Use the
    // `set_block_size` macro to generate a safe wrapper.
    unsafe fn encrypt_unsafe(&self, blocks: usize, data: &[u8], buff: &mut [u8]) {
        AES128_encrypt(&self.context, blocks, buff.as_mut_ptr(), data.as_ptr());
    }

    // Raw decryption function.  If called with the wrong block-size,
    // this can lead to out of bounds reads/writes.  Use the
    // `set_block_count!` macro to generate a safe wrapper.
    unsafe fn decrypt_unsafe(&self, blocks: usize, data: &[u8], buff: &mut [u8]) {
        AES128_decrypt(&self.context, blocks, buff.as_mut_ptr(), data.as_ptr());
    }
}


// nothing to see here...
