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
use gtk::*;
use sourceview::*;

pub fn get_content(buff: &Buffer) -> Option<String> {
    let start = buff.get_start_iter();
    let end = buff.get_end_iter();
    if let Some(gstr) = buff.get_text(&start, &end, true) {
        Some(gstr.to_owned())
    } else {
        None
    }
}
