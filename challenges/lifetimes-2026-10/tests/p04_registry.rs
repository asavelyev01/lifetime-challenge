//! HARNESS — do not edit.
use lifetimes_2026_10::p04_registry::*;

fn reg(items: &[(&str, i64, &[&str])]) -> Registry {
    let mut r = Registry::new();
    for (name, score, tags) in items {
        let e = r.get_or_insert(name);
        e.score = *score;
        e.tags = tags.iter().map(|t| t.to_string()).collect();
    }
    r
}

fn snapshot(r: &Registry) -> Vec<(String, i64)> {
    r.entries()
        .iter()
        .map(|e| (e.name.clone(), e.score))
        .collect()
}

fn s(items: &[(&str, i64)]) -> Vec<(String, i64)> {
    items.iter().map(|(n, v)| (n.to_string(), *v)).collect()
}

#[test]
fn statement_example() {
    let mut r = Registry::new();
    r.get_or_insert("ann").score += 5;
    r.get_or_insert("bob");
    assert_eq!(snapshot(&r), s(&[("ann", 5), ("bob", 0)]));
    r.transfer("ann", "bob", 2).unwrap();
    assert_eq!(snapshot(&r), s(&[("ann", 3), ("bob", 2)]));
    assert_eq!(r.merge("ann", "bob").unwrap().score, 5);
    assert_eq!(snapshot(&r), s(&[("bob", 5)]));
}

#[test]
fn get_and_get_mut() {
    let mut r = reg(&[("a", 1, &[]), ("b", 2, &[])]);
    assert_eq!(r.get("b").map(|e| e.score), Some(2));
    assert!(r.get("c").is_none());
    r.get_mut("a").unwrap().score = 10;
    assert!(r.get_mut("zzz").is_none());
    assert_eq!(snapshot(&r), s(&[("a", 10), ("b", 2)]));
}

#[test]
fn get_or_insert_returns_existing_without_duplicating() {
    let mut r = Registry::new();
    r.get_or_insert("x").score = 4;
    let e = r.get_or_insert("x");
    assert_eq!(e.score, 4, "must return the existing entry");
    e.tags.push("t".into());
    assert_eq!(r.len(), 1);
    assert_eq!(r.get("x").unwrap().tags, ["t"]);
    let fresh = r.get_or_insert("y");
    assert_eq!(
        fresh,
        &mut Entry {
            name: "y".into(),
            score: 0,
            tags: vec![]
        }
    );
    assert_eq!(r.len(), 2);
}

#[test]
fn pair_mut_cases() {
    let mut r = reg(&[("a", 1, &[]), ("b", 2, &[]), ("c", 3, &[])]);
    {
        let (c, a) = r.pair_mut("c", "a").expect("both exist");
        assert_eq!(
            (c.name.as_str(), a.name.as_str()),
            ("c", "a"),
            "argument order"
        );
        c.score += a.score;
        a.score = 0;
    }
    assert_eq!(snapshot(&r), s(&[("a", 0), ("b", 2), ("c", 4)]));
    assert!(r.pair_mut("a", "a").is_none(), "same entry twice");
    assert!(r.pair_mut("a", "x").is_none());
    assert!(r.pair_mut("x", "a").is_none());
    let (a, b) = r.pair_mut("a", "b").unwrap();
    assert_eq!((a.score, b.score), (0, 2));
}

#[test]
fn transfer_errors_in_order_and_leave_state_unchanged() {
    use RegistryError::*;
    let mut r = reg(&[("a", 5, &[]), ("b", 0, &[])]);
    let before = snapshot(&r);
    assert_eq!(r.transfer("a", "b", 0), Err(InvalidAmount));
    assert_eq!(r.transfer("a", "b", -1), Err(InvalidAmount));
    assert_eq!(
        r.transfer("x", "x", 1),
        Err(SameEntry),
        "SameEntry before Unknown"
    );
    assert_eq!(
        r.transfer("x", "y", 1),
        Err(Unknown("x".into())),
        "from checked first"
    );
    assert_eq!(r.transfer("a", "y", 1), Err(Unknown("y".into())));
    assert_eq!(r.transfer("b", "a", 1), Err(Insufficient { available: 0 }));
    assert_eq!(r.transfer("a", "b", 6), Err(Insufficient { available: 5 }));
    assert_eq!(snapshot(&r), before);
    assert_eq!(r.transfer("a", "b", 5), Ok(()), "exactly the whole balance");
    assert_eq!(snapshot(&r), s(&[("a", 0), ("b", 5)]));
}

