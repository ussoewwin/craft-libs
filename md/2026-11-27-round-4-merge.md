# The fourth round of 2026-11-27 in craft-libs

## What was merged

`storytold/craft-libs` `c0977ad61` — 255 commits over `9524262a8`, the point this fork took in the
third round — merged into `main` as `32463d74a`, nothing left behind (`git rev-list --count
HEAD..upstream/main` = 0). One file conflicted.

## The conflict: the picker's sample text

`crates/font-manager/src/picker/rows.rs`. Upstream moved `SampleText` into the parent module and gave
it a key and a label, so a host's sample texts can be listed by name; the fork had kept its own
`SampleText { text, auto }` in this file, where `auto` said whether the picker chose the sample word
itself. The two shapes cannot share a name.

The fork's flag is still needed: `FamilyInfos::sample_text` takes it and returns the kana sample only
for an automatic one, so a host that names its own preview text keeps that text in every row. The
merge therefore keeps upstream's type and passes the flag beside it, as a small local `RowText<'_>`
that also keeps both functions at seven arguments, where the workspace's clippy setting stops.

## The design lint's six findings, which are upstream's own

The workspace lint read the merged tree and named six places upstream added: a handle side and a
window's opening size (both sizes in points, as a component field), a focus-recovery pass that reads
egui's memory and a raw Tab key, a selection layer's own click state, and a public struct with public
fields.

Five are registered as exceptions beside a marker in the code (`design/exceptions/craft-draw/
transform_box.toml`, `craft-ui-patterns/preferences.toml`, `craft-ui-patterns/form_dialogs.toml`,
`craft-ui-widgets/text.toml`), each with the reason that makes the site legitimate rather than a
rewrite of upstream's behaviour: a host's preference overrides a theme token, a window opens at the
layout's size until the user resizes it, focus recovery asks what arrived in this pass. The sixth,
`CheckContext`, takes `#[non_exhaustive]`, which is what the rule asks for.

## Measured

| what | result |
| fork crates (12) | 3,394 run, 3,394 passed |
| workspace suite | 19,689 run, 19,689 passed, 66 skipped (973 s) |
| format, clippy | 0, 0 |
| design lint | 28 passed |
| merge check | 0 dropped upstream files, 0 lost fork files |
| merge base | `c0977ad61`, behind 0 |
