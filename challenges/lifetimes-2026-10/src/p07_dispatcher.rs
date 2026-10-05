//! # P07 — Handlers that borrow              (difficulty: ★★★☆☆)
//!
//! An event dispatcher stores boxed closures. Two kinds of borrowing meet here:
//!
//! * **events** borrow short-lived data (`Event::Text(&str)`), and one dispatcher must accept
//!   events of *any* lifetime — a handler sees an event only for the duration of the call;
//! * **handlers** may borrow from their environment (a log, a counter, a config…) — the
//!   dispatcher must then not outlive what its handlers borrow.
//!
//! Right now `Box<dyn FnMut(..)>` silently means `Box<dyn FnMut(..) + 'static>`, so handlers
//! can't borrow anything. Fix the types so the checks in `src/checks/p07.rs` compile (and the
//! `compile_fail` ones keep failing), then implement everything.
//!
//! The free function `texts` has a separate issue: in edition 2024 a returned `impl Trait`
//! captures *every* lifetime in scope — including the `filter` borrow, which the iterator
//! shouldn't need to keep. Fix it so the filter may be a temporary.
//!
//! ## Dispatcher semantics
//!
//! * `subscribe(handler)` — appends a handler, returns a fresh `HandlerId` (ids are never
//!   reused, even after unsubscribing).
//! * `unsubscribe(id)` — removes that handler; `true` if it was present.
//! * `dispatch(&event)` — calls the handlers in subscription order; if one returns
//!   `Flow::Stop`, later handlers are not called. Returns the number of handlers called.
//! * `len()` — number of subscribed handlers.
//!
//! ## `texts(events, filter)`
//!
//! Yields, in order, the payloads of the `Event::Text` events whose payload has at least
//! `filter.min_chars` chars (`chars().count()`) and does not start with `filter.skip_prefix`
//! (an empty `skip_prefix` filters nothing).
//!
//! Keep type and function names, and keep `subscribe`/`dispatch` callable as
//! `d.subscribe(|e| ...)` / `d.dispatch(&event)`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event<'a> {
    Key(char),
    Click { x: i32, y: i32 },
    Text(&'a str),
    Resize { w: u32, h: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Continue,
    Stop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HandlerId(u64);

/// An ordered list of event handlers.
///
/// To fix: `Box<dyn FnMut(..)>` means `Box<dyn FnMut(..) + 'static>`, so handlers can't borrow
/// anything from their environment. Handlers must be allowed to borrow local state (the
/// dispatcher then can't outlive it), while still accepting events of **any** lifetime.
// TODO(challenge): fix the types
#[derive(Default)]
pub struct Dispatcher {
    handlers: Vec<(HandlerId, Box<dyn FnMut(&Event<'_>) -> Flow>)>,
    next_id: u64,
}

// TODO(challenge): fix the types + implement
impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            handlers: Vec::new(),
            next_id: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }

    /// Appends `handler` and returns a fresh [`HandlerId`]. Ids are unique per dispatcher and never
    /// reused, even after unsubscribing.
    ///
    /// To fix: accept closures that borrow local state (`&mut Vec`, `&Cell`, …), not only `'static`
    /// ones. Must stay callable as `d.subscribe(|e| …)` without annotations. A handler must not be
    /// able to keep the event's data after the call returns.
    pub fn subscribe(&mut self, handler: impl FnMut(&Event<'_>) -> Flow + 'static) -> HandlerId {
        let _ = (handler, self.next_id);
        todo!("p07: subscribe")
    }

    /// Removes the handler with this id. Returns `true` if it was present (`false` if it was
    /// already removed or never belonged to this dispatcher). Other handlers keep their order.
    pub fn unsubscribe(&mut self, id: HandlerId) -> bool {
        let _ = id;
        todo!("p07: unsubscribe")
    }

    /// Calls the handlers with `event` in subscription order. If a handler returns `Flow::Stop`,
    /// later handlers are not called. Returns the number of handlers called.
    ///
    /// `event` may borrow data that lives only for this call (e.g. a `String` created in a loop).
    pub fn dispatch(&mut self, event: &Event<'_>) -> usize {
        let _ = event;
        todo!("p07: dispatch")
    }
}

#[derive(Debug, Clone, Default)]
pub struct Filter {
    pub min_chars: usize,
    pub skip_prefix: String,
}

/// Lazily yields, in order, the payloads of the `Event::Text` events whose payload has at least
/// `filter.min_chars` chars and does not start with `filter.skip_prefix` (an empty
/// `skip_prefix` filters nothing).
///
/// To fix: in edition 2024 the returned `impl Iterator` captures the `filter` borrow too, so the
/// filter can't be a temporary. The iterator must borrow only `events`.
// TODO(challenge): implement + fix the signature
pub fn texts<'e>(events: &'e [Event<'e>], filter: &Filter) -> impl Iterator<Item = &'e str> {
    let _ = (events, filter);
    std::iter::from_fn(|| todo!("p07: texts"))
}
