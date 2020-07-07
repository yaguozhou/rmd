/*
 * Copyright (c) 2020 Yaguo Zhou
 * rmd is licensed under Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *          http://license.coscl.org.cn/MulanPSL2
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
 * EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
 * MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 * See the Mulan PSL v2 for more details.
 */
use std::path::PathBuf;

use tiny_keccak::Hasher;
use tiny_keccak::Sha3;

pub struct Metadata {
    path: PathBuf,
    hash: [u8; 32],
}

impl Metadata {
    pub fn new(path: PathBuf, data: &[u8]) -> Self {
        Metadata {
            path,
            hash: get_hash(data),
        }
    }

    pub fn is_same_as(&self, data: &[u8]) -> bool {
        self.hash == get_hash(data)
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.hash = get_hash(data);
    }
}

fn get_hash(data: &[u8]) -> [u8; 32] {
    let mut sha3 = Sha3::v256();
    let mut hash = [0u8; 32];
    sha3.update(data);
    sha3.finalize(&mut hash);
    hash
}
