// -*- coding:utf-8 -*-
// @FileName : hello_pyo3.rs
// @Time : 2024/2/9 12:33
// @Author : fiv

use pyo3::indoc::indoc;
use pyo3::prelude::*;
use pyo3::prepare_freethreaded_python;

pub fn hello_pyo3() -> PyResult<()> {
    prepare_freethreaded_python();
    // let py = unsafe { Python::assume_gil_acquired() };
    Python::with_gil(|py| {
        let source: &str = indoc! {
            r#"
def hello():
    print("Hello, PyO3!")


hello()
# get version of python
import sys

print("Python version: ", sys.version)
print("Python version info: ", sys.version_info)
print("Python interpreter location: ", sys.executable)
            "#
        };
        py.run(source, None, None).unwrap();
    });

    Ok(())
}

// Hello, PyO3!
// Python version:  3.10.12 (main, Nov 20 2023, 15:14:05) [GCC 11.4.0]
// Python version info:  sys.version_info(major=3, minor=10, micro=12, releaselevel='final', serial=0)
// Python interpreter location:  /usr/bin/python3
