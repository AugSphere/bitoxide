use bitburner_api::NS;

pub fn get_threads_for_full_weaken(ns: &NS, host: &str, cores: u32) -> u32 {
    let min_security = ns.get_server_min_security_level(host);
    let cur_security = ns.get_server_security_level(host);
    if cur_security <= min_security {
        return 0;
    }
    let diff = cur_security - min_security;
    let mut right_bound = 1;
    while ns.weaken_analyze(right_bound, Some(cores)) < diff {
        right_bound *= 2;
    }
    let left_bound = right_bound / 2;
    binary_search(
        |&threads| ns.weaken_analyze(threads, Some(cores)) >= diff,
        |l, r| (l + r) / 2,
        left_bound,
        right_bound,
    )
}

pub fn weaken_threads_to_grow(ns: &NS, threads: u32, cores: u32) -> u32 {
    let left_bound = 0;
    let right_bound = threads;
    let splitter = |l, r| (l + r) / 2;
    let pred = |&weaken_threads: &u32| -> bool {
        let sec_inc = ns.growth_analyze_security(threads - weaken_threads, None, Some(cores));
        let sec_dec = ns.weaken_analyze(weaken_threads, Some(cores));
        sec_dec >= sec_inc
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
