# Rust lifetimes — 10 problems (2026-10)

**Brief:** Rust lifetimes, with a focus on structs and enums that borrow, async code, and the
type-system side: signatures and type definitions that have to compile for the right callers
and fail to compile for the wrong ones. Problems get harder as you go, from fixing
over-constrained signatures up to GAT lending iterators and an async middleware pipeline.

| #   | Problem                              | Concept                                                                 | Difficulty |
|-----|--------------------------------------|-------------------------------------------------------------------------|------------|
| p01 | `p01_signatures` — relax the signatures | input/output lifetime relationships, `'static` returns                  | ★☆☆☆☆ |
| p02 | `p02_lexer` — zero-copy lexer        | struct + enum borrowing a source, `Iterator` items that outlive `self`  | ★☆☆☆☆ |
| p03 | `p03_fields` — Cow fields            | `Cow<'a, str>` in enums, `into_owned() -> T<'static>`                   | ★★☆☆☆ |
| p04 | `p04_registry` — mutable borrows     | `&mut` returns, conditional borrows, two disjoint `&mut`, no `unsafe`   | ★★☆☆☆ |
| p05 | `p05_scanner` — two lifetimes        | a struct that needs two lifetime parameters                             | ★★★☆☆ |
| p06 | `p06_interner` — borrowed forever    | the `&'a mut self` anti-pattern, invariance, covariance                 | ★★★☆☆ |
| p07 | `p07_dispatcher` — handlers that borrow | `dyn Trait + 'h`, HRTB over borrowed events, edition-2024 `impl Trait` capture | ★★★☆☆ |
| p08 | `p08_async_store` — futures that borrow | what futures capture, borrowed concurrency, `'static` tasks, async closures | ★★★★☆ |
| p09 | `p09_lending` — lending iterators    | GATs, and why an HRTB over a GAT can demand `'static`                   | ★★★★★ |
| p10 | `p10_pipeline` — async middleware    | dyn-compatible async traits, `Send` futures, HRTB closures, `Cow` paths  | ★★★★★ |

## How it works

Each problem has two kinds of checks:

* **Runtime tests:** `tests/pNN_*.rs`. Behaviour, edge cases, randomized oracles, and a perf
  test in p06.
* **Compile checks:** doctests in `src/checks/pNN.rs`. Plain blocks must **compile and
  run**: they show how the API has to be usable. `compile_fail` blocks must be **rejected** by
  the compiler, which keeps the API sound. Some `compile_fail` checks already pass on the
  stubs. They're there to stop "fixes" that loosen the API too much.

Read the compile checks. For several problems they *are* the spec of what has to compile.

## Running

```sh
cd challenges/lifetimes-2026-10
./run.sh                 # everything
./run.sh p05             # one problem: runtime tests + compile checks

# or directly:
cargo test -p lifetimes-2026-10 --test p05_scanner                    # runtime tests of one problem
cargo test -p lifetimes-2026-10 --test p05_scanner -- keyword_of      # one runtime test
cargo test -p lifetimes-2026-10 --doc -- checks::p05::                # compile checks of one problem
cargo test -p lifetimes-2026-10 --doc -- checks::p05::spans_outlive   # one compile check
```

## Rules

* Edit only the problem files `src/pNN_*.rs`. Everything to do is marked
  `grep -rn "TODO(challenge)" src/`.
* Don't edit `tests/`, `src/checks/`, `src/lib.rs`, or `run.sh`.
* You may change signatures, struct fields, impls, and add private helpers where the problem
  says so. Keep public names so the harness compiles.
* No `unsafe` (`#![forbid(unsafe_code)]`), no `Box::leak`/`mem::forget` tricks, no
  `'static` cheats.

Reference solutions: `REF_SHA` (**spoilers**: `git show REF_SHA`)
Start of the challenge: `START_SHA`
