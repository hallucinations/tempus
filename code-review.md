# Code Review: `period` v0.6.0-alpha.1

**Reviewer:** Claude Code
**Date:** 2026-02-22
**Scope:** Full codebase review — all `src/` modules

---

## Summary

The library is well-structured and correct. No critical bugs were found. The code
consistently applies safe arithmetic (`Duration::try_*`, `checked_*`), avoids heap
allocation on the error path, and ships strong test coverage (~150 tests). The
findings below are minor issues and design observations only.

**Verdict: Approved with minor notes.**

---

## Bugs / Correctness Issues

### Issue 1 — `humanize`: jarring bucket transition at 90 s (minor)

**Location:** `src/relative/humanize.rs`

The bucketing logic produces:

| Range | Output |
|-------|--------|
| 30–89 s | `"a minute ago"` (article form) |
| 90–119 s | `"1 minute ago"` (numeric singular) |
| 120+ s | `"2 minutes ago"` etc. |

At the 90-second boundary the output flips from `"a minute ago"` to `"1 minute
ago"`. This is intentional (matches ActiveSupport's `time_ago_in_words`) but is
not documented in the threshold table in the doc comment. A reader of the table
would not predict the 90 s behaviour without reading the implementation.

**Recommendation:** Add a note to the `# Thresholds` doc comment table at the
`< 90 s → "a minute ago"` row:

```
/// | < 90 seconds  | "a minute ago"   | "in a minute"   |
/// |               | (90–119 s shows "1 minute ago" — numeric singular) |
```

---

### Issue 2 — `humanize`: "yesterday"/"tomorrow" triggers at 22 h, not 24 h (minor)

**Location:** `src/relative/humanize.rs`

A datetime 22–35 h in the past returns `"yesterday"` even when it still falls on
the same calendar day (e.g., 11 PM yesterday vs. today at 2 AM is only 3 h but
would return `"yesterday"` for anything > 22 h). The threshold is intentional and
documented in the CLAUDE.md roadmap table, but the *function-level* doc comment
does not explain it.

**Recommendation:** Add a note to the `# Thresholds` section clarifying that
`"yesterday"` / `"tomorrow"` refer to elapsed seconds, not calendar-day boundaries.

---

### Issue 3 — `years_ago` / `years_from_now`: `saturating_mul` could be replaced with `checked_mul` (cosmetic)

**Location:** `src/relative/functions/year.rs:20`, `year.rs:46`

```rust
let months = u32::try_from(years.saturating_mul(12)).map_err(|_| PeriodError::Overflow { ... })?;
```

When `years` is in the range `(i64::MAX / 12, i64::MAX]`, `saturating_mul(12)`
silently clamps to `i64::MAX` instead of the true product. The subsequent
`u32::try_from` still rejects the clamped value and returns `Overflow`, so
observable behaviour is correct. However, using `checked_mul` would make the
intent explicit and avoid the silent saturation:

```rust
// Preferred — intent is clear:
let months_i64 = years.checked_mul(12).ok_or(PeriodError::Overflow { unit: "years", value: years })?;
let months = u32::try_from(months_i64).map_err(|_| PeriodError::Overflow { unit: "years", value: years })?;
```

---

## Test Coverage Issues

### Issue 4 — Future-facing `humanize` tests rely on ad-hoc time buffers (low risk)

**Location:** `src/relative/humanize.rs` tests (multiple `future_dt` helpers)

Several tests compute a future timestamp, then call `humanize` and assert the
result. Because `humanize` internally calls `Local::now()`, any non-zero elapsed
time between setup and assertion shifts the effective delta. The tests compensate
by adding a 30-second buffer (e.g., `future_dt(45 * 60 + 30)` to test the 45-min
threshold). Under heavy system load this buffer could be exhausted, causing a
spurious failure.

**Recommendation (longer term):** Extract the core bucketing logic into a private
`humanize_with_now(datetime, now)` function. The public `humanize` wrapper calls
it with `Local::now()`. Tests call the private variant with a fixed `now`, making
them deterministic and removing all time buffers.

---

### Issue 5 — Magic numbers in overflow tests (cosmetic)

**Location:** `src/relative/functions/day.rs:119`, `day.rs:156`

```rust
assert!(days_ago(200_000_000).is_err());
assert!(days_from_now(200_000_000).is_err());
```

The value `200_000_000` is not explained. A reader cannot tell whether this was
chosen to be above chrono's internal limit, above `i64::MAX / SECONDS_PER_DAY`,
or just "a big number". A comment linking it to the chrono epoch limit would
clarify intent:

```rust
// chrono's NaiveDate range is ~±262,000 years (~95,000,000 days).
// 200_000_000 days exceeds that limit and must overflow.
assert!(days_ago(200_000_000).is_err());
```

---

### Issue 6 — No test for `humanize` at extreme input values (gap)

**Location:** `src/relative/humanize.rs` tests

The `>= 18 months` (years) branch is tested with inputs of ~19 months and 3 years,
but not with values large enough to exercise `secs.saturating_abs()` (i.e., inputs
near `DateTime<Local>` min/max). This is a low-risk gap since `saturating_abs` on
`i64::MIN` produces `i64::MAX`, which simply outputs a very large year count rather
than panicking. A test asserting that `humanize` returns `"N years ago"` for an
extreme past date would close the gap.

---

## Design Observations (Non-blocking)

### Observation A — `humanize` calls `Local::now()` internally

The function signature `pub fn humanize(datetime: DateTime<Local>) -> String`
computes the current time internally. This is the simplest API, but it couples the
function to wall-clock time, limits deterministic testing, and prevents callers
from computing relative strings against a reference time other than "right now".

A future refactor could introduce:

```rust
// Private — testable, accepts explicit reference time
fn humanize_impl(datetime: DateTime<Local>, now: DateTime<Local>) -> String { ... }

// Public — calls humanize_impl with Local::now()
pub fn humanize(datetime: DateTime<Local>) -> String {
    humanize_impl(datetime, Local::now())
}
```

This is a non-breaking addition. It would allow all existing tests to switch from
wall-clock-relative assertions to deterministic ones.

### Observation B — `MONTH = 30 * DAY` approximation in `humanize`

The `humanize` function uses a fixed 30-day month (2,592,000 s). This is
intentional, consistent with ActiveSupport, and documented in the CLAUDE.md
threshold table. Callers who need calendar-accurate month boundaries should use
`months_ago` / `months_from_now` instead.

### Observation C — `yesterday()` / `tomorrow()` use `expect()`

```rust
// day.rs:67
Local::now().date_naive().pred_opt().expect("date underflow")
```

The `expect` is appropriate here — `pred_opt` returns `None` only on
`NaiveDate::MIN` (year −262144), which cannot occur in practice. The `# Panics`
doc section is correctly written. No action required.

---

## Strengths

- **Safe arithmetic throughout.** Every fallible computation uses `Duration::try_*`
  or `checked_*`. No `unwrap` in production paths.
- **Zero-heap allocation on error path.** All `PeriodError` fields are `&'static str`
  or primitive integers.
- **`#[non_exhaustive]` on `PeriodError`.** Forward-compatible; callers must handle
  future variants.
- **Symmetrical API.** Every relative function has both `_ago` and `_from_now`
  variants. Negative-value errors suggest the opposite function by name.
- **`Relative` wrapper.** Clean separation between timestamp storage and extraction
  (`.as_date()`, `.as_datetime()`, `.as_time()`). `Copy` + `From` impls make it
  ergonomic.
- **Test breadth.** Spot-check correctness, zero, negative, overflow, temporal
  ordering, cross-unit equivalence, and round-trips are all covered.
- **Strict linting.** `clippy::pedantic` with warnings-as-errors prevents
  accumulation of lint debt.
