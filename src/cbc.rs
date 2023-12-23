/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-23 19:57:47
 * @LastEditors: goodpeanuts goddpeanuts@foxmail.com
 * @LastEditTime: 2023-12-24 03:26:03
 * @FilePath: /file_encrypt/src/cbc.rs
 * @Description:
 *
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved.
 */
use crypto::buffer::{ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer};
use std::fs::{File, OpenOptions};
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{Read, Write};
use std::time::Instant;
use rand::Rng;
// use crypto::aessafe::*;
// use crypto::blockmodes::*;
// use crypto::symmetriccipher::*;
// use crypto::aes::*;
// use std::vec;

// 指定固定的密钥和IV，实际应用中需要使用安全的随机生成的密钥和IV
const KEY256: &[u8] = b"0123456789abcdef0123456789abcdef"; // 用于256的测试密钥
                                                           // const KEY128: &[u8] = b"0123456789abcdef";  // 用于128的测试密钥
const IV: &[u8] = b"0123456789abcdef";

// 用于测试缓冲读入加密解密速度
pub fn speedtest() {
    let mut in_file = File::open("file.txt").expect("file not found");

    let mut encrypted_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("file_encrypted.txt")
        .expect("create file failed");
    let mut decryped_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("file_decryped.txt")
        .expect("create file failed");

    aes256_cbc_encrypt(&mut in_file, &mut encrypted_file, &KEY256, &IV);

    encrypted_file.seek(SeekFrom::Start(0)).unwrap();
    aes256_cbc_decrypt(&mut encrypted_file, &mut decryped_file, &KEY256, &IV);

    // 不缓冲读入的加密解密
    // let encrypted_data_base64 = encode(&encrypted_data);
    // println!("Encrypted data (Base64):");
    // println!("{}", encrypted_data_base64);
    // encrypted_file
    //     .write_all(&encrypted_data_base64.as_bytes())
    //     .expect("write failed");
    // std::println!("encrypted_data: {:?}", encrypt_string);

    // 解密
    // let decrypted_data = aes256_cbc_decrypt(&encrypted_data, &KEY, &IV).ok().unwrap();
    // let decrypted_string = String::from_utf8(decrypted_data).expect("Found invalid UTF-8");
    // std::println!("decrypted_data");
    // std::println!("{}", decrypted_string);
    // decryped_file
    //     .write_all(&decrypted_string.as_bytes())
    //     .expect("write failed");
}

// for test
// Encrypt a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn aes256_cbc_encrypt(in_file: &mut File, out_file: &mut File, key: &[u8], iv: &[u8]) {
    let start = Instant::now();
    let mut time = start;
    let mut encryptor =
        aes::cbc_encryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);
    let buffer_size = 4096 * 1024;
    let mut read_buffer = vec![0; buffer_size]; // 将缓冲区分配到堆上
    let mut write_buffer = vec![0; buffer_size];
    let mut buffer = buffer::RefWriteBuffer::new(&mut write_buffer);
    let mut count = 0;

    loop {
        let n = in_file.read(&mut read_buffer).unwrap();
        count = count + 1;

        if n == 0 {
            break;
        }
        println!("encryptor read {} kB ", n / 1024);

        let mut read_buffer = buffer::RefReadBuffer::new(&read_buffer[..n]);
        encryptor
            .encrypt(&mut read_buffer, &mut buffer, n < buffer_size)
            .unwrap();

        out_file
            .write_all(buffer.take_read_buffer().take_remaining())
            .expect("write failed");
        buffer.reset();
        println!("@loop {} cost time: {:?}", count, Instant::now() - time);
        time = Instant::now();
    }
    println!();
    println!(
        "====  encryptor cost time: {:?}  ====",
        Instant::now() - start
    );
    println!();
}

// for test
// Decrypts a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn aes256_cbc_decrypt(in_file: &mut File, out_file: &mut File, key: &[u8], iv: &[u8]) {
    // 获取当前时间
    let start = Instant::now();
    let mut time = start;
    let mut decryptor =
        aes::cbc_decryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    let buffer_size = 4096 * 1024;
    let mut read_buffer = vec![0; buffer_size]; // 将缓冲区分配到堆上
    let mut write_buffer = vec![0; buffer_size];
    let mut buffer = buffer::RefWriteBuffer::new(&mut write_buffer);
    let mut count = 0;

    loop {
        let n = in_file.read(&mut read_buffer).unwrap();
        count = count + 1;

        if n == 0 {
            break;
        }
        println!("decryptor read {} kb ", n / 1024);

        let mut read_buffer = buffer::RefReadBuffer::new(&read_buffer[..n]);
        decryptor
            .decrypt(&mut read_buffer, &mut buffer, n < buffer_size)
            .unwrap();

        // 直接将解密后的数据写入到输出文件
        out_file
            .write_all(buffer.take_read_buffer().take_remaining())
            .expect("write failed");
        buffer.reset();
        println!("@loop {} cost time: {:?}", count, Instant::now() - time);
        time = Instant::now();
    }
    println!();
    println!(
        "====  decryptor cost time: {:?}  ====",
        Instant::now() - start
    );
    println!();
}

pub fn aes_cbc_encrypt(input: &[u8], keysize: aes::KeySize, key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut encryptor = aes::cbc_encryptor(keysize, key, iv, blockmodes::PkcsPadding);
    let input_size = input.len();
    let mut input_buffer = buffer::RefReadBuffer::new(&input);
    let mut output = vec![0; input_size + 16]; // 这里需要使用Vec<u8>，而不是String，String不可变
    let mut output_buffer = buffer::RefWriteBuffer::new(&mut output);

    encryptor
        .encrypt(&mut input_buffer, &mut output_buffer, true)
        .unwrap();

    output_buffer.take_read_buffer().take_remaining().to_vec()
}

// Decrypts a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
pub fn aes_cbc_decrypt(input: & [u8], keysize: aes::KeySize, key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut decryptor = aes::cbc_decryptor(keysize, key, iv, blockmodes::PkcsPadding);
    let input_size = input.len();
    let mut input_buffer = buffer::RefReadBuffer::new(&input);
    let mut output = vec![0; input_size + 16]; // 这里需要使用Vec<u8>，而不是String，String不可变
    let mut output_buffer = buffer::RefWriteBuffer::new(&mut output);

    decryptor
        .decrypt(&mut input_buffer, &mut output_buffer, true)
        .unwrap();

    output_buffer.take_read_buffer().take_remaining().to_vec()
}

pub fn generate_aes256_key() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut key = vec![0; 32];
    for i in 0..key.len() {
        key[i] = rng.gen::<u8>();
    }
    key
}
