// rust bindings for the `ctaes` AES encrption library.
// original c can be found at: `github.com/bitcoin-core/ctaes`
#![no_std]

// reexport the utils mod.
pub mod utils;

//// Public Api Wrappers ////

// public wrapper around the C struct definition.
pub struct AES128 { ctx: AES128_ctx }

// public interface to the C functions.
impl AES128 {

    #[inline]
    // create a new instance of the aes context.
    pub fn new() -> Self {
        AES128 { ctx: AES128_ctx::new() }
    }

    // initialize the aes instance with a key.
    pub fn init(&mut self, key: &[u8;16]) {
        unsafe { AES128_init(&mut self.ctx, key.as_ptr()); }
    }

    // encrypt some data to a buffer.
    pub fn encrypt(&self, buff: &mut [u8;16], data: &[u8;16]) {
        unsafe { AES128_encrypt(&self.ctx, 1, buff.as_mut_ptr(), data.as_ptr()); }
    }

    // decrypt some data to a buffer.
    pub fn decrypt(&self, buff: &mut [u8;16], data: &[u8;16]) {
        unsafe { AES128_decrypt(&self.ctx, 1, buff.as_mut_ptr(), data.as_ptr()); }
    }
}



//// C Struct Definitions ////

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
struct AES128_ctx {
    rk: [AES_state;11]
}

impl AES128_ctx {
    // generate a blank/zeroed instance.
    pub fn new() -> Self {
        AES128_ctx {
            rk: [AES_state::new();11]
        }
    }
}


//// External Linking ////

// we need to specify that we want a `static` linking,
// otherwise we can't compile to a single binary.
#[link(name = "ctaes", kind = "static")]
extern {
    // void AES128_init(AES128_ctx* ctx, const unsigned char* key16);
    fn AES128_init(ctx: *mut AES128_ctx, key16: *const u8);

    // void AES128_encrypt(const AES128_ctx* ctx, size_t blocks, unsigned char* cipher16, const unsigned char* plain16);
    fn AES128_encrypt(ctx: *const AES128_ctx, blocks: usize, cipher16: *mut u8, plain16: *const u8 );

    // void AES128_decrypt(const AES128_ctx* ctx, size_t blocks, unsigned char* plain16, const unsigned char* cipher16);
    fn AES128_decrypt(ctx: *const AES128_ctx, blocks: usize, cipher16: *mut u8, plain16: *const u8);
}


//// Nothing To See Here ////
