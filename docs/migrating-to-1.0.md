# Migrating from 0.3 to 1.0

1.0 removes the abbreviated constructors, makes absolute setters take plain values, and makes
arithmetic between rules follow dateutil. The serialized shape of `RelativeDelta` is unchanged, so
stored rules keep working. The MSRV stays at 1.88.

## Update `Cargo.toml`

```toml
[dependencies]
relativedelta = { version = "1", features = ["chrono"] }
```

The `jiff` backend is new in 1.0, and the `time` backend now also covers `time::Date`.

## Changes that stop code from compiling

### Absolute setters take a value or an `Option`

`with_year`, `with_month`, `with_day`, `with_hour`, `with_minute`, `with_second`,
`with_nanosecond` and their `and_*` builder counterparts, plus `and_weekday`, take
`impl Into<Option<T>>`. Existing calls with `Some(..)` and `None` still compile; the `Some` can go.

```diff
-RelativeDelta::with_day(Some(1)).and_weekday(Some((Weekday::Fri, 2))).build()
+RelativeDelta::with_day(1).and_weekday((Weekday::Fri, 2)).build()
```

These setters are no longer `const fn`.

### The abbreviated constructors are gone

Chain the named setters instead; each one says which field it sets.

```diff
-RelativeDelta::yysmmsdds(Some(2020), 1, Some(1), 3, None, 12).build()
+RelativeDelta::with_year(2020).and_years(1).and_month(1).and_months(3).and_days(12).build()

-RelativeDelta::hhsmmssss(Some(12), 1, Some(30), 15, Some(45), 20).build()
+RelativeDelta::with_hour(12).and_hours(1).and_minute(30).and_minutes(15).and_second(45).and_seconds(20).build()
```

`Builder::and_yysmmsdds` and `Builder::and_hhsmmssss` go the same way. For the fractional
`ysmsdshsmsssns_f`, build the whole-number rule and multiply it:

```diff
-RelativeDelta::ysmsdshsmsssns_f(1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0).build()
+RelativeDelta::with_years(3).build() * 0.5
```

`Builder::new`, deprecated since 0.3, is removed; call `build()`.

### `Weekday` from a number is fallible

`From<u8>` wrapped silently, so `Weekday::from(8)` was Tuesday. `TryFrom<u8>` accepts `0..=6` from
Monday and returns `InvalidWeekday` otherwise. If the number came from a backend, convert the
backend's weekday directly; `Weekday` implements `From` for `chrono::Weekday`, `time::Weekday` and
`jiff::civil::Weekday`.

```diff
-let weekday = Weekday::from(date.weekday().num_days_from_monday() as u8);
+let weekday = Weekday::from(date.weekday());
```

### Conversion errors have a public type

`TryFrom<RelativeDelta>` for the date-time types returns `relativedelta::ConversionError`, which
code can now name and match on. It is `#[non_exhaustive]`. The variants `InvalidDateComponents` and
`InvalidTimeComponents` are now `InvalidDate` and `InvalidTime`, and a year the target type cannot
represent reports `InvalidDate` instead of `MissingYear`.

## Deprecations

The backend-specific `checked_add_*` methods are deprecated in favour of the generic
`RelativeDelta::checked_add`, which takes every supported type. They will be removed in 2.0.

| 0.3 | 1.0 |
|-----|-----|
| `checked_add_date` | `checked_add` |
| `checked_add_naive_datetime` | `checked_add` |
| `checked_add_datetime` | `checked_add` |
| `checked_add_primitive_datetime` | `checked_add` |
| `checked_add_offset_datetime` | `checked_add` |
| `checked_add_utc_datetime` | `checked_add` |

## Changes in behaviour

These compile unchanged but can give different results. Check code that relies on them.

- **Combining rules keeps absolute fields.** `a + b` used to drop every absolute field and the
  weekday; it now takes each from `b` where set and from `a` otherwise. `a - b` takes them from `a`
  first. Multiplication now keeps the weekday too.
- **Weekdays follow dateutil.** The `nth` occurrence counts the date itself, in both directions:
  `(Weekday::Mon, -1)` on a Monday is that Monday. An `nth` of 0 behaves as 1.
- **`time::OffsetDateTime` keeps its offset.** Adding a rule to a date-time at `+02:00` returned
  the result at `+00:00` with the same wall-clock time.
- **Overflow returns `None`.** `checked_add` on chrono and time types panicked when years, months
  or durations overflowed; it now returns `None`, as it already did for out-of-range dates.
- **Negating a rule negates `months_f`**, the fractional months left over by multiplication.

## Licence

The licence is `MPL-2.0`; 0.3 declared `MPL-2.0+`.
