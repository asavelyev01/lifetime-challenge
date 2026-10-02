//! P07 compile checks. HARNESS — do not edit.

/// ```
/// use lifetimes_2026_10::p07_dispatcher::*;
/// use std::cell::Cell;
/// let clicks = Cell::new(0);
/// let mut log: Vec<String> = Vec::new();
/// {
///     let mut d = Dispatcher::new();
///     d.subscribe(|e| {
///         if let Event::Click { .. } = e { clicks.set(clicks.get() + 1); }
///         Flow::Continue
///     });
///     d.subscribe(|e| { log.push(format!("{e:?}")); Flow::Continue });
///     d.dispatch(&Event::Click { x: 1, y: 2 });
///     d.dispatch(&Event::Key('k'));
/// } // dispatcher (and its borrows of `clicks` and `log`) end here
/// assert_eq!(clicks.get(), 1);
/// assert_eq!(log, ["Click { x: 1, y: 2 }", "Key('k')"]);
/// ```
pub fn handlers_may_borrow_local_state() {}

/// ```
/// use lifetimes_2026_10::p07_dispatcher::*;
/// let mut longest = 0;
/// let mut d = Dispatcher::new();
/// d.subscribe(|e| {
///     if let Event::Text(t) = e { longest = longest.max(t.len()); }
///     Flow::Continue
/// });
/// for i in 0..3 {
///     let msg = "x".repeat(i + 1); // each event borrows a fresh, short-lived String
///     d.dispatch(&Event::Text(&msg));
/// }
/// drop(d);
/// assert_eq!(longest, 3);
/// ```
pub fn events_may_be_shorter_lived_than_the_dispatcher() {}

/// ```
/// use lifetimes_2026_10::p07_dispatcher::*;
/// let events = [Event::Text("hello"), Event::Key('a'), Event::Text("#skip"), Event::Text("ok")];
/// let it = texts(&events, &Filter { min_chars: 2, skip_prefix: "#".to_string() });
/// let found: Vec<&str> = it.collect(); // the Filter temporary is already gone
/// assert_eq!(found, ["hello", "ok"]);
/// ```
pub fn texts_filter_may_be_a_temporary() {}

/// ```compile_fail
/// use lifetimes_2026_10::p07_dispatcher::*;
/// let mut kept: Vec<&str> = Vec::new();
/// let mut d = Dispatcher::new();
/// d.subscribe(|e| {
///     if let Event::Text(t) = e { kept.push(*t); } // the event's data must not escape the call
///     Flow::Continue
/// });
/// d.dispatch(&Event::Text("static"));
/// drop(d);
/// println!("{kept:?}");
/// ```
pub fn handlers_cannot_keep_event_data() {}

/// ```compile_fail
/// use lifetimes_2026_10::p07_dispatcher::*;
/// use std::cell::Cell;
/// let mut d = Dispatcher::new();
/// {
///     let counter = Cell::new(0);
///     d.subscribe(|_| { counter.set(counter.get() + 1); Flow::Continue });
/// }
/// d.dispatch(&Event::Key('x'));
/// ```
pub fn dispatcher_cannot_outlive_borrowed_state() {}

/// ```compile_fail
/// use lifetimes_2026_10::p07_dispatcher::*;
/// let mut log: Vec<String> = Vec::new();
/// let mut d = Dispatcher::new();
/// d.subscribe(|e| { log.push(format!("{e:?}")); Flow::Continue });
/// println!("{}", log.len()); // still mutably borrowed by the handler
/// d.dispatch(&Event::Key('x'));
/// ```
pub fn borrowed_state_is_locked_while_dispatcher_lives() {}

/// ```compile_fail
/// use lifetimes_2026_10::p07_dispatcher::*;
/// let found: Vec<&str>;
/// {
///     let events = vec![Event::Text("a")];
///     found = texts(&events, &Filter::default()).collect();
/// }
/// println!("{found:?}");
/// ```
pub fn texts_cannot_outlive_the_events_slice() {}
