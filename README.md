# record-plant

`record-plant` is the Bitneedle crate for physical vinyl manufacturing proof
schema, guide rendering, and plant-facing preflight rules.

Curated manufacturer/template facts live in
[`fixtures/record-plant-registry.json`](fixtures/record-plant-registry.json),
not in Rust. The crate takes supplied template JSON and returns deterministic
proof overlays, manifests, record-context previews, and preflight policy.

## Scope

- JSON schema for plant templates with source URLs and retrieval dates.
- Physical geometry in millimeters for trim, bleed, safety, folds, spines, and
  center holes.
- Basic guide export as SVG for proofing and UI previews.
- Proof/export manifest serialization for `proof` and `plant-ready` modes.
- Initial preflight policy checks for template confidence, guide-layer handling,
  and artwork-specific checks still required before plant submission.
- Requirements metadata for color mode, raster resolution, font handling, PDF
  standard, and whether template layers must be removed.

The current implementation intentionally does not create print-ready PDFs yet.
That should be layered on top of this crate once Presser has a plant pack
export flow. The browser remains responsible for canvas/PDF/file APIs. This
crate owns deterministic proof metadata and preflight policy for the supplied
template payload. WASM exports should stay as facade calls into this crate.

## Research Docs

- [`docs/vinyl-manufacturing-proof-research.md`](docs/vinyl-manufacturing-proof-research.md)
- [`docs/template-registry.md`](docs/template-registry.md)
- [`docs/plant-proof-export-plan.md`](docs/plant-proof-export-plan.md)
- [`../docs/presser-scale-and-label-geometry.md`](../docs/presser-scale-and-label-geometry.md)

## Checks

```bash
cargo test -p record-plant
```

## Source Refresh

```bash
make record-plant-registry-json
make record-plant-sources-download
make record-plant-sources-refresh
make record-plant-sources-analyze
make record-plant-json-sync
make record-plant-registry-validate
```

`make record-plant-refresh` runs the full sequence. The first step prepares the
generated registry JSON from `fixtures/record-plant-registry.json`. The sync
step writes the normalized collect-store seed JSON to
`workers/collect-store/seeds/record-plant-templates.json` and the hydrated
Plant app registry asset to
`apps/press/assets/plant/record-plant-templates.json`.

`make record-plant-sources-download` is cache-first. Source archives and PDFs
are stored in `.cache/record-plant/sources`, and the download manifest records
the cache path, byte count, and SHA-256 for each source. Use
`make record-plant-sources-refresh` only when you intentionally want to re-fetch
every upstream source file.

The sync step also writes D1 seed SQL to
`workers/collect-store/seeds/record-plant-templates.sql`. Use
`make record-plant-d1-seed-local` or `make record-plant-d1-seed-remote` when
you intentionally want to apply the generated registry to D1.

The plant registry contract is always an object with top-level
`manufacturers[]` and `templates[]`. Raw template arrays are invalid. The
validation target checks that the fixture, generated registry, Press app asset,
and collect-store seed all preserve those two arrays and the same IDs.
