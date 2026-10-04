//! HARNESS — do not edit. (`from_fn` is covered by `src/checks/p10.rs`.)
use lifetimes_2026_10::p10_pipeline::*;
use std::borrow::Cow;

fn req(path: &str) -> Request<'_> {
    Request::new(path)
}

#[test]
fn header_lookup() {
    let r = req("/")
        .with_header("Content-Type", "a")
        .with_header("content-type", "b")
        .with_header("X", "");
    assert_eq!(
        r.header("content-type"),
        Some("a"),
        "case-insensitive, first wins"
    );
    assert_eq!(r.header("CONTENT-TYPE"), Some("a"));
    assert_eq!(r.header("x"), Some(""));
    assert_eq!(r.header("content"), None, "whole name only");
    assert_eq!(req("/").header("x"), None);
}

#[tokio::test]
async fn require_header() {
    let m = RequireHeader {
        name: "Auth".into(),
    };
    let p = Pipeline::new().with(&m);
    assert!(p.run(req("/").with_header("auth", "t")).await.is_ok());
    assert_eq!(
        p.run(req("/")).await,
        Err(Reject::MissingHeader("Auth".into()))
    );
}

#[tokio::test]
async fn strip_prefix_keeps_borrowed_paths_borrowed() {
    let m = StripPrefix {
        prefix: "/api".into(),
    };
    let p = Pipeline::new().with(&m);
    let raw = String::from("/api/users");
    let out = p.run(req(&raw)).await.unwrap();
    assert_eq!(out.path, "/users");
    match out.path {
        Cow::Borrowed(s) => assert_eq!(
            s.as_ptr(),
            raw[4..].as_ptr(),
            "must be a slice of the input"
        ),
        Cow::Owned(_) => panic!("a borrowed path must stay borrowed"),
    }
}

#[tokio::test]
async fn strip_prefix_edge_cases() {
    let m = StripPrefix {
        prefix: "/api".into(),
    };
    let p = Pipeline::new().with(&m);
    assert_eq!(
        p.run(req("/api")).await.unwrap().path,
        "/",
        "empty result becomes /"
    );
    assert_eq!(p.run(req("/apix")).await.unwrap().path, "x");
    assert_eq!(p.run(req("/v1/api")).await, Err(Reject::NotFound));
    assert_eq!(p.run(req("")).await, Err(Reject::NotFound));
    let mut owned = req("ignored");
    owned.path = Cow::Owned("/api/z".to_string());
    assert_eq!(p.run(owned).await.unwrap().path, "/z");
    let empty = StripPrefix {
        prefix: String::new(),
    };
    let p = Pipeline::new().with(&empty);
    assert_eq!(p.run(req("/q")).await.unwrap().path, "/q");
}

#[tokio::test]
async fn stages_run_in_order_and_stop_at_first_rejection() {
    let need_a = RequireHeader { name: "a".into() };
    let need_b = RequireHeader { name: "b".into() };
    let strip = StripPrefix {
        prefix: "/p".into(),
    };
    let p = Pipeline::new().with(&strip).with(&need_a).with(&need_b);
    assert_eq!(p.run(req("/x")).await, Err(Reject::NotFound));
    assert_eq!(
        p.run(req("/p/x")).await,
        Err(Reject::MissingHeader("a".into()))
    );
    assert_eq!(
        p.run(req("/p/x").with_header("a", "1")).await,
        Err(Reject::MissingHeader("b".into()))
    );
    let ok = p
        .run(req("/p/x").with_header("b", "2").with_header("a", "1"))
        .await
        .unwrap();
    assert_eq!(ok, req("/x").with_header("b", "2").with_header("a", "1"));
    // A stage that rewrites the path affects later stages:
    let strip2 = StripPrefix {
        prefix: "/x".into(),
    };
    let p = Pipeline::new().with(&strip).with(&strip2);
    assert_eq!(p.run(req("/p/x/y")).await.unwrap().path, "/y");
    assert_eq!(
        Pipeline::new().run(req("/same")).await.unwrap(),
        req("/same"),
        "no stages"
    );
}

