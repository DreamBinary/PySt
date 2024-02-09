// -*- coding:utf-8 -*-
// @FileName : main.rs
// @Time : 2024/2/9 11:39
// @Author : fiv
mod hello_pyo3;

use hello_pyo3::hello_pyo3;

mod test_para_sum;
mod hello_rust_python;
use hello_rust_python::hello_rust_python;
use test_para_sum::test_para_sum;

fn main() {
    test_para_sum();
}
