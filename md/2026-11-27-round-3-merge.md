# The third round of 2026-11-27 in craft-libs

## What was merged

`storytold/craft-libs` `9524262a8` — 646 commits over the fork's base — merged into the fork's
`main` as a single merge commit with no conflict, the fork's own crates, their trials and its Space
rule kept. Upstream retired its `next` line in the same period (`next` at `a46af76c7` is an ancestor
of `9524262a8`, and the branch is gone), so both repositories now follow `main` only. The fork's
branch was renamed to `main` to match, and the merge measured `behind 0`.

## The fork's rules, carried onto the new API

Six files changed where upstream's contract moved under the fork's own behaviour:

- `printers_via_checked` keeps the `lpstat` exit status and stderr inside one absolute deadline
  (`output_within_full`), so a failed listing is a failure rather than a system without printers.
- `PressRoute::Space` holds the stop through `space_held::hold` (a repeat does not hold a second
  time), and the release is collected outside the target branch so a viewport that lost focus still
  releases its hold.
- `input.rs` no longer requires a modifier on the Space release, so releasing with Shift down does
  not leave a record held.
- `host_words` returns to `Region::system()` instead of pinning en-US; the weekday headings in the
  date picker's trial are read from `DateFormat::new(&Region::system())`, which keeps the trial
  independent of the machine's locale.
- `rank.rs` counts a CFF glyph's drawing commands through skrifa's `OutlinePen` instead of treating
  an empty CharString as ink.

## The design lint's exemptions, which no longer cover anything

Upstream rewrote the dialog trial that the previous round's exemptions named, so `design-lint.toml`
and `design/exceptions/craft-ui-panels/dialog.toml` covered zero sites and the lint reported them as
`exception/stale`. Both were removed and the empty `dialog.toml` restored: 28 passed.

## The eighth-bit RGBA oracle, measured against FFmpeg itself

The `ffv1` trial's 8-bit RGBA case failed on its alpha channel. The cause is in FFmpeg's own
`gbrap` path, not in this encoder: encoding the same 8-bit RGBA frame and decoding it back with the
same FFmpeg binary leaves 2,073 of 4,608 alpha samples one level apart between the `bgra`/`rgba`
output and the `gbrap` output. FFmpeg 7.1's ffv1 encoder lists `bgra`, `bgr0` and the 10-to-16-bit
`gbrap` formats, but no 8-bit `gbrap`. The trial now asks for `rgba` in that one case and compares
`raw_bytes_interleaved`; every other case keeps the planar comparison.

## The formula printer, off the call stack

`craft-xlsx`'s long-chain trial aborted the process with 0xC00000FD on a 256 KiB thread. A staged
probe over six runs put the abort between `decode` and the end of `print`, which pointed at
`xlsb::expr::write_expr`: one frame per nesting level, and a chain of thousands of `&` operands is
thousands of levels. `write_expr` now walks an explicit stack of nodes, operator text and single
characters, so a formula's depth costs heap, not call stack; `write_args` disappeared with it. The
trial passed six runs out of six afterwards, and `craft-xlsx` is 110 of 110.

## Measured

| what | result |
| workspace suite | 19,445 run, 19,445 passed, 66 skipped |
| `craft-xlsx` | 110 passed |
| `craft-video` FFV1 trial | 18 passed |
| format, clippy | 0, 0 |
| design lint | 28 passed |
| merge base | `9524262a8`, behind 0 |
| the `--all-features` failures | `h264_encode::high10_oracle`, `hevc_decode::conformance::bframes`, `draft_mode_changes_only_flagged_non_reference_pictures` — the same three fail on the tree from before this round's changes, and the release gate runs the default features |