#[tokio::test]
async fn rate_limit_sequential() {
    let rl = RateLimit::new(2);
    let p = Pipeline::new().with(&rl);
    for _ in 0..2 {
        assert!(p.run(req("/").with_header("user", "ann")).await.is_ok());
    }
    assert_eq!(
        p.run(req("/").with_header("User", "ann")).await,
        Err(Reject::RateLimited { user: "ann".into() })
    );
    assert!(
        p.run(req("/").with_header("user", "bob")).await.is_ok(),
        "per user"
    );
    assert!(p.run(req("/")).await.is_ok());
    assert!(p.run(req("/")).await.is_ok());
    assert_eq!(
        p.run(req("/")).await,
        Err(Reject::RateLimited {
            user: "anonymous".into()
        })
    );
    assert_eq!(
        rl.counters().peek("ann"),
        2,
        "counts live in Counters, rejected requests don't count"
    );
    assert_eq!(rl.counters().peek("bob"), 1);
    assert_eq!(rl.counters().peek("anonymous"), 2);
}

#[tokio::test]
async fn rate_limit_zero_rejects_everything() {
    let rl = RateLimit::new(0);
    let p = Pipeline::new().with(&rl);
    assert_eq!(
        p.run(req("/")).await,
        Err(Reject::RateLimited {
            user: "anonymous".into()
        })
    );
}

#[tokio::test]
async fn rate_limit_is_correct_under_concurrency() {
    let rl = RateLimit::new(3);
    let p = Pipeline::new().with(&rl);
    let reqs: Vec<Request> = (0..10)
        .map(|i| req("/").with_header("user", if i % 5 == 4 { "bob" } else { "ann" }))
        .collect();
    let results = p.run_all(reqs).await;
    let ok_ann = results
        .iter()
        .filter(|r| r.as_ref().is_ok_and(|r| r.header("user") == Some("ann")))
        .count();
    let ok_bob = results
        .iter()
        .filter(|r| r.as_ref().is_ok_and(|r| r.header("user") == Some("bob")))
        .count();
    assert_eq!(
        (ok_ann, ok_bob),
        (3, 2),
        "8 concurrent requests for ann with a limit of 3"
    );
    assert_eq!(rl.counters().peek("ann"), 3);
    assert_eq!(rl.counters().peek("bob"), 2);
}

#[tokio::test]
async fn run_all_preserves_order() {
    let m = StripPrefix {
        prefix: "/a".into(),
    };
    let p = Pipeline::new().with(&m);
    let raw: Vec<String> = (0..20)
        .map(|i| {
            if i % 3 == 0 {
                format!("/b{i}")
            } else {
                format!("/a{i}")
            }
        })
        .collect();
    let results = p.run_all(raw.iter().map(|s| req(s)).collect()).await;
    for (i, r) in results.iter().enumerate() {
        if i % 3 == 0 {
            assert_eq!(r, &Err(Reject::NotFound), "request {i}");
        } else {
            assert_eq!(r.as_ref().unwrap().path, i.to_string(), "request {i}");
        }
    }
    assert!(p.run_all(vec![]).await.is_empty());
}

struct Probe {
    in_flight: std::sync::atomic::AtomicUsize,
    max: std::sync::atomic::AtomicUsize,
}

impl Middleware for Probe {
    fn handle<'a, 'r>(&'a self, _req: &'a mut Request<'r>) -> BoxFuture<'a, Result<(), Reject>> {
        use std::sync::atomic::Ordering::SeqCst;
        Box::pin(async move {
            let now = self.in_flight.fetch_add(1, SeqCst) + 1;
            self.max.fetch_max(now, SeqCst);
            tokio::task::yield_now().await;
            self.in_flight.fetch_sub(1, SeqCst);
            Ok(())
        })
    }
}

#[tokio::test]
async fn run_all_is_concurrent() {
    let probe = Probe {
        in_flight: 0.into(),
        max: 0.into(),
    };
    let p = Pipeline::new().with(&probe);
    let out = p.run_all((0..8).map(|_| req("/")).collect()).await;
    assert!(out.iter().all(Result::is_ok));
    assert_eq!(
        probe.max.load(std::sync::atomic::Ordering::SeqCst),
        8,
        "all requests in flight together"
    );
}
