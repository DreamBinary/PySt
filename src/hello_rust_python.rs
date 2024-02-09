// -*- coding:utf-8 -*-
// @FileName : hello_rust_python
// @Time : 2024/2/9 11:03
// @Author : fiv


use pyo3::indoc::{formatdoc, indoc, printdoc};
use rustpython_vm as vm;


pub fn hello_rust_python() -> vm::PyResult<()> {
    vm::Interpreter::without_stdlib(Default::default()).enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        let source: &str = indoc! {
            r#"
def hello():
    print("Hello, RustPython!")


hello()
# get version of python
import sys

print("Python version: ", sys.version)
print("Python version info: ", sys.version_info)
print("Python interpreter location: ", sys.executable)
            "#
        };
        printdoc!("{}", source);
        println!();
        let source_path = "<embedded>".to_owned();
        // "<embedded>" is used to indicate that the source code is not from a file.
        let code_obj = vm
            .compile(source, vm::compiler::Mode::Exec, source_path)
            .map_err(|err| vm.new_syntax_error(&err, Some(source)))?;

        vm.run_code_obj(code_obj, scope)?;

        Ok(())
    })
}