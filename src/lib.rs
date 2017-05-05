// rust bindings for the `ctaes` AES encrption library.
// original c can be found at: `github.com/bitcoin-core/ctaes`
#![no_std]
// reexport the utils mod.
pub mod utils;


// The `Aes` trait is shared by all aes instances.  The
// only significant difference between implementations is
// the key size used (E.g.; &[u8;16] for Aes128).
pub trait Aes {
    // Type of key used to perform encryption.
    // This will generally be an array of `u8`s.
    type Key;

    // Construct an Aes context with the supplied key.
    #[inline] // Politely suggest that this operation be inlined.
    fn init(&Key) -> Self;

    // Raw encryption function.  If called with the wrong block-size,
    // this can lead to out of bounds reads/writes.  Use the
    // `set_block_size` macro to generate a safe wrapper.
    unsafe fn encrypt_unsafe(&self, usize, &[u8], &mut [u8]);

    // Raw decryption function.  If called with the wrong block-size,
    // this can lead to out of bounds reads/writes.  Use the
    // `set_block_count!` macro to generate a safe wrapper.
    unsafe fn decrypt_unsafe(&self, usize, &[u8], &mut [u8]);
}


// Use `set_block_count!` to generate safe size-enforing wrappers
// for encryption and decryption.  Each block is 128 bits; E.g.;
// a block-count of one means `[u8;16]`, block count of two means
// `[u8;32]`, and so on.
#[macro_export]
macro_rules! set_block_count {
    ($name: ident, $blocks: expr) => {
        impl $name {
            // safe size-enforcing wrapper around `self.encrypt_unsafe`.
            pub fn encrypt(&self, data: &[u8;{$blocks * 16}], buff: &mut [u8;{$blocks * 16}]) {
                unsafe { self.encrypt_unsafe($blocks, data, buff); }
            }

            // safe size-enforcing wrapper around `self.decrypt_unsafe`.
            pub fn decrypt(&self, data: &[u8;{$blocks * 16}], buff: &mut [u8;{$blocks * 16}]) {
                unsafe { self.decrypt_unsafe($blocks, data, buff) }
            }
        }
    }
}
