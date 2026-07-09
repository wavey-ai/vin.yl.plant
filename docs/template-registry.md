# Record Plant Template Registry

Last updated: 2026-06-04

The curated registry fixture is
[`../fixtures/record-plant-registry.json`](../fixtures/record-plant-registry.json).
It stores measurements and source references, not manufacturer-owned template
artwork. `make record-plant-refresh` prepares generated JSON, downloads source
files for analysis, writes the Plant app asset, and writes normalized D1 seed
JSON/SQL.

## Seed Templates

| ID | Plant | Product | Seed geometry | Status |
| --- | --- | --- | --- | --- |
| `sonic-wax-12-label-2up-106mm-2022-03` | Sonic Wax Pressing | 12 in label, two-up | 126 x 232 mm PDF page, two 106 mm label references | Derived from plant template |
| `sonic-wax-7-uk-label-2up-90mm-2022-03` | Sonic Wax Pressing | 7 in UK label, two-up | 126 x 232 mm PDF page, 90 mm artwork, 84 mm final trim, 38 mm dinkout, 7.5 mm spindle guide | Plant-published |
| `sonic-wax-7-us-label-2up-96mm-2022-03` | Sonic Wax Pressing | 7 in US label, two-up | 136 x 232 mm PDF page, 96 mm artwork, 90 mm final trim, 38 mm dink, 7.5 mm spindle guide | Plant-published |
| `united-record-pressing-7-large-hole-label-2025-08` | United Record Pressing | 7 in large-hole center label | 228.6 x 149.2 mm PDF page, 98.4 mm artwork, 91.7 mm trim, 85.4 mm safe area, 38.1 mm dink, 7.5 mm spindle guide | Derived from plant template |
| `united-record-pressing-7-small-hole-label-2025-08` | United Record Pressing | 7 in small-hole center label | 228.6 x 149.2 mm PDF page, 98.4 mm artwork, 91.7 mm trim, 85.4 mm safe area, 6.8 mm center hole | Derived from plant template |
| `memphis-record-pressing-7-center-label-gd17e-2019-01` | Memphis Record Pressing | 7 in center label, small or large hole | 127 x 127 mm PDF page, 90 mm artwork, 84 mm trim, 78.4 mm safe area, 38 mm dink, 7.3 mm center hole | Derived from plant template |
| `cravedog-7-label-7mm-hole-2024-06` | Cravedog | 7 in label, 7 mm hole | 325 x 200 mm PDF page, 94.9 mm artwork, 88.9 mm trim, 84.9 mm safe area, 7 mm center hole | Derived from plant template |
| `cravedog-7-label-38mm-hole-2024-06` | Cravedog | 7 in label, 38 mm hole | 325 x 200 mm PDF page, 94.9 mm artwork, 88.9 mm trim, 84.9 mm safe area, 38 mm dink, 7.5 mm spindle guide | Derived from plant template |
| `sonic-wax-12-sleeve-3mm-spine-2022-03` | Sonic Wax Pressing | 12 in sleeve, 3 mm spine | 655.7 x 337.7 mm PDF page, named 633 x 315 mm area, 3 mm spine | Derived from plant template |
| `sonic-wax-7-sleeve-3mm-spine-2022-03` | Sonic Wax Pressing | 7 in sleeve, 3 mm spine | 395.7 x 207.7 mm PDF page, 185 mm sleeve reference, 3 mm spine | Derived from plant template |
| `skivtryck-12-label-2025-09` | Skivtryck | 12 in center label | 106 x 106 mm document, 100 mm trim, 7 mm hole, 3 mm bleed | Plant-published |
| `bladudflies-12-center-label-digital-2025-04` | Bladud Flies | 12 in center label | 106 x 106 mm document, 100 mm trim, 7.3 mm hole, 3 mm bleed | Plant-published |
| `celebrate-12-label` | Celebrate Records | 12 in center label | 106 mm artwork diameter, 100 mm final diameter, 7.5 mm hole, 3 mm bleed | Plant-published |
| `celebrate-12-picture-label` | Celebrate Records | 12 in picture label | 298 mm artwork diameter, 292 mm final diameter, 7.5 mm hole, 3 mm bleed | Plant-published |
| `vinylpressing-au-12-sleeve-3mm-spine` | VinylPressing.com.au | 12 in sleeve, 3 mm spine | 631 mm width, two 314 mm panels, 3 mm spine | Derived from plant template |
| `record-industry-12-jacket-3mm-spine-v92012` | Record Industry / Sony MediaTemplates | 12 in jacket, 3 mm spine | 667 x 352 mm document, 3 mm spine, optional 100 mm punch hole | Derived from plant template |
| `cascade-record-pressing-12-label-ab-2025-03` | Cascade Record Pressing | 12 in center label | A4 landscape source page, 105.7 mm artwork, 99.9 mm trim, 93.8 mm safe area, 7.2 mm hole | Derived from plant template |
| `cascade-record-pressing-7-small-hole-label-ab-2025-03` | Cascade Record Pressing | 7 in small-hole center label | A4 landscape source page, 97.9 mm artwork, 91.6 mm trim, 85.6 mm safe area, 7 mm hole | Derived from plant template |
| `cascade-record-pressing-7-large-hole-label-ab-2025-03` | Cascade Record Pressing | 7 in large-hole center label | A4 landscape source page, 97.9 mm artwork, 91.6 mm trim, 85.6 mm safe area, 37.7 mm dink, 7.5 mm spindle guide | Derived from plant template |
| `hellbender-10-12-center-label-4in-2024-10` | Hellbender Vinyl | 10/12 in center label | 107.95 mm bleed, 101.6 mm trim, 95.25 mm safety, 7.9375 mm hole | Plant-published |
| `hellbender-7-small-hole-label-2024-07` | Hellbender Vinyl | 7 in small-hole center label | 98.25 mm bleed, 91.9 mm trim, 85.55 mm safety, 6.97 mm hole | Derived from plant template |
| `gotta-groove-12-label-2023-illustrator` | Gotta Groove Records | 12 in center label | 107.95 mm bleed, 101.6 mm trim, 95.25 mm safety | Derived from plant template |
| `gotta-groove-7-small-hole-label-2014-12` | Gotta Groove Records | 7 in small-hole center label | 98.42 mm bleed, 92.07 mm trim, 85.72 mm safety | Derived from plant template |
| `gotta-groove-7-large-hole-label-2014-12` | Gotta Groove Records | 7 in large-hole center label | 98.42 mm bleed, 92.07 mm trim, 85.72 mm safety, 38.1 mm dink | Derived from plant template |
| `standard-vinyl-12-center-label-4in-2022-06` | Standard Vinyl | 12 in center label | 107.78 mm bleed, 101.6 mm trim, 95.07 mm safety, 7.9375 mm hole | Derived from plant template |
| `standard-vinyl-10-center-label-4in-2018-07` | Standard Vinyl | 10 in center label | 107.78 mm bleed, 101.6 mm trim, 95.08 mm safety, 7.9375 mm hole | Derived from plant template |
| `standard-vinyl-7-small-hole-label-2018-10` | Standard Vinyl | 7 in small-hole center label | 98.35 mm bleed, 92 mm trim, 85.65 mm safety, 7.9375 mm hole | Derived from plant template |
| `standard-vinyl-7-large-hole-label-2018-07` | Standard Vinyl | 7 in large-hole center label | 98.35 mm bleed, 92 mm trim, 85.65 mm safety, 38.99 mm dink | Derived from plant template |
| `precision-record-pressing-7-small-hole-label-7-l001-a1` | Precision Record Pressing | 7 in small-hole center label | 90.28 mm bleed, 83.94 mm trim, 77.59 mm safety, 7.23 mm hole | Derived from plant template |
| `precision-record-pressing-7-large-hole-label-7-l002-a1` | Precision Record Pressing | 7 in large-hole center label | 90.28 mm bleed, 83.94 mm trim, 77.59 mm safety, 37.97 mm dink | Derived from plant template |
| `precision-record-pressing-10-center-label-10-l001-a1` | Precision Record Pressing | 10 in center label | 106.23 mm bleed, 99.89 mm trim, 93.54 mm safety, 7.23 mm hole | Derived from plant template |
| `precision-record-pressing-12-center-label-ab-12-l001-a3` | Precision Record Pressing | 12 in center label | 106.21 mm bleed, 99.87 mm trim, 93.53 mm safety, 7.23 mm hole | Derived from plant template |
| `the-jungle-record-press-12-label-ab-106mm-2023-10` | The Jungle Record Press | 12 in center label, A/B sheet | 126 x 232 mm PDF page, two 106 mm bleed/artwork labels, 101 mm trim, 94.6 mm no-type area, 6.8 mm hole | Derived from plant template |
| `press-on-vinyl-7-centre-label-2024-03` | Press On Vinyl | 7 in centre label | 111.17 mm PDF page, 90 mm artwork, 84 mm trim, 38 mm dink, 7.5 mm spindle guide | Needs plant confirmation |
| `cram-duplication-10-12-label-100mm` | Cram Duplication | 10/12 in center label | 106 mm bleed, 100 mm trim, 7.24 mm hole, 3 mm bleed | Plant-published |
| `mighty-media-discs-7-label-2up-90mm-2014-07` | Mighty Media Discs | 7 in label, two-up | 146 x 252 mm PDF page, two 90 mm artwork labels, 84 mm trim, 38 mm dink, 3 mm bleed | Plant-published |
| `mighty-media-discs-12-label-2up-106mm-2014-07` | Mighty Media Discs | 12 in label, two-up | 147.17 x 253.17 mm PDF page, two 106 mm artwork labels, 101 mm trim, 95 mm no-type area, 7 mm hole, 2.5 mm bleed | Plant-published |
| `sixtysix-productions-12-label-100mm-2016-05` | SixtySix Productions | 12 in label | 106 mm artwork, 100 mm trim, 7.5 mm hole, 3 mm bleed | Plant-published |
| `duophonic-10-12-label-2024-07` | Duophonic | 10 and 12 in label | 106 mm artwork, 100 mm trim, 7 mm hole, 3 mm bleed | Plant-published |
| `duophonic-7-small-hole-label-2024-07` | Duophonic | 7 in small-hole label | 98 mm artwork, 92 mm trim, 7 mm hole, 3 mm bleed | Plant-published |
| `duophonic-7-big-hole-label-2024-07` | Duophonic | 7 in big-hole label | 98 mm artwork, 92 mm trim, 38 mm dink, 7 mm spindle guide, 3 mm bleed | Plant-published |
| `polvinyl-12-label-2024-07` | Polvinyl | 12 in label | 106 mm artwork, 100 mm trim, 7.3 mm hole, 3 mm bleed | Plant-published |
| `polvinyl-7-small-center-hole-label-2024-07` | Polvinyl | 7 in small-hole label | 98 mm artwork, 92 mm trim, 7.3 mm hole, 3 mm bleed | Plant-published |
| `polvinyl-7-large-center-hole-label-2024-07` | Polvinyl | 7 in large-hole label | 98 mm artwork, 92 mm trim, 38 mm dink, 7.5 mm spindle guide, 3 mm bleed | Plant-published |
| `phono-press-12-label-101mm` | Phono-Press International | 12 in center label | 111 mm bleed, 101 mm trim, 7 mm hole | Plant-published |
| `phono-press-7-label-92mm-small-or-large-hole` | Phono-Press International | 7 in center label, small or large hole | 92 mm trim, 7 mm hole, 38 mm big-hole reference | Plant-published |
| `optimal-media-12-label-100mm` | optimal media | 12 in center label | 106 mm artwork, 100 mm trim, 7.5 mm hole, 3 mm bleed | Plant-published |
| `pallas-group-12-label-2025-10` | Pallas Group | 12 in label | 106 mm artwork, 100 mm trim, 90 mm safety, 7 mm hole, 3 mm bleed | Plant-published |
| `pallas-group-10-label-2025-10` | Pallas Group | 10 in label | 106 mm artwork, 100 mm trim, 90 mm safety, 7 mm hole, 3 mm bleed | Plant-published |
| `pallas-group-7-small-hole-label-2025-10` | Pallas Group | 7 in small-hole label | 98 mm artwork, 92 mm trim, 82 mm safety, 7 mm hole, 3 mm bleed | Plant-published |
| `pallas-group-7-big-hole-label-2025-10` | Pallas Group | 7 in big-hole label | 98 mm artwork, 92 mm trim, 82 mm safety, 38 mm dink, 3 mm bleed | Plant-published |
| `gz-media-12-basic-label-100mm-2010-01` | GZ Media | 12 in label | 100 mm trim, 7 mm hole | Plant-published |
| `gz-media-7-basic-label-84mm-2010-01` | GZ Media | 7 in label | 84 mm trim, 7 mm hole | Plant-published |
| `pitch-beats-12-label-100mm` | Pitch Beats by Sweden | 12 in center label | 100 mm trim | Plant-published |
| `pitch-beats-7-label-90mm` | Pitch Beats by Sweden | 7 in center label | 90 mm trim | Plant-published |
| `grama-7-single-label-92mm-2025-06` | Grama | 7 in single label | 92 mm trim, 7.3 mm small hole, 38 mm big-hole option | Plant-published |
| `celebrate-7-small-hole-label` | Celebrate Records | 7 in small-hole label | 98 mm artwork, 92 mm trim, 7.5 mm hole, 3 mm bleed | Plant-published |
| `celebrate-7-large-hole-label` | Celebrate Records | 7 in large-hole label | 98 mm artwork, 92 mm trim, 38 mm dink, 7.5 mm spindle guide, 3 mm bleed | Plant-published |
| `celebrate-7-picture-label` | Celebrate Records | 7 in picture label | 175 mm artwork, 169 mm trim, 7.5 mm hole, 3 mm bleed | Plant-published |
| `green-lakes-pressing-12-label-2024-11` | Green Lakes Pressing | 12 in label | 106 mm bleed box, 100 mm trim box | Plant-published |
| `green-lakes-pressing-7-label-2023-07` | Green Lakes Pressing | 7 in label | 98 mm artwork, 92 mm trim | Plant-published |
| `deepgrooves-12-label-100mm-2019-07` | Deepgrooves | 12 in label | 110 mm open format, 100 mm trim, 5 mm bleed | Plant-published |
| `deepgrooves-7-label-92mm-2019-07` | Deepgrooves | 7 in label | 102 mm open format, 92 mm trim, 7.3 mm hole, 38 mm dink, 5 mm bleed | Plant-published |
| `rand-muzik-12-label-2019` | R.A.N.D. Muzik | 12 in label | 106 mm artwork, 100 mm trim, 7.3 mm hole, 3 mm bleed | Plant-published |
| `rand-muzik-7-small-hole-label-2019` | R.A.N.D. Muzik | 7 in small-hole label | 98 mm artwork, 92 mm trim, 7.3 mm hole, 3 mm bleed | Plant-published |
| `rand-muzik-7-large-hole-label-2019` | R.A.N.D. Muzik | 7 in large-hole label | 98 mm artwork, 92 mm trim, 38 mm dink, 7.3 mm spindle guide, 3 mm bleed | Plant-published |
| `pirates-press-7-basic-label-84mm` | Pirates Press | 7 in basic label | 84 mm final label diameter | Plant-published |
| `pirates-press-10-12-basic-label-100mm` | Pirates Press | 10/12 in basic label | 100 mm final label diameter | Plant-published |
| `pirates-press-12-picture-disc-label-292mm` | Pirates Press | 12 in picture label | 292 mm final picture-disc label diameter | Plant-published |
| `new-orleans-record-press-12-center-label-4in-low-run` | New Orleans Record Press | 12 in center label | 101.6 mm final label diameter | Plant-published |
| `new-orleans-record-press-7-small-hole-center-label-3p625in` | New Orleans Record Press | 7 in small-hole center label | 92.075 mm final label diameter | Plant-published |
| `new-orleans-record-press-7-large-hole-center-label-3p625in` | New Orleans Record Press | 7 in large-hole center label | 92.075 mm final label diameter | Plant-published |
| `digital-force-12-record-label-4in-2014-10` | Digital Force | 12 in record label | 107.95 mm bleed, 101.6 mm trim, 95.25 mm safety | Plant-published |
| `united-record-pressing-12-center-label-2010-02` | United Record Pressing | 12 in center label | 107.95 mm bleed, 100 mm trim | Plant-published |
| `audio-geography-7-label-3p25in` | Audio Geography Studios | 7 in size label | 82.55 mm final label diameter | Plant-published |
| `audio-geography-12-label-4in` | Audio Geography Studios | 12 in size label | 101.6 mm final label diameter | Plant-published |
| `vinylpressing-au-10-12-label-2020-02` | VinylPressing.com.au | 10/12 in label | 106 mm bleed, 100 mm trim, 94 mm safety, 7.2 mm hole | Plant-published |
| `pressing-business-12-center-label-2355-2020-11` | Pressing Business | 12 in center label | 297 x 208.33 mm PDF page, 106 mm bleed, 100 mm trim, 94 mm safety, 13 mm center clearance, 7 mm hole | Derived from plant template |
| `pressing-business-7-small-hole-center-label-2408-2020-11` | Pressing Business | 7 in small-hole center label | 297 x 208.33 mm PDF page, 90 mm bleed, 84 mm trim, 7 mm hole | Derived from plant template |
| `xvinylx-12-center-label-gd30e-2020-03` | XVINYLX | 12 in center label | A4 landscape page, 105.69 mm bleed, 99.88 mm trim, 93.78 mm safety, 7.23 mm hole | Derived from plant template |
| `xvinylx-7-small-hole-center-label-gd17e-2020-03` | XVINYLX | 7 in small-hole center label | A4 landscape page, 89.90 mm bleed, 83.90 mm trim, 77.62 mm safety, 7.23 mm hole | Derived from plant template |
| `xvinylx-7-big-hole-center-label-gd17e-2017-09` | XVINYLX | 7 in big-hole center label | A4 landscape page, 89.62 mm bleed, 83.62 mm trim, 77.62 mm safety, 37.67 mm dink | Derived from plant template |
| `elasticstage-12-center-label-v1-6` | elasticStage | 12 in labels, two-up A/C and B/D | 106 x 212 mm document, two 106 mm bleed labels, 100 mm trim, 8 mm center-hole guide | Derived from plant template |
| `elasticstage-quick-easy-vinyl-jpg-v1-1` | elasticStage | Quick & Easy vinyl raster upload template | JPG/PNG raster slots, front slot approximately 318 x 324 mm, label slot 106 x 106 mm | Derived from plant template |

