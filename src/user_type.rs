/*
 * @Author: goodpeanuts goddpeanuts@foxmail.com
 * @Date: 2023-12-23 15:24:35
 * @LastEditors: goodpeanuts goddpeanuts@foxmail.com
 * @LastEditTime: 2023-12-24 00:58:16
 * @FilePath: /file_encrypt/src/user_type.rs
 * @Description: 
 * 
 * Copyright (c) 2023 by goodpeanuts, All Rights Reserved. 
 */
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum FileAccessLevel {
    Fa,
    Fb,
    Fc,
    Fd,
    Fe,
}

struct UserAccessA {
    file_access: [FileAccessLevel; 5], // 使用数组，长度为 3
}

struct UserAccessB {
    file_access: [FileAccessLevel; 2], 
}

struct UserAccessC {
    file_access: [FileAccessLevel; 2], 
}

struct UserAccessD {
    file_access: [FileAccessLevel; 2], 
}

struct UserAccessE {
    file_access: [FileAccessLevel; 1], 
}

// 定义 UserAccess 的常量
const USER_ACCESS_A_CONSTANT: UserAccessA = UserAccessA {
    file_access: [FileAccessLevel::Fa, FileAccessLevel::Fb, FileAccessLevel::Fc, FileAccessLevel::Fd, FileAccessLevel::Fe],
};

const USER_ACCESS_B_CONSTANT: UserAccessB = UserAccessB {
    file_access: [FileAccessLevel::Fa, FileAccessLevel::Fb],
};

const USER_ACCESS_C_CONSTANT: UserAccessC = UserAccessC {
    file_access: [FileAccessLevel::Fc, FileAccessLevel::Fd],
};

const USER_ACCESS_D_CONSTANT: UserAccessD = UserAccessD {
    file_access: [FileAccessLevel::Fb, FileAccessLevel::Fe],
};

const USER_ACCESS_E_CONSTANT: UserAccessE = UserAccessE {
    file_access: [FileAccessLevel::Fd],
};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub salt: String,
    pub level: String,
}