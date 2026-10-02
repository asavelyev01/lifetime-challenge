//! HARNESS — do not edit.
//!
//! Handlers here only capture `'static` state (Rc) so this file compiles against both the
//! stub and the fixed API; borrowing handlers are covered by `src/checks/p07.rs`.
use lifetimes_2026_10::p07_dispatcher::*;
use std::cell::RefCell;
use std::rc::Rc;

type Log = Rc<RefCell<Vec<String>>>;

fn recorder(log: &Log, name: &'static str, flow: Flow) -> impl FnMut(&Event<'_>) -> Flow + 'static {
    let log = Rc::clone(log);
    move |e| {
        log.borrow_mut().push(format!("{name}:{e:?}"));
        flow
    }
}

#[test]
fn dispatch_calls_handlers_in_order() {
    let log = Log::default();
    let mut d = Dispatcher::new();
    d.subscribe(recorder(&log, "a", Flow::Continue));
    d.subscribe(recorder(&log, "b", Flow::Continue));
    assert_eq!(d.len(), 2);
    assert_eq!(d.dispatch(&Event::Key('x')), 2);
    assert_eq!(*log.borrow(), ["a:Key('x')", "b:Key('x')"]);
}

#[test]
fn dispatch_on_empty_dispatcher() {
    let mut d = Dispatcher::new();
    assert!(d.is_empty());
    assert_eq!(d.dispatch(&Event::Resize { w: 1, h: 1 }), 0);
}

#[test]
fn stop_short_circuits() {
    let log = Log::default();
    let mut d = Dispatcher::new();
    d.subscribe(recorder(&log, "a", Flow::Continue));
    d.subscribe(recorder(&log, "b", Flow::Stop));
    d.subscribe(recorder(&log, "c", Flow::Continue));
    assert_eq!(d.dispatch(&Event::Click { x: 0, y: -1 }), 2, "a and b called, c not");
    assert_eq!(log.borrow().len(), 2);
}

#[test]
fn handlers_receive_borrowed_text() {
    let log = Log::default();
    let mut d = Dispatcher::new();
    d.subscribe(recorder(&log, "t", Flow::Continue));
    let s = String::from("héllo");
    d.dispatch(&Event::Text(&s));
    assert_eq!(*log.borrow(), ["t:Text(\"héllo\")"]);
}

#[test]
fn handlers_keep_state_between_calls() {
    let count = Rc::new(RefCell::new(0));
    let c = Rc::clone(&count);
    let mut d = Dispatcher::new();
    d.subscribe(move |e| {
        if matches!(e, Event::Key(_)) {
            *c.borrow_mut() += 1;
        }
        if *c.borrow() >= 2 { Flow::Stop } else { Flow::Continue }
    });
    for ch in "abc".chars() {
        d.dispatch(&Event::Key(ch));
    }
    assert_eq!(*count.borrow(), 3, "FnMut state persists");
}

#[test]
fn unsubscribe_removes_only_that_handler_and_ids_are_unique() {
    let log = Log::default();
    let mut d = Dispatcher::new();
    let a = d.subscribe(recorder(&log, "a", Flow::Continue));
    let b = d.subscribe(recorder(&log, "b", Flow::Continue));
    let c = d.subscribe(recorder(&log, "c", Flow::Continue));
    assert!(a != b && b != c && a != c);
    assert!(d.unsubscribe(b));
    assert!(!d.unsubscribe(b), "already removed");
    let e = d.subscribe(recorder(&log, "e", Flow::Continue));
    assert!(e != a && e != b && e != c, "ids are never reused");
    assert_eq!(d.len(), 3);
    d.dispatch(&Event::Key('k'));
    assert_eq!(*log.borrow(), ["a:Key('k')", "c:Key('k')", "e:Key('k')"]);
    assert!(d.unsubscribe(a) && d.unsubscribe(c) && d.unsubscribe(e));
    assert!(d.is_empty());
}

#[test]
fn unsubscribing_handlers_from_another_dispatcher_is_a_no_op() {
    let log = Log::default();
    let mut d1 = Dispatcher::new();
    let mut d2 = Dispatcher::new();
    let _ = d1.subscribe(recorder(&log, "x", Flow::Continue));
    let id = d1.subscribe(recorder(&log, "y", Flow::Continue));
    assert!(!d2.unsubscribe(id));
    assert_eq!(d1.len(), 2);
}

#[test]
fn texts_filters() {
    let events = [
        Event::Text(""),
        Event::Text("ab"),
        Event::Key('q'),
        Event::Text("ée"),
        Event::Text("#tag"),
        Event::Text("abc"),
        Event::Resize { w: 0, h: 0 },
    ];
    let f = Filter { min_chars: 2, skip_prefix: "#".into() };
    assert_eq!(texts(&events, &f).collect::<Vec<_>>(), ["ab", "ée", "abc"], "chars, not bytes");
    let f = Filter { min_chars: 0, skip_prefix: String::new() };
    assert_eq!(texts(&events, &f).collect::<Vec<_>>(), ["", "ab", "ée", "#tag", "abc"], "empty prefix filters nothing");
    let f = Filter { min_chars: 3, skip_prefix: "a".into() };
    assert_eq!(texts(&events, &f).collect::<Vec<_>>(), ["#tag"]);
    assert_eq!(texts(&[], &f).count(), 0);
}
