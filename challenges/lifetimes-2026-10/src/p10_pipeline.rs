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
        Request { path: Cow::Borrowed(path), headers: Vec::new() }
    }

    pub fn with_header(mut self, name: &'r str, value: &'r str) -> Self {
        self.headers.push((name, value));
        self
    }

    // The values are `&'r str` copied out of the Vec, so the result borrows the raw input,
    // not `self` — callers can keep it while mutating the request.
    pub fn header(&self, name: &str) -> Option<&'r str> {
        self.headers.iter().find(|(n, _)| n.eq_ignore_ascii_case(name)).map(|&(_, v)| v)
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

pub struct RateLimit {
    max_per_user: usize,
    counters: Counters,
    // Serialises the read-modify-write on `counters` across `.await`s. It has to be an async
    // mutex: a `std::sync::MutexGuard` held across an await makes the future `!Send`, and
    // `BoxFuture` requires `Send`. (A per-user lock would be finer-grained; one is enough here.)
    gate: tokio::sync::Mutex<()>,
}

impl RateLimit {
    pub fn new(max_per_user: usize) -> Self {
        RateLimit { max_per_user, counters: Counters::default(), gate: tokio::sync::Mutex::new(()) }
    }

    pub fn counters(&self) -> &Counters {
        &self.counters
    }
}

impl Middleware for RequireHeader {
    fn handle<'a, 'r>(&'a self, req: &'a mut Request<'r>) -> BoxFuture<'a, Result<(), Reject>> {
        Box::pin(async move {
            match req.header(&self.name) {
                Some(_) => Ok(()),
                None => Err(Reject::MissingHeader(self.name.clone())),
            }
        })
    }
}

impl Middleware for StripPrefix {
    fn handle<'a, 'r>(&'a self, req: &'a mut Request<'r>) -> BoxFuture<'a, Result<(), Reject>> {
        Box::pin(async move {
            let prefix = self.prefix.as_str();
            match &mut req.path {
                Cow::Borrowed(path) => {
                    // Copy the `&'r str` out of the `&mut &'r str` so the new slice is `&'r`
                    // too (slicing through the `&mut` would only give a short reborrow).
                    let full: &'r str = path;
                    let rest = full.strip_prefix(prefix).ok_or(Reject::NotFound)?;
                    *path = if rest.is_empty() { "/" } else { rest };
                }
                Cow::Owned(path) => {
                    if !path.starts_with(prefix) {
                        return Err(Reject::NotFound);
                    }
                    path.drain(..prefix.len());
                    if path.is_empty() {
                        path.push('/');
                    }
                }
            }
            Ok(())
        })
    }
}

impl Middleware for RateLimit {
    fn handle<'a, 'r>(&'a self, req: &'a mut Request<'r>) -> BoxFuture<'a, Result<(), Reject>> {
        Box::pin(async move {
            let user = req.header("user").unwrap_or("anonymous");
            let _guard = self.gate.lock().await;
            let used = self.counters.get(user).await;
            if used >= self.max_per_user {
                return Err(Reject::RateLimited { user: user.to_string() });
            }
            self.counters.set(user, used + 1).await;
            Ok(())
        })
    }
}

pub struct FnMiddleware<F>(F);

// The bound has to be spelled out on `from_fn` itself: closure signatures are inferred from
// the bound at the call site, and only an explicit higher-ranked `Fn(&'a mut Request<'r>) ->
// BoxFuture<'a, _>` tells the compiler that the returned future may borrow the argument.
pub fn from_fn<F>(f: F) -> FnMiddleware<F>
where
    F: for<'a, 'r> Fn(&'a mut Request<'r>) -> BoxFuture<'a, Result<(), Reject>> + Send + Sync,
{
    FnMiddleware(f)
}

impl<F> Middleware for FnMiddleware<F>
where
    F: for<'a, 'r> Fn(&'a mut Request<'r>) -> BoxFuture<'a, Result<(), Reject>> + Send + Sync,
{
    fn handle<'a, 'r>(&'a self, req: &'a mut Request<'r>) -> BoxFuture<'a, Result<(), Reject>> {
        (self.0)(req)
    }
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

    pub async fn run<'r>(&self, mut req: Request<'r>) -> Result<Request<'r>, Reject> {
        for stage in &self.stages {
            stage.handle(&mut req).await?;
        }
        Ok(req)
    }

    pub async fn run_all<'r>(&self, reqs: Vec<Request<'r>>) -> Vec<Result<Request<'r>, Reject>> {
        futures::future::join_all(reqs.into_iter().map(|req| self.run(req))).await
    }
}
