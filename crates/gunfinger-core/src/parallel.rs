//! One worker per item, on scoped threads.

use std::panic;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// Applies `work` to every item on `jobs` threads, each thread taking the
/// next unprocessed item. Results come back in item order. A panic in `work`
/// is re-raised on the calling thread.
pub fn map_in_order<T: Sync, R: Send>(
    items: &[T],
    jobs: usize,
    work: impl Fn(&T) -> R + Sync,
) -> Vec<R> {
    map_in_order_with(items, jobs, || (), |(), item| work(item))
}

/// Like `map_in_order`, giving each thread a scratch value made once by
/// `make_scratch` and passed to `work` for every item the thread takes: a
/// buffer reused from item to item instead of allocated for each.
pub fn map_in_order_with<T: Sync, S, R: Send>(
    items: &[T],
    jobs: usize,
    make_scratch: impl Fn() -> S + Sync,
    work: impl Fn(&mut S, &T) -> R + Sync,
) -> Vec<R> {
    let next = AtomicUsize::new(0);
    let mut results: Vec<(usize, R)> = thread::scope(|scope| {
        let workers: Vec<_> = (0..jobs.clamp(1, items.len().max(1)))
            .map(|_| {
                scope.spawn(|| {
                    let mut scratch = make_scratch();
                    let mut done = Vec::new();
                    loop {
                        let position = next.fetch_add(1, Ordering::Relaxed);
                        let Some(item) = items.get(position) else {
                            break;
                        };
                        done.push((position, work(&mut scratch, item)));
                    }
                    done
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| {
                worker
                    .join()
                    .unwrap_or_else(|payload| panic::resume_unwind(payload))
            })
            .collect()
    });
    results.sort_by_key(|(position, _)| *position);
    results.into_iter().map(|(_, result)| result).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn results_keep_the_order_of_the_items() {
        let items: Vec<u64> = (0..100).collect();

        let squares = map_in_order(&items, 7, |item| item * item);

        assert_eq!(
            squares,
            items.iter().map(|item| item * item).collect::<Vec<_>>()
        );
    }

    #[test]
    fn each_thread_keeps_its_scratch_from_item_to_item() {
        let items: Vec<u64> = (0..100).collect();

        let seen = map_in_order_with(&items, 3, Vec::new, |seen: &mut Vec<u64>, item| {
            seen.push(*item);
            seen.len()
        });

        // Three threads share the items, so 100 items reach their scratch
        // values with 100 pushes in all.
        assert_eq!(seen.len(), 100);
        assert!(seen.iter().filter(|&&count| count == 1).count() <= 3);
    }

    #[test]
    fn no_items_need_no_work() {
        let items: [u8; 0] = [];

        assert!(map_in_order(&items, 4, |item| *item).is_empty());
    }
}
