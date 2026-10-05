//! # P10 — An async middleware pipeline       (difficulty: ★★★★★)
//!
//! Requests borrow from the raw input (`Request<'r>`); middlewares are trait objects with an
//! async, **dyn-compatible** method (so no `async fn` in the trait — it returns a boxed,
//! `Send` future that borrows both the middleware and the request); the pipeline runs them
//! sequentially per request and many requests concurrently.
//!
//! ## Tasks
//!
//! 1. `Request::header(name)` — the value of the first header whose name equals `name`
//!    ASCII-case-insensitively, or `None`. The value borrows the **raw input**, not the
//!    request: middlewares must be able to read a header and then mutate the request while
//!    still holding the value (see `src/checks/p10.rs`). Fix the signature.
//! 2. Implement the middlewares:
//!    * `RequireHeader { name }` — `Err(Reject::MissingHeader(name))` if the header is absent.
//!    * `StripPrefix { prefix }` — if the path starts with `prefix`, remove it (an empty
//!      result becomes `"/"`); otherwise `Err(Reject::NotFound)`. A `Cow::Borrowed` path must
//!      **stay borrowed** (no allocation); an owned path may be modified in place.
//!    * `RateLimit` — the user is the `user` header (or `"anonymous"`). It allows at most
//!      `max_per_user` accepted requests per user, otherwise `Err(Reject::RateLimited { user })`
//!      (rejected requests don't count). The count lives in the provided async [`Counters`]
//!      service: read it with `get`, write it back with `set` (both await internally). It must
//!      be correct when many requests for the same user run **concurrently** — and remember
//!      that the returned future must be `Send`. You may add fields to `RateLimit`.
//! 3. `from_fn(f)` — turns a closure into a middleware, so that one can write
//!    `from_fn(|req| Box::pin(async move { …; Ok(()) }))` **without annotating** the closure.
//!    Add the bounds `from_fn` needs and `impl Middleware for FnMiddleware<F>`.
//!    The closure may borrow local state (it does not have to be `'static`).
//! 4. `Pipeline::run(req)` — runs the stages in order on the request; stops at the first
//!    rejection. `Pipeline::run_all(reqs)` — runs the pipeline on all requests
//!    **concurrently** (on the current task, no spawning), results in input order.
//!
//! Don't change `BoxFuture`, `Reject`, `Middleware`, `Counters`, or the public fields of
//! `Request`, `RequireHeader`, `StripPrefix`.

use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reject {
    MissingHeader(String),
    NotFound,
    RateLimited { user: String },
}

pub trait Middleware: Send + Sync {
    fn handle<'a, 'r>(&'a self, req: &'a mut Request<'r>) -> BoxFuture<'a, Result<(), Reject>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request<'r> {
    pub path: Cow<'r, str>,
    pub headers: Vec<(&'r str, &'r str)>,
}

impl<'r> Request<'r> {
    pub fn new(path: &'r str) -> Self {
        Request {
            path: Cow::Borrowed(path),
            headers: Vec::new(),
        }
    }

    pub fn with_header(mut self, name: &'r str, value: &'r str) -> Self {
        self.headers.push((name, value));
        self
    }

    /// The value of the first header whose name equals `name` ASCII-case-insensitively, or `None`.
    ///
    /// To fix: the value borrows the **raw input** (`'r`), not the request — a middleware must be
    /// able to read a header and then mutate the request while still holding the value, and the
    /// value must stay usable after the request is dropped.
    // TODO(challenge): implement + fix the signature
    pub fn header(&self, name: &str) -> Option<&str> {
        let _ = name;
        todo!("p10: header")
    }
}

/// Provided — don't modify. A "remote" per-user counter service.
#[derive(Debug, Default)]
pub struct Counters {
    inner: Mutex<HashMap<String, usize>>,
}

impl Counters {
    pub async fn get(&self, user: &str) -> usize {
        tokio::task::yield_now().await;
        self.peek(user)
    }

    pub async fn set(&self, user: &str, n: usize) {
        tokio::task::yield_now().await;
        self.inner.lock().unwrap().insert(user.to_string(), n);
    }