## Registry Fields

Each template records:

- stable `id`;
- manufacturer and product name;
- template kind, such as center label, picture label, or outer sleeve;
- physical document rectangle in millimeters;
- guide geometry for bleed, trim, safety, folds, spines, and holes;
- measurement confidence;
- submission requirements;
- source title, URL, and retrieval date;
- notes that explain exactly which measurements came from the source.

## Confidence Rules

`PlantPublished` means the public source explicitly states the geometry we use.

`DerivedFromPlantTemplate` means the public source states enough visible geometry
to seed a guide, but we still need a manual template validation pass before
automated plant-ready export.

`NeedsPlantConfirmation` is reserved for measurements inferred from surrounding
practice or secondary references. These should not be used for plant-ready output
without direct plant confirmation.

## Next Registry Targets

- Precision Record Pressing jackets, gatefolds, inner sleeves, inserts,
  picture discs, on-disc effects, obi strips, posters, and stickers after
  vector geometry extraction from the published PDFs.
- URP current 10/12 in center-label template archive after vector geometry
  extraction from the published 2025 ZIP.
- Hellbender 7 in and 12 in jacket rect/fold geometry after careful PDF
  decomposition.
- Standard Vinyl picture-disc labels, etched-record labels, jackets, gatefolds,
  inner sleeves, inserts, posters, obi strips, stickers, and record boxes after
  vector geometry extraction from the published PDFs.