#[test]
fn merge_moves_score_and_new_tags() {
    let mut r = reg(&[
        ("a", 1, &["x", "y"]),
        ("b", 2, &[]),
        ("c", 3, &["y", "z"]),
        ("d", 4, &[]),
    ]);
    let merged = r.merge("a", "c").unwrap();
    assert_eq!(merged.name, "c");
    assert_eq!(merged.score, 4);
    assert_eq!(
        merged.tags,
        ["y", "z", "x"],
        "only missing tags appended, in from's order"
    );
    assert_eq!(
        snapshot(&r),
        s(&[("b", 2), ("c", 4), ("d", 4)]),
        "order preserved"
    );
}

#[test]
fn merge_into_an_earlier_entry() {
    let mut r = reg(&[("a", 1, &[]), ("b", 2, &["t"]), ("c", 3, &[])]);
    let merged = r.merge("c", "a").unwrap();
    assert_eq!((merged.name.as_str(), merged.score), ("a", 4));
    let merged = r.merge("b", "a").unwrap();
    assert_eq!(
        (merged.name.as_str(), merged.score, merged.tags.clone()),
        ("a", 6, vec!["t".to_string()])
    );
    assert_eq!(snapshot(&r), s(&[("a", 6)]));
}

#[test]
fn merge_errors() {
    use RegistryError::*;
    let mut r = reg(&[("a", 1, &[]), ("b", 2, &[])]);
    assert_eq!(r.merge("a", "a").err(), Some(SameEntry));
    assert_eq!(r.merge("q", "q").err(), Some(SameEntry));
    assert_eq!(r.merge("x", "y").err(), Some(Unknown("x".into())));
    assert_eq!(r.merge("a", "y").err(), Some(Unknown("y".into())));
    assert_eq!(snapshot(&r), s(&[("a", 1), ("b", 2)]));
}

#[test]
fn top_by_tag_picks_max_then_earliest() {
    let r = reg(&[
        ("a", 5, &["x"]),
        ("b", 9, &["y"]),
        ("c", 9, &["x", "y"]),
        ("d", 9, &["x"]),
        ("e", -1, &["z"]),
    ]);
    assert_eq!(r.top_by_tag("x").unwrap().name, "c");
    assert_eq!(r.top_by_tag("y").unwrap().name, "b");
    assert_eq!(
        r.top_by_tag("z").unwrap().name,
        "e",
        "negative scores still count"
    );
    assert!(r.top_by_tag("w").is_none());
    assert!(Registry::new().top_by_tag("x").is_none());
}

#[test]
fn randomized_against_a_model() {
    use rand::{Rng, SeedableRng, rngs::SmallRng};
    let mut rng = SmallRng::seed_from_u64(4);
    let names = ["a", "b", "c", "d", "e"];
    for _ in 0..200 {
        let mut r = Registry::new();
        let mut model: Vec<(String, i64)> = vec![];
        for _ in 0..40 {
            let x = names[rng.random_range(0..names.len())];
            let y = names[rng.random_range(0..names.len())];
            match rng.random_range(0..3) {
                0 => {
                    let amt = rng.random_range(1..5);
                    r.get_or_insert(x).score += amt;
                    match model.iter_mut().find(|(n, _)| n == x) {
                        Some(e) => e.1 += amt,
                        None => model.push((x.into(), amt)),
                    }
                }
                1 => {
                    let amt = rng.random_range(-1..6);
                    let ok = r.transfer(x, y, amt).is_ok();
                    let fi = model.iter().position(|(n, _)| n == x);
                    let ti = model.iter().position(|(n, _)| n == y);
                    let want = match (fi, ti) {
                        (Some(f), Some(t)) if amt > 0 && f != t && model[f].1 >= amt => {
                            model[f].1 -= amt;
                            model[t].1 += amt;
                            true
                        }
                        _ => false,
                    };
                    assert_eq!(ok, want, "transfer({x}, {y}, {amt})");
                }
                _ => {
                    let ok = r.merge(x, y).is_ok();
                    let fi = model.iter().position(|(n, _)| n == x);
                    let ti = model.iter().position(|(n, _)| n == y);
                    let want = matches!((fi, ti), (Some(f), Some(t)) if f != t);
                    if want {
                        let (_, sc) = model.remove(fi.unwrap());
                        model.iter_mut().find(|(n, _)| n == y).unwrap().1 += sc;
                    }
                    assert_eq!(ok, want, "merge({x}, {y})");
                }
            }
            assert_eq!(snapshot(&r), model);
        }
    }
}
