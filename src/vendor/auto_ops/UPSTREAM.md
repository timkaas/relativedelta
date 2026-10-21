# Vendored `auto_ops` source

This directory contains source derived from [`auto_ops`](https://github.com/carbotaniuman/auto_ops), copied from commit [`74e36f039937478d717cf1155b5301470a19c63f`](https://github.com/carbotaniuman/auto_ops/commit/74e36f039937478d717cf1155b5301470a19c63f).

The upstream source is copyright (c) 2019 Quy Nguyen and is licensed under the MIT License. The full license text is in [`LICENSE.txt`](LICENSE.txt).

## Local adaptations

- Integrated the upstream source as an internal module of `relativedelta`; the crate-level `#![no_std]` attribute is provided by the parent crate.
- Removed upstream `#[macro_export]` declarations and used `#[macro_use]` on the internal modules so the macros remain scoped to this crate.
- Adjusted helper macro invocations to work with lexical macro scoping.
- Added scoped lint allowances for upstream macro definitions and arms that this crate does not use.
- Marked upstream usage examples as ignored doctests where they depend on importing `auto_ops` as a separate crate; the vendored macros are intentionally crate-private.
