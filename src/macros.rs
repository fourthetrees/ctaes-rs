use super::{AES128_ctx,AES128_init,AES128_encrypt,AES128_decrypt};


// macro to generate size-enforced impls at compile-time.
// woo!!
#[macro_export]
macro_rules! aes_impl_128 {

    ($name: ident, $blocks: expr) => {
        // public wrapper around the C struct definition.
        pub struct $name { context: AES128_ctx }

        // public interface to the C functions.
        impl $name {

            #[inline]
            // create a new instance of the aes context.
            pub fn new() -> Self {
                $name { context: AES128_ctx::new() }
            }

            // initialize the aes instance with a key.
            pub fn init(&mut self, key: &[u8;16]) {
                unsafe { AES128_init(&mut self.context, key.as_ptr()); }
            }

            // encrypt some data to a buffer.
            pub fn encrypt(&self, buff: &mut [u8;{$blocks * 16}], data: &[u8;{$blocks * 16}]) {
                unsafe { AES128_encrypt(&self.context, $blocks, buff.as_mut_ptr(), data.as_ptr()); }
            }

            // decrypt some data to a buffer.
            pub fn decrypt(&self, buff: &mut [u8;{$blocks * 16}], data: &[u8;{$blocks * 16}]) {
                unsafe { AES128_decrypt(&self.context, $blocks, buff.as_mut_ptr(), data.as_ptr()); }
            }
        }
    }
}


#[test]
fn macro_test() {
    aes_impl_128!(AES128,2);
    
    let key: [u8;16] = [0b1010101;16];
    let original: [u8;32] = [0xf0;32];
    let mut ebuf: [u8;32] = [0x00;32];
    let mut dbuf: [u8;32] = [0x00;32];
    
    let mut aes = AES128::new();
    aes.init(&key);
    aes.encrypt(&mut ebuf, &original);
    aes.decrypt(&mut dbuf, &ebuf);
    assert_eq!(dbuf,original);
}