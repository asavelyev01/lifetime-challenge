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

/// A boxed handler that may borrow from its environment for `'h`, and that accepts events of
/// *any* lifetime (`&Event<'_>` in `Fn` sugar is higher-ranked: `for<'e> FnMut(&Event<'e>)`).
type Handler<'h> = Box<dyn FnMut(&Event<'_>) -> Flow + 'h>;

#[derive(Default)]
pub struct Dispatcher<'h> {
    handlers: Vec<(HandlerId, Handler<'h>)>,
    next_id: u64,
}

impl<'h> Dispatcher<'h> {
    pub fn new() -> Self {
        Dispatcher { handlers: Vec::new(), next_id: 0 }
    }

    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }

    pub fn subscribe(&mut self, handler: impl FnMut(&Event<'_>) -> Flow + 'h) -> HandlerId {
        let id = HandlerId(self.next_id);
        self.next_id += 1;
        self.handlers.push((id, Box::new(handler)));
        id
    }

    pub fn unsubscribe(&mut self, id: HandlerId) -> bool {
        let before = self.handlers.len();
        self.handlers.retain(|(h, _)| *h != id);
        self.handlers.len() != before
    }

    pub fn dispatch(&mut self, event: &Event<'_>) -> usize {
        let mut called = 0;
        for (_, handler) in &mut self.handlers {
            called += 1;
            if handler(event) == Flow::Stop {
                break;
            }
        }
        called
    }
}

#[derive(Debug, Clone, Default)]
pub struct Filter {
    pub min_chars: usize,
    pub skip_prefix: String,
}

// Edition 2024: `impl Trait` in return position captures every lifetime in scope unless told
// otherwise. `use<'e>` limits it to `'e`, which is only sound because we copy what we need
// out of `filter` before building the iterator.
pub fn texts<'e>(events: &'e [Event<'e>], filter: &Filter) -> impl Iterator<Item = &'e str> + use<'e> {
    let Filter { min_chars, skip_prefix } = filter.clone();
    events.iter().filter_map(move |e| match *e {
        Event::Text(t)
            if t.chars().count() >= min_chars && (skip_prefix.is_empty() || !t.starts_with(&skip_prefix)) =>
        {
            Some(t)
        }
        _ => None,
    })
}
