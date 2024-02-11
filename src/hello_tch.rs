// -*- coding:utf-8 -*-
// @FileName : hello_tch.rs
// @Time : 2024/2/11 13:45
// @Author : fiv


use tch::Cuda;
use tch::Tensor;

pub fn hello_tch() {
    println!("Hello, tch!");
    println!("Cuda device count: {}", Cuda::device_count());
    println!("Cuda is available: {}", Cuda::is_available());

    let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    t.print();
}