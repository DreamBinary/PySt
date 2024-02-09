use std::time;
use rustpython_vm as vm;
use pyo3::indoc::formatdoc;
use pyo3::prelude::*;


#[pyfunction]
pub fn para_sum(n: i64) -> i64 {
    let thread_num = 10;
    let mut sum = 0;
    let mut handles = vec![];
    for i in 0..thread_num {
        let start = i * n / thread_num;
        let end = (i + 1) * n / thread_num;
        let handle = std::thread::spawn(move || one_sum(start, end));
        handles.push(handle);
    }
    for handle in handles {
        sum += handle.join().unwrap();
    }
    return sum;
}


#[pyfunction]
pub fn no_para_sum(n: i64) -> i64 {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }
    return sum;
}

// #[pyfunction]
fn one_sum(a: i64, b: i64) -> i64 {
    let mut sum = 0;
    for i in a..b {
        sum += i;
    }
    return sum;
}
// #[pyfunction]
pub fn sum_rust_python(n: i64) -> vm::PyResult<()> {
    vm::Interpreter::without_stdlib(Default::default()).enter(|vm| {
        let source = formatdoc! {
       r#"
from multiprocessing.pool import ThreadPool

def para_sum_py(n):
    thread_num = 10
    sum = 0
    handles = []
    with ThreadPool(thread_num) as pool:
        for i in range(thread_num):
            start = i * n // thread_num
            end = (i + 1) * n // thread_num
            handles.append(pool.apply_async(one_sum, (start, end)))
        pool.close()
        pool.join()
    for handle in handles:
        sum += handle.get()
    return sum


def one_sum(a, b):
    s = 0
    for i in range(a, b):
        s += i
    return s


def no_para_sum_py(n):
    s = 0
    for i in range(n):
        s += i
    return s


def test(num=100):
    import time
    start = time.perf_counter()
    para_sum_py(num)
    end = time.perf_counter()
    t_py_para = end - start

    start = time.perf_counter()
    no_para_sum_py(num)
    end = time.perf_counter()
    t_py_no_para = end - start

    print("num: ", num)
    print("py para: ", t_py_para)
    print("py no para: ", t_py_no_para)

test({num})
       "#,
        num = n
    };
        let source = source.as_str();
        // println!("{}", source);
        let source_path = "<embedded>".to_owned();
        use rustpython_vm as vm;
        let scope = vm.new_scope_with_builtins();

        let code_obj = vm
            .compile(source, vm::compiler::Mode::Exec, source_path)
            .map_err(|err| vm.new_syntax_error(&err, Some(source)))?;

        vm.run_code_obj(code_obj, scope)?;

        Ok(())
    })
}

pub fn test_para_sum() {
    for i in [100, 10000, 1000000, 100000000] {
        let start = time::Instant::now();
        no_para_sum(i);
        let duration = start.elapsed();
        println!("{}: {:?}", i, duration);
    }

    for i in [100, 10000, 1000000, 100000000] {
        let start = time::Instant::now();
        para_sum(i);
        let duration = start.elapsed();
        println!("{}: {:?}", i, duration);
    }

    for i in [100, 10000, 1000000, 100000000] {
        // start = time.perf_counter()
        let start = time::Instant::now();
        sum_rust_python(i);
        let duration = start.elapsed();
        println!("{}: {:?}", i, duration);
    }
}
/*
100: 902ns
10000: 81.426µs
1000000: 6.78381ms
100000000: 618.34921ms
100: 691.258µs
10000: 414.591µs
1000000: 1.535779ms
100000000: 94.933083ms
100: 69.64043ms
10000: 55.899628ms
1000000: 61.170857ms
100000000: 61.489806ms
*/