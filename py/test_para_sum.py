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

# num = 100
# rust para: 0.0008132419898174703
# rust no para: 1.073098974302411e-05
# py para: 0.012421645995345898
# py no para: 6.541988113895059e-06
# num = 10000
# rust para: 0.0005313739966368303
# rust no para: 6.447300256695598e-05
# py para: 0.006745228005456738
# py no para: 0.0003836519899778068
# num = 1000000
# rust para: 0.0014951210032450035
# rust no para: 0.005787941001472063
# py para: 0.04720868400181644
# py no para: 0.042303146998165175
# num = 100000000
# rust para: 0.09585643401078414
# rust no para: 0.6242619310069131
# py para: 4.069613264990039
# py no para: 4.310906865008292