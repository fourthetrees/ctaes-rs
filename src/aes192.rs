// This module contains wrappers for 192-bit aes
// encryption.

use super::Aes;

// Rustic handle for the underlying
// `AES192_ctx` C struct.
pub struct Aes192 {
    context: AES192_ctx
}

// Basic public interface.
impl Aes for Aes192 {
    // fixed-size secret key.
    type Key = [u8;24];

    // Construct an Aes context with the supplied key.
    #[inline] // Politely suggest that this operation be inlined.
    fn init(key: &Self::Key) -> Self {
        // generate a blank aes context.
        let mut ctx = AES192_ctx::new();
        // initialize the context with the supplied key.
        unsafe { AES192_init(&mut ctx, key.as_ptr()); }
        // return public handle to the context.
        Aes192 { context: ctx }
    }

    // Raw encryption function.  If called with the wrong block-size,
    // this can lead to out of bounds reads/writes.  Use the
    // `set_block_size` macro to generate a safe wrapper.
    unsafe fn encrypt_unsafe(&self, blocks: usize, data: &[u8], buff: &mut [u8]) {
        AES192_encrypt(&self.context, blocks, buff.as_mut_ptr(), data.as_ptr());
    }

    // Raw decryption function.  If called with the wrong block-size,
    // this can lead to out of bounds reads/writes.  Use the
    // `set_block_count!` macro to generate a safe wrapper.
    unsafe fn decrypt_unsafe(&self, blocks: usize, data: &[u8], buff: &mut [u8]) {
        AES192_decrypt(&self.context, blocks, buff.as_mut_ptr(), data.as_ptr());
    }
}


// -- C Struct Definitions  -- //

// rust definition for struct defined in `ctaes`.
#[repr(C)] // use C-compatible mem layout.
#[derive(Copy,Clone)]
struct AES_state {
    slice: [u16;8] // uint16_t slice[8]
}

impl AES_state {
    // generate a new black/zeroed instance.
    fn new() -> Self {
        AES_state {
            slice: [0;8]
        }
    }
}


// rust definition for struct defined in `ctaes`.
#[repr(C)] // use C-compatible mem layout.
struct AES192_ctx {
    rk: [AES_state;13]
}

impl AES192_ctx {
    // generate a blank/zeroed instance.
    fn new() -> Self {
        AES192_ctx {
            rk: [AES_state::new();13]
        }
    }
}


// -- External Linking -- //

// we need to specify that we want a `static` linking,
// otherwise we can't compile to a single binary.
#[link(name = "ctaes", kind = "static")]
extern {
    // void AES192_init(AES192_ctx* ctx, const unsigned char* key16);
    fn AES192_init(ctx: *mut AES192_ctx, key24: *const u8);

    // void AES192_encrypt(const AES192_ctx* ctx, size_t blocks, unsigned char* cipher16, const unsigned char* plain16);
    fn AES192_encrypt(ctx: *const AES192_ctx, blocks: usize, cipher16: *mut u8, plain16: *const u8 );

    // void AES192_decrypt(const AES192_ctx* ctx, size_t blocks, unsigned char* plain16, const unsigned char* cipher16);
    fn AES192_decrypt(ctx: *const AES192_ctx, blocks: usize, cipher16: *mut u8, plain16: *const u8);
}
