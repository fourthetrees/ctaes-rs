// basic test of 128-bit AES functionality.
#[macro_use]
extern crate ctaes;

#[test]
fn test1() {
    aes_impl_128!(AES128,1);
    // initialzie test values.
    let key: [u8;16] = [
        0x2b,0x7e,0x15,0x16,0x28,0xae,0xd2,0xa6,
        0xab,0xf7,0x15,0x88,0x09,0xcf,0x4f,0x3c
    ];
    let plain: [u8;16] = [
        0x6b,0xc1,0xbe,0xe2,0x2e,0x40,0x9f,0x96,
        0xe9,0x3d,0x7e,0x11,0x73,0x93,0x17,0x2a
    ];
    let cipher: [u8;16] = [
        0x3a,0xd7,0x7b,0xb4,0x0d,0x7a,0x36,0x60,
        0xa8,0x9e,0xca,0xf3,0x24,0x66,0xef,0x97
    ];

    // initialize buffers.
    let mut encrypt_buff: [u8;16] = [0;16]; // buffer to recieve encrypted data.
    let mut decrypt_buff: [u8;16] = [0;16]; // buffer to recieve decrypted data.

    // initialize context.
    let mut aes = AES128::new();
    // set up the secret/key.
    aes.init(&key);

    // encrypt plain data to buffer.
    aes.encrypt(&mut encrypt_buff,&plain);
    // check that encryption succeeded.
    assert_eq!(encrypt_buff,cipher);

    // decrypt cipher data to buffer.
    aes.decrypt(&mut decrypt_buff,&cipher);
    // check that decryption succeeded.
    assert_eq!(decrypt_buff,plain);
}
