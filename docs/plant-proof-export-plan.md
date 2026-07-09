# Plant Proof Export Plan

Last updated: 2026-06-04

## Product Shape

Presser should export a manufacturing pack instead of a single ambiguous
"print file." The pack should contain:

- `proof.pdf`: human proof with visible trim, bleed, safety, fold, spine, and
  hole guides.
- `plant-ready.pdf`: final artwork with template and guide layers removed or
  marked non-printing according to the plant's instruction.
- `record-plant-spec.json`: exact template ID, dimensions, source URL, export
  settings, color expectations, and preflight results.
- `preflight.md`: human-readable pass/warn/fail checklist.
- `README-for-plant.md`: short instructions for the plant or project manager.

## Rust Boundary

`record-plant` should own:

- template payload schema;
- millimeter geometry parsing/rendering;
- unit conversion;
- guide model for trim, bleed, safety, holes, folds, and spines;
- plant requirements and preflight policy;
- deterministic proof/spec serialization.

The curated manufacturer/template records should stay in fixture data and D1,
not in Rust or WASM. The crate receives a selected template payload and returns
proof/spec data for that payload.

Presser JavaScript should own:

- file input and drag/drop;
- browser preview interaction;
- network uploads and downloads;
- APIs that only exist in the browser;
- final integration with browser PDF, canvas, or file-system capabilities.

Future WASM bundles can expose the same registry and preflight logic to each app
without reimplementing plant rules in JavaScript.

## Export Modes

`Proof` mode is for human review. It may show visible guides, labels, notes, and
source template IDs.

`PlantReady` mode is for submission. It must remove visible guide layers unless
the target plant explicitly asks for non-printing dieline layers.

## Current API

Implemented in `record-plant`:

- payload-driven proof bundles via `record_plant_template_proof_bundle_json`;
- compact user-dimension proof bundles via
  `custom_record_plant_template_proof_bundle_json`;
- preflight policy checks for template selection, confidence, guide-layer
  policy, and required artwork-specific preflight.

Exposed through `record-wasm` as façade functions for Presser, without bundling
the curated plant registry into the WASM build:

- `recordPlantTemplateProofBundleJson`;
- `customRecordPlantTemplateProofBundleJson`.

## Preflight Checks To Add

- Template ID and version selected.
- Artboard dimensions exactly match the template.
- Bleed area is filled.
- Text and critical elements stay inside the safe area.
- Center-label artwork fills the center hole where required.
- Raster images meet minimum ppi at placed size.
- Color mode is CMYK, grayscale, or approved Pantone/PMS.
- Fonts are embedded or outlined.
- PDF standard matches plant preference, such as PDF/X-1a.
- Guide/template layers are absent from the plant-ready artifact.
- Total ink coverage warnings where the plant publishes a limit.

## Implementation Sequence

1. Keep growing `fixtures/record-plant-registry.json` from public plant
   measurements.
2. Sync fixture data to generated app JSON and D1 seed JSON/SQL.
3. Generate visible SVG/PDF proof overlays from registry geometry.
4. Add plant-ready PDF export with guide removal guarantees.
5. Add preflight reports and golden tests per template.