    /// Synchronous read, for tests.
    pub fn peek(&self, user: &str) -> usize {
        self.inner.lock().unwrap().get(user).copied().unwrap_or(0)
    }
}

pub struct RequireHeader {
    pub name: String,
}

pub struct StripPrefix {
    pub prefix: String,
}

/// Allows at most `max_per_user` **accepted** requests per user (the `user` header, or
/// `"anonymous"`); further requests get `Err(Reject::RateLimited { user })` and don't count.
///
/// The counts must live in `counters` (read with `Counters::get`, write back with
/// `Counters::set`, both async). Must be correct when many requests for the same user run
/// concurrently, and its future must be `Send`. You may add fields.
// TODO(challenge): add fields if needed
pub struct RateLimit {
    max_per_user: usize,
    counters: Counters,
}

impl RateLimit {
    /// A rate limiter allowing `max_per_user` accepted requests per user, with empty counters.
    // TODO(challenge): adjust if you add fields
    pub fn new(max_per_user: usize) -> Self {
        RateLimit {
            max_per_user,
            counters: Counters::default(),
        }
    }

    pub fn counters(&self) -> &Counters {
        &self.counters
    }
}

/// Ok if the request has the header `self.name` (ASCII-case-insensitive), otherwise
/// `Err(Reject::MissingHeader(self.name.clone()))`. Doesn't modify the request.
// TODO(challenge): implement
impl Middleware for RequireHeader {
    fn handle<'a, 'r>(&'a self, req: &'a mut Request<'r>) -> BoxFuture<'a, Result<(), Reject>> {
        let _ = (req, &self.name);
        todo!("p10: RequireHeader")
    }
}

/// If the path starts with `self.prefix`, removes it (an empty result becomes `"/"`); otherwise
/// `Err(Reject::NotFound)`. Examples with prefix `"/api"`: `"/api/x"` → `"/x"`, `"/api"` → `"/"`,
/// `"/apix"` → `"x"`, `"/v1/api"` → `NotFound`.
///
/// A `Cow::Borrowed` path must **stay borrowed** (a sub-slice of the original input, no
/// allocation); a `Cow::Owned` path may be modified in place.
// TODO(challenge): implement
impl Middleware for StripPrefix {
    fn handle<'a, 'r>(&'a self, req: &'a mut Request<'r>) -> BoxFuture<'a, Result<(), Reject>> {
        let _ = (req, &self.prefix);
        todo!("p10: StripPrefix")
    }
}

/// See [`RateLimit`]. Note that the returned future must be `Send` — think about what you hold
/// across `.await`.
// TODO(challenge): implement
impl Middleware for RateLimit {
    fn handle<'a, 'r>(&'a self, req: &'a mut Request<'r>) -> BoxFuture<'a, Result<(), Reject>> {
        let _ = (req, self.max_per_user, &self.counters);
        todo!("p10: RateLimit")
    }
}

pub struct FnMiddleware<F>(F);

/// Wraps a closure as a [`Middleware`], so one can write
/// `from_fn(|req| Box::pin(async move { …; Ok(()) }))` with **no type annotations** on the
/// closure; the boxed future may borrow `req`. The closure may borrow local state (it doesn't
/// have to be `'static`), but must be thread-safe (the trait requires `Send + Sync`).
///
/// To do: add the bounds this needs, and write `impl Middleware for FnMiddleware<F>`
/// (it just calls the closure).
// TODO(challenge): add bounds (and an `impl Middleware for FnMiddleware<F>`)
pub fn from_fn<F>(f: F) -> FnMiddleware<F> {
    FnMiddleware(f)
}

pub struct Pipeline<'m> {
    stages: Vec<&'m dyn Middleware>,
}

impl Default for Pipeline<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'m> Pipeline<'m> {
    pub fn new() -> Self {
        Pipeline { stages: Vec::new() }
    }

    pub fn with(mut self, stage: &'m dyn Middleware) -> Self {
        self.stages.push(stage);
        self
    }

    /// Runs the stages in order on `req`; returns the (possibly modified) request, or the first
    /// rejection (later stages are not run). With no stages, returns `req` unchanged.
    // TODO(challenge): implement
    pub async fn run<'r>(&self, req: Request<'r>) -> Result<Request<'r>, Reject> {
        let _ = (req, &self.stages);
        todo!("p10: run")
    }

    /// Runs the pipeline on every request **concurrently** (all in flight at the same time, on the
    /// current task — no spawning); results in input order.
    // TODO(challenge): implement
    pub async fn run_all<'r>(&self, reqs: Vec<Request<'r>>) -> Vec<Result<Request<'r>, Reject>> {
        let _ = reqs;
        todo!("p10: run_all")
    }
}
