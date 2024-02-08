mod test_para_sum;

use test_para_sum::para_sum;
use std::time;

fn main() {
    for i in [100, 10000, 1000000] {
        // start = time.perf_counter()
        let start = time::Instant::now();
        para_sum(i);
        let duration = start.elapsed();
        println!("{}: {:?}", i, duration);
    }
}