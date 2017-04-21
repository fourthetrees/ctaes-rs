// rust bindings for the `ctaes` AES encrption library.
// original c can be found at: `github.com/bitcoin-core/ctaes`
#![no_std]


pub struct AES128 { ctx: AES128_ctx }

impl AES128 {
    pub fn new() -> Self {
        AES128 { ctx: AES128_ctx::new() }
    }
    pub fn init(&mut self, key: &[u8;16]) {
        init_128(&mut self.ctx, key);
    }
    pub fn encrypt(&self, buff: &mut [u8;16], data: &[u8;16]) {
        encrypt_128(&self.ctx, buff, data);
    }
    pub fn decrypt(&self, buff: &mut [u8;16], data: &[u8;16]) {
        decrypt_128(&self.ctx, buff, data);
    }
}


// minimal safe wrapper around init function.
fn init_128(ctx: &mut AES128_ctx, key: &[u8;16]) {
    unsafe { AES128_init(ctx, key.as_ptr()); }
}
// minimal safe wrapper around encrypt function.
fn encrypt_128(ctx: &AES128_ctx, buff: &mut [u8;16], data: &[u8;16]) {
    unsafe { AES128_encrypt(ctx, 1, buff.as_mut_ptr(), data.as_ptr()); }
}
// minimal safe wrapper around decrypt function.
fn decrypt_128(ctx: &AES128_ctx, buff: &mut [u8;16], data: &[u8;16]) {
    unsafe { AES128_decrypt(ctx, 1, buff.as_mut_ptr(), data.as_ptr()); }
}


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


// This is where the actual linking to `ctaes` occurs.
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


// A bare minimum test of the safe interface functions.
#[test]
fn basic_test() {
    // initialzie test values.
    let key: [u8;16] = to_u8(0x2b7e151628aed2a6, 0xabf7158809cf4f3c);
    let plain: [u8;16] = to_u8(0x6bc1bee22e409f96, 0xe93d7e117393172a);
    let cipher: [u8;16] = to_u8(0x3ad77bb40d7a3660, 0xa89ecaf32466ef97);
    // initialize buffers.
    let mut ciphered: [u8;16] = [0;16]; // buffer to recieve encrypted data.
    let mut deciphered: [u8;16] = [0;16]; // buffer to recieve decrypted data.
    let mut ctx = AES128_ctx::new(); // generate the context struct.

    // init_128(context: &mut AES128_ctx, key: &[u8;16])
    init_128(&mut ctx, &key);
    // encrypt_128(cxt: &AES128_ctx, buff: &mut [u8;16], data: &[u8;16])
    encrypt_128(&ctx, &mut ciphered, &plain);
    // decrypt_128(ctx: &AES128_ctx, buff: &mut [u8;16], data: &[u8;16])
    decrypt_128(&ctx, &mut deciphered, &cipher);
    // check that encryption succeeded.
    assert_eq!(ciphered,cipher);
    // check that decryption succeeded.
    assert_eq!(deciphered,plain);
}


// a helper function for the tests.
fn to_u8(a: u64, b: u64) -> [u8;16] {
    let masks: [u64;8] = [
        0xff00000000000000, 0x00ff000000000000,
        0x0000ff0000000000, 0x000000ff00000000,
        0x00000000ff000000, 0x0000000000ff0000,
        0x000000000000ff00, 0x00000000000000ff
    ];
    let mut collector: [u8;16] = [0;16];
    for (i,m) in masks.clone().iter().enumerate() {
        collector[i] = ((a & m) >> ((7 - i) * 8)) as u8;
    }
    for (i,m) in masks.iter().enumerate() {
        collector[i+8] = ((b & m) >> ((7 - i) * 8)) as u8;
    }
    collector
}


// ensure that helper function is splitting u64's as expected.
#[test]
fn helper_test() {
    let a: u64 = 0x00ff00ff00ff00ff;
    let b: u64 = 0x00ff00ff00ff00ff;
    let c: [u8;16] = [
        0x00,0xff,0x00,0xff,0x00,0xff,0x00,0xff,
        0x00,0xff,0x00,0xff,0x00,0xff,0x00,0xff
    ];
    let d: [u8;16] = to_u8(a,b);
    assert_eq!(c,d);
}
