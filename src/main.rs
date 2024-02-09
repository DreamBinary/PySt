// -*- coding:utf-8 -*-
// @FileName : main.rs
// @Time : 2024/2/9 11:39
// @Author : fiv
mod hello_pyo3;

use hello_pyo3::hello_pyo3;

fn main() {
    hello_pyo3().unwrap();
}
