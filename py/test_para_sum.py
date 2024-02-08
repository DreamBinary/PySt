# -*- coding:utf-8 -*-
# @FileName : test_para_sum.py
# @Time : 2024/2/7 17:10
# @Author : fiv

from pyst import para_sum, no_para_sum
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
    para_sum(num)
    end = time.perf_counter()
    t_rust_para = end - start

    start = time.perf_counter()
    no_para_sum(num)
    end = time.perf_counter()
    t_rust_no_para = end - start

    start = time.perf_counter()
    para_sum_py(num)
    end = time.perf_counter()
    t_py_para = end - start

    start = time.perf_counter()
    no_para_sum_py(num)
    end = time.perf_counter()
    t_py_no_para = end - start

    print(f"num = {num}")
    print(f"rust para: {t_rust_para}")
    print(f"rust no para: {t_rust_no_para}")
    print(f"py para: {t_py_para}")
    print(f"py no para: {t_py_no_para}")


if __name__ == '__main__':
    for i in [100, 10000, 1000000, 100000000]:
        test(i)
"""
mod test_para_sum;

use test_para_sum::para_sum;
use std::time;

fn main() {
    for i in [100, 10000, 1000000, 100000000] {
        // start = time.perf_counter()
        let start = time::Instant::now();
        para_sum(i);
        let duration = start.elapsed();
        println!("{}: {:?}", i, duration);
    }
}
100: 1.963492ms
10000: 533.593µs
1000000: 1.608709ms

"""
# num = 100
# rust para: 0.0010953399978461675
# rust no para: 7.9849996836856e-06
# py para: 0.008112716997857206
# py no para: 6.080997991375625e-06
# num = 10000
# rust para: 0.0005697739979950711
# rust no para: 6.185800157254562e-05
# py para: 0.005052483000326902
# py no para: 0.00035104000562569126
# num = 1000000
# rust para: 0.0013833439952577464
# rust no para: 0.005882632001885213
# py para: 0.04755044199555414
# py no para: 0.041981973001384176
