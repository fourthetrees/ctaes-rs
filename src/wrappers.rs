// This module contains the low-level C definitions & FFI
// linkings which are used by the rest of the library.


// -- External Linking -- //

// we need to specify that we want a `static` linking,
// otherwise we can't compile to a single binary.
#[link(name = "ctaes", kind = "static")]
extern {
    // void AES128_init(AES128_ctx* ctx, const unsigned char* key16);
    pub fn AES128_init(ctx: *mut AES128_ctx, key16: *const u8);

    // void AES128_encrypt(const AES128_ctx* ctx, size_t blocks, unsigned char* cipher16, const unsigned char* plain16);
    pub fn AES128_encrypt(ctx: *const AES128_ctx, blocks: usize, cipher16: *mut u8, plain16: *const u8 );

    // void AES128_decrypt(const AES128_ctx* ctx, size_t blocks, unsigned char* plain16, const unsigned char* cipher16);
    pub fn AES128_decrypt(ctx: *const AES128_ctx, blocks: usize, cipher16: *mut u8, plain16: *const u8);


    // void AES192_init(AES192_ctx* ctx, const unsigned char* key16);
    pub fn AES192_init(ctx: *mut AES192_ctx, key24: *const u8);

    // void AES192_encrypt(const AES192_ctx* ctx, size_t blocks, unsigned char* cipher16, const unsigned char* plain16);
    pub fn AES192_encrypt(ctx: *const AES192_ctx, blocks: usize, cipher16: *mut u8, plain16: *const u8 );

    // void AES192_decrypt(const AES192_ctx* ctx, size_t blocks, unsigned char* plain16, const unsigned char* cipher16);
    pub fn AES192_decrypt(ctx: *const AES192_ctx, blocks: usize, cipher16: *mut u8, plain16: *const u8);


    // void AES256_init(AES256_ctx* ctx, const unsigned char* key16);
    pub fn AES256_init(ctx: *mut AES256_ctx, key32: *const u8);

    // void AES256_encrypt(const AES256_ctx* ctx, size_t blocks, unsigned char* cipher16, const unsigned char* plain16);
    pub fn AES256_encrypt(ctx: *const AES256_ctx, blocks: usize, cipher16: *mut u8, plain16: *const u8 );

    // void AES256_decrypt(const AES256_ctx* ctx, size_t blocks, unsigned char* plain16, const unsigned char* cipher16);
    pub fn AES256_decrypt(ctx: *const AES256_ctx, blocks: usize, cipher16: *mut u8, plain16: *const u8);
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
pub struct AES128_ctx {
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


// rust definition for struct defined in `ctaes`.
#[repr(C)] // use C-compatible mem layout.
pub struct AES192_ctx {
    rk: [AES_state;13]
}

impl AES192_ctx {
    // generate a blank/zeroed instance.
    pub fn new() -> Self {
        AES192_ctx {
            rk: [AES_state::new();13]
        }
    }
}


// rust definition for struct defined in `ctaes`.
#[repr(C)] // use C-compatible mem layout.
pub struct AES256_ctx {
    rk: [AES_state;15]
}

impl AES256_ctx {
    // generate a blank/zeroed instance.
    pub fn new() -> Self {
        AES256_ctx {
            rk: [AES_state::new();15]
        }
    }
}


// nothing to see here...