- Hand Drawn Pressing 7 in and 12 in label templates after vector geometry
  extraction from the published PDFs and ZIP files.
- UK market coverage: DMS, Mighty Media Discs, and SixtySix label specs are
  encoded where public PDF text exposes dimensions. Seabass Vinyl, The Vinyl
  Factory/Noon Media Solutions, Cutsy, Cyclone Music, Breed Media, Key
  Production, Packaged Sounds Group, Vinyl Presents, Hoxton Vinyl, Diamond Black
  Vinyl, Sound Performance, AGR Manufacturing, VDC Group, Mediaplant, Media Hut,
  Disc Wizards, Eurodisc Manufacturing, Well Tempered, Trapeze, Duplion, Vinyl
  Press UK, CD Unity, and Takt Direct UK have supplier metadata only until source
  geometry is extracted or confirmed. Curved Pressing, Once Upon A Time Music,
  legacy lathe-cut services, and other UK support businesses should be resolved
  source-by-source before we claim full UK coverage.
- New Orleans Record Press jackets, inner sleeves, inserts, and 12 in bulk
  center-label variants after PDF geometry extraction.
- The Jungle Record Press 7 in labels and sleeve templates only after their own
  published files are inspected; the encoded 106/101 mm label is LP/12 in
  geometry, not a 45 label.
- VinylPressing.com.au 7 in label after resolving the conflicting text
  extraction around 92 mm, 87 mm, and 85 mm diameters.
- Green Lakes 10 in labels and outer sleeves with 3 mm, 4 mm, and 7 mm spines.
- Polvinyl inner sleeves and spined sleeves.
- Gotta Groove jackets, 7 in sleeves, inserts, sticker labels, and additional
  package templates after vector geometry extraction from the published files.
- Remaining Sonic Wax templates: spineless sleeves, inserts, obi
  strips, 12 in discobags, 5 mm and 7 mm spines, gatefold, inserts, and obi
  strips.

These should be added only as measurements and metadata unless we get explicit
permission to redistribute plant template files.

## Sources Without Registry Geometry

Blood Records / Blood Recs is currently recorded as product research rather than
template geometry. Its public site describes limited-edition, exclusively pressed
vinyl records and zoetrope picture-disc viewing guidance, but I did not find a
public client-facing template or downloadable dieline to encode.
