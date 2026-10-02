//! P10 compile checks. HARNESS — do not edit.

/// ```
/// use lifetimes_2026_10::p10_pipeline::*;
/// let raw = String::from("user: ann");
/// let value = {
///     let req = Request::new("/").with_header(&raw[..4], &raw[6..]);
///     req.header("USER") // the request dies here, the value borrows `raw`
/// };
/// assert_eq!(value, Some("ann"));
/// ```
pub fn header_value_outlives_the_request() {}

/// ```
/// use lifetimes_2026_10::{checks::block_on, p10_pipeline::*};
/// let tag = from_fn(|req| Box::pin(async move {
///     let user = req.header("user").unwrap_or("anonymous"); // keep the value…
///     tokio::task::yield_now().await;
///     req.headers.push(("x-seen-user", user));              // …while mutating the request
///     Ok(())
/// }));
/// let pipeline = Pipeline::new().with(&tag);
/// let out = block_on(pipeline.run(Request::new("/a").with_header("User", "bo"))).unwrap();
/// assert_eq!(out.header("x-seen-user"), Some("bo"));
/// ```
pub fn from_fn_closure_needs_no_annotations() {}

/// ```
/// use lifetimes_2026_10::{checks::block_on, p10_pipeline::*};
/// let allowed = vec![String::from("ann")]; // borrowed by the middleware below
/// let auth = from_fn(|req| {
///     let ok = req.header("user").is_some_and(|u| allowed.iter().any(|a| a == u));
///     Box::pin(async move { if ok { Ok(()) } else { Err(Reject::NotFound) } })
/// });
/// let pipeline = Pipeline::new().with(&auth);
/// assert!(block_on(pipeline.run(Request::new("/").with_header("user", "ann"))).is_ok());
/// assert_eq!(block_on(pipeline.run(Request::new("/"))), Err(Reject::NotFound));
/// drop(pipeline);
/// drop(auth);
/// drop(allowed);
/// ```
pub fn from_fn_closure_may_borrow_local_state() {}

/// ```
/// use lifetimes_2026_10::p10_pipeline::*;
/// fn assert_send<T: Send>(t: T) -> T { t }
/// let rh = RequireHeader { name: "h".into() };
/// let rl = RateLimit::new(1);
/// let pipeline = Pipeline::new().with(&rh).with(&rl);
/// let path = String::from("/x");
/// let fut = assert_send(pipeline.run(Request::new(&path)));
/// let all = assert_send(pipeline.run_all(vec![Request::new(&path)]));
/// drop((fut, all));
/// ```
pub fn pipeline_futures_are_send() {}

/// ```
/// use lifetimes_2026_10::{checks::block_on, p10_pipeline::*};
/// let sp = StripPrefix { prefix: "/api".into() };
/// let out = {
///     let pipeline = Pipeline::new().with(&sp); // the pipeline dies before the result
///     block_on(pipeline.run(Request::new("/api/x")))
/// };
/// assert_eq!(out.unwrap().path, "/x");
/// ```
pub fn results_outlive_the_pipeline() {}

/// ```compile_fail
/// use lifetimes_2026_10::p10_pipeline::*;
/// let value;
/// {
///     let raw = String::from("user");
///     let req = Request::new("/").with_header(&raw, "x");
///     value = req.header("user");
/// }
/// println!("{value:?}");
/// ```
pub fn header_value_cannot_outlive_the_raw_input() {}

/// ```compile_fail
/// use lifetimes_2026_10::p10_pipeline::*;
/// use std::cell::Cell;
/// let calls = Cell::new(0); // Cell is not Sync: a middleware sharing it is not thread-safe
/// let _m = from_fn(|_req| {
///     calls.set(calls.get() + 1);
///     Box::pin(async { Ok(()) })
/// });
/// let _p = Pipeline::new().with(&_m);
/// ```
pub fn from_fn_rejects_non_sync_closures() {}
