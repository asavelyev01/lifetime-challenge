//! HARNESS — do not edit.
use lifetimes_2026_10::p08_async_store::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn store() -> Store {
    Store::new([("a", "1"), ("b", "2"), ("c", "3"), ("é", "✓")])
}

#[tokio::test]
async fn lookup_finds_values() {
    let s = store();
    assert_eq!(lookup(&s, "a").await, Some("1"));
    assert_eq!(lookup(&s, "é").await, Some("✓"));
    assert_eq!(lookup(&s, "zz").await, None);
    assert_eq!(s.calls(), 3);
}

#[tokio::test]
async fn lookup_all_preserves_order_and_handles_missing() {
    let s = store();
    let keys = ["c", "zz", "a", "a", "é"];
    assert_eq!(lookup_all(&s, &keys).await, [Some("3"), None, Some("1"), Some("1"), Some("✓")]);
    assert_eq!(s.calls(), 5, "exactly one get per key");
}

#[tokio::test]
async fn lookup_all_empty() {
    let s = store();
    assert_eq!(lookup_all(&s, &[]).await, Vec::<Option<&str>>::new());
}

#[tokio::test]
async fn lookup_all_runs_concurrently() {
    let s = store();
    let keys: Vec<&str> = ["a", "b", "c", "x"].iter().copied().cycle().take(12).collect();
    let got = lookup_all(&s, &keys).await;
    assert_eq!(got.len(), 12);
    assert_eq!(s.max_in_flight(), 12, "all lookups must be in flight at the same time");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn spawn_lookups_returns_owned_values_in_order() {
    let s = Arc::new(store());
    let handles = {
        let keys = [String::from("b"), String::from("nope"), String::from("é")];
        let refs: Vec<&str> = keys.iter().map(String::as_str).collect();
        spawn_lookups(Arc::clone(&s), &refs)
    };
    let mut out = vec![];
    for h in handles {
        out.push(h.await.unwrap());
    }
    assert_eq!(out, [Some("2".to_string()), None, Some("✓".to_string())]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn spawn_lookups_spawns_eagerly() {
    let s = Arc::new(store());
    let handles = spawn_lookups(Arc::clone(&s), &["a", "b", "c"]);
    drop(handles); // detaches the tasks; they must still run
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while s.calls() < 3 {
        assert!(tokio::time::Instant::now() < deadline, "tasks did not run: {} calls", s.calls());
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
}

#[tokio::test]
async fn for_each_found_calls_in_order_for_found_keys_only() {
    let s = store();
    let log = Arc::new(Mutex::new(Vec::new()));
    let l = Arc::clone(&log);
    let n = for_each_found(&s, &["c", "x", "a", "c"], move |key: &str, value: &str| {
        let entry = format!("{key}={value}");
        let l = Arc::clone(&l);
        async move {
            tokio::task::yield_now().await;
            l.lock().unwrap().push(entry);
        }
    })
    .await;
    assert_eq!(n, 3);
    assert_eq!(*log.lock().unwrap(), ["c=3", "a=1", "c=3"]);
}

#[tokio::test]
async fn for_each_found_awaits_each_callback_before_the_next_lookup() {
    let s = Arc::new(store());
    let log = Arc::new(Mutex::new(Vec::new()));
    let (l, s2) = (Arc::clone(&log), Arc::clone(&s));
    let n = for_each_found(&s, &["a", "b"], move |key: &str, _: &str| {
        let (l, s2, key) = (Arc::clone(&l), Arc::clone(&s2), key.to_string());
        async move {
            tokio::task::yield_now().await;
            l.lock().unwrap().push(format!("{key}@{}", s2.calls()));
        }
    })
    .await;
    assert_eq!(n, 2);
    assert_eq!(*log.lock().unwrap(), ["a@1", "b@2"], "callback for a must finish before b is looked up");
}

#[tokio::test]
async fn for_each_found_with_nothing_found() {
    let s = store();
    let n = for_each_found(&s, &["x", "y"], |_: &str, _: &str| async {
        if true {
            panic!("must not be called")
        }
    }).await;
    assert_eq!(n, 0);
    assert_eq!(s.calls(), 2);
}
