/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-23 00:41:58
 * @LastEditors: goodpeanuts goddpeanuts@foxmail.com
 * @LastEditTime: 2023-12-24 02:56:03
 * @FilePath: /file_encrypt/src/rsa1.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::{pkcs1::DecodeRsaPrivateKey, pkcs1::DecodeRsaPublicKey, pkcs1::EncodeRsaPrivateKey, pkcs1::EncodeRsaPublicKey};
use crate::pem;

pub fn main() {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let dec_data = priv_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);

    let pem1 = RsaPrivateKey::to_pkcs1_pem(&priv_key, rsa::pkcs1::LineEnding::LF).expect("failed to convert to pem");
    println!("{:}", *pem1);
    let pem2 = RsaPublicKey::to_pkcs1_pem(&pub_key, rsa::pkcs1::LineEnding::LF).expect("failed to convert to pem");
    println!("{:}", pem2);
}

pub fn test() {
    let priv_key = RsaPrivateKey::from_pkcs1_pem(pem::PRIV_KEY_E).expect("failed to convert to pem");
    let pub_key = RsaPublicKey::from_pkcs1_pem(pem::PUB_KEY_E).expect("failed to convert to pem");
    let mut rng = rand::thread_rng();
    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);
    println!("{:?}", enc_data);
    // Decrypt
    let dec_data = priv_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("failed to decrypt");
    println!("{:}", String::from_utf8(dec_data).unwrap());
}
