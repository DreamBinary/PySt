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
