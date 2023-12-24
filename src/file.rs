/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-24 01:24:32
 * @LastEditors: goodpeanuts goodpeanuts@foxmail.com
 * @LastEditTime: 2023-12-25 02:24:09
 * @FilePath: \file-cryption\src\file.rs
 * @Description: 使用随机生成的aes256密钥对文件内容的加密解密，使用公钥密码实现对文件fek， ddf， drf， 
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */

use crate::{cbc, pem};
use crypto::aes;
use rsa::{pkcs1::DecodeRsaPrivateKey, pkcs1::DecodeRsaPublicKey};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

// 指定固定的密钥和IV，实际应用中需要使用安全的随机生成的密钥和IV
const IV: &[u8] = b"0123456789abcdef";

#[derive(Serialize, Deserialize)]
pub struct EncryptedFile {
    ddf: Vec<u8>,
    drf: Vec<u8>,
    encrypted_data: Vec<u8>,
}

pub fn encrypt_file(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let fek = cbc::generate_aes256_key();
    let pub_key = RsaPublicKey::from_pkcs1_pem(pem::PUB_KEY_E).expect("failed to convert to pem");
    let pub_key_root =
        RsaPublicKey::from_pkcs1_pem(pem::PUB_KEY_ROOT).expect("failed to convert to pem");
    let mut rng = rand::thread_rng();
    let ddf = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &fek[..])
        .expect("failed to encrypt");
    let drf = pub_key_root
        .encrypt(&mut rng, Pkcs1v15Encrypt, &fek[..])
        .expect("failed to encrypt");

    let encrypted_data = cbc::aes_cbc_encrypt(&data, aes::KeySize::KeySize256, &fek, IV);

    let json = serde_json::to_string(&EncryptedFile {
        ddf: ddf,
        drf: drf,
        encrypted_data: encrypted_data,
    });

    let mut file = File::create("10e.txt").unwrap();
    file.write_all(json.unwrap().as_bytes()).unwrap();
}

pub fn decrypt_file(filename: &str, key: &str) -> Result<String, String> {
    let mut path = PathBuf::from("ciphertext");
    path.push(filename);
    let file = std::fs::File::open(path);
    match file {
        Ok(file) => {
            let encrypted_file: EncryptedFile;
            match serde_json::from_reader(file) {
                Ok(c) => encrypted_file = c,
                Err(_) => {
                    return Err("不是加密文件".to_string());
                }
            }
            let priv_key;
            match RsaPrivateKey::from_pkcs1_pem(key) {
                Ok(c) => priv_key = c,
                Err(_) => {
                    return Err("无权限访问".to_string());
                }
            }
            let fek;
            match priv_key.decrypt(Pkcs1v15Encrypt, &encrypted_file.ddf) {
                Ok(c) => fek = c,
                Err(_) => {
                    return Err("无权限访问".to_string());
                }
            }
            let data = cbc::aes_cbc_decrypt(
                &encrypted_file.encrypted_data,
                aes::KeySize::KeySize256,
                &fek,
                IV,
            );
            return Ok(String::from_utf8(data).unwrap());
        }
        Err(_) => {
            return Err("文件打开失败".to_string()); // for test
        }
    }
}
pub fn recover_file(filename: &str) -> Result<String, String> {
    let mut path = PathBuf::from("ciphertext");
    path.push(filename);
    let file = std::fs::File::open(path);
    match file {
        Ok(file) => {
            let encrypted_file: EncryptedFile;
            match serde_json::from_reader(file) {
                Ok(c) => encrypted_file = c,
                Err(_) => {
                    return Err("不是加密文件".to_string());
                }
            }
            let priv_key;
            match RsaPrivateKey::from_pkcs1_pem(pem::PRIV_KEY_ROOT) {
                Ok(c) => priv_key = c,
                Err(_) => {
                    return Err("无权限访问".to_string());
                }
            }
            let fek;
            match priv_key.decrypt(Pkcs1v15Encrypt, &encrypted_file.drf) {
                Ok(c) => fek = c,
                Err(_) => {
                    return Err("无权限访问".to_string());
                }
            }
            let data = cbc::aes_cbc_decrypt(
                &encrypted_file.encrypted_data,
                aes::KeySize::KeySize256,
                &fek,
                IV,
            );
            return Ok(String::from_utf8(data).unwrap());
        }
        Err(e) => {
            return Err(e.to_string()); // for test
        }
    }
}
