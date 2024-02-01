use bitburner_api::NS;

pub fn get_threads_for_full_weaken(ns: &NS, host: &str, cores: u32) -> u32 {
    let min_security = ns.get_server_min_security_level(host);
    let cur_security = ns.get_server_security_level(host);
    if cur_security <= min_security {
        return 0;
    }
    let diff = cur_security - min_security;
    weaken_analyze_threads(ns, diff, Some(cores))
}

pub fn weaken_analyze_threads(ns: &NS, security_decrease: f64, cores: Option<u32>) -> u32 {
    let mut max_weaken_threads = 1;
    while ns.weaken_analyze(max_weaken_threads, cores) < security_decrease {
        max_weaken_threads *= 2;
    }
    split_thread_solve(
        max_weaken_threads,
        |t| ns.weaken_analyze(t, cores),
        |_| security_decrease,
    )
}

fn split_thread_solve<F1, F2>(threads: u32, high_prio_fn: F1, low_prio_fn: F2) -> u32
where
    F1: Fn(u32) -> f64,
    F2: Fn(u32) -> f64,
{
    let left_bound = 0;
    let right_bound = threads;
    let splitter = |l, r| (l + r) / 2;
    let pred = |&high_prio_threads: &u32| -> bool {
        let high_prio_out = high_prio_fn(high_prio_threads);
        let low_prio_out = low_prio_fn(threads - high_prio_threads);
        high_prio_out >= low_prio_out
    };
    binary_search(pred, splitter, left_bound, right_bound)
}

/// Find first argument of a monotonic predicate for which it is true
fn binary_search<P, I, S>(pred: P, splitter: S, left_bound: I, right_bound: I) -> I
where
    P: Fn(&I) -> bool,
    S: Fn(I, I) -> I,
    I: std::cmp::PartialEq + std::marker::Copy,
{
    let mut left_bound = left_bound;
    let mut right_bound = right_bound;
    loop {
        let split = splitter(left_bound, right_bound);
        let done = split == left_bound;
        if done {
            return right_bound;
        }
        if !pred(&split) {
            left_bound = split;
        } else {
            right_bound = split;
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::thread_search::binary_search;

    #[quickcheck]
    fn test_binary_search(xs: Vec<i32>) {
        let xs = xs.clone();
        if xs.is_empty() {
            return;
        }

        let idx = binary_search(
            |&idx| xs[idx] >= 0,
            |l, r| (l + r) / 2,
            0usize,
            xs.len() - 1,
        );
        if xs[xs.len() - 1] >= 0 {
            assert!(xs[idx] >= 0)
        }
        if xs[0] < 0 && xs.len() > 1 {
            assert!(xs[idx - 1] < 0)
        }
    }
}
