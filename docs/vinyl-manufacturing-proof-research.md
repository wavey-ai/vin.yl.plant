# Vinyl Manufacturing Proof Research

Last updated: 2026-06-04

## Main Finding

There is no single universal vinyl manufacturing template that every plant
accepts. The practical standard is plant-specific dielines plus a common
prepress discipline:

- use the plant's current template for the exact product;
- build at 1:1 physical scale;
- keep bleed outside the trim;
- keep text and critical artwork inside a safe area;
- remove or disable template/dieline layers before final submission;
- submit CMYK, grayscale, or approved Pantone artwork;
- embed fonts or convert type to outlines;
- provide print-ready PDF where the plant asks for PDF output.

Bitneedle should therefore export "manufacturing packs" rather than claim one
global vinyl template. A pack can include a plant-ready artifact, a human proof
with visible guides, a machine-readable spec, and a README for the plant.

## Cross-Plant Rules Seen Repeatedly

| Area | Common requirement |
| --- | --- |
| Template use | Use the manufacturer's supplied template and do not resize it. |
| Final guides | Do not embed, flatten, or include template lines in final plant-ready art. |
| Bleed | 1/8 in (3.175 mm) or 3 mm is common; Memphis recommends 1/4 in when possible. |
| Safety | 1/8 in is common; Green Lakes uses 5 mm for critical elements. |
| Resolution | 300 ppi/dpi is common; Memphis asks for 800 ppi for bitmap images. |
| Color | CMYK, grayscale, or official Pantone/PMS. RGB should be converted. |
| Fonts | Embed fonts, package fonts, or convert type to outlines/curves. |
| Labels | Center-label art should usually fill the center hole rather than knock it out. |
| PDF | PDF/X-1a is requested by some plants; print-ready PDF is widely preferred. |

## Source Notes

### Precision Record Pressing

Precision publishes a broad vinyl template library across jackets, gatefolds,
center labels, inner sleeves, inserts, posters, picture discs, obi strips, and
more. Their page says accepted artwork formats include PDF, AI, PSD, and EPS;
JPG and PNG are not accepted. It also states all artwork should be CMYK and
warns that artwork can shift up to 1/8 in during printing and finishing.

Downloaded and inspected on 2026-06-04:

- `prp-7-small-hole-label-7-l001-a1.pdf` visibly identifies a 7 in small-hole
  label template. Its outlined instructions say the outer dotted line is bleed,
  middle solid line is cut, inner dotted line is safety margin, and the
  Template layer should stay locked and separate from art. Vector extraction
  measured approximately 90.28 mm bleed, 83.94 mm trim, 77.59 mm safety, and
  7.23 mm center hole.
- `prp-7-large-hole-label-7-l002-a1.pdf` uses the same outer geometry and
  measured approximately 37.97 mm for the large center cut.
- `prp-10-label-10-l001-a1.pdf` measured approximately 106.23 mm bleed,
  99.89 mm trim, 93.54 mm safety, and 7.23 mm center hole.
- `prp-12-label-ab-12-l001-a3.pdf` measured approximately 106.21 mm bleed,
  99.87 mm trim, 93.53 mm safety, and 7.23 mm center hole. The public page
  also lists a C/D label PDF, but it is not encoded separately because the A/B
  sheet captures the shared label geometry.
- The public contact page exposes phone numbers and contact forms, but no
  public email address was found in the inspected page text or HTML.

Sources:

- https://www.precisionpressing.com/templates
- https://www.precisionpressing.com/contact
- https://www.datocms-assets.com/146666/1738546476-prp-7-small-hole-label-7-l001-a1.pdf
- https://www.datocms-assets.com/146666/1738546476-prp-7-large-hole-label-7-l002-a1.pdf
- https://www.datocms-assets.com/146666/1739465679-prp-10-label-10-l001-a1.pdf
- https://www.datocms-assets.com/146666/1738546579-prp-12-label-ab-12-l001-a3.pdf

### The Jungle Record Press

The Jungle Record Press publishes a public templates page with artwork specs
and template downloads. The inspected label PDF is explicitly a 12 in A/B label
sheet; its 126 x 232 mm page, two 106 mm artwork edges, and 101 mm final trims
are encoded as LP/12 in geometry only, not as a 7 in/45 label.

Downloaded and inspected on 2026-06-04:

- `12inch_Label_Template_A_B_106mm.pdf` visibly states that artwork must fit to
  the 106 mm edge and is trimmed to a 101 mm finished size, with 2.5 mm bleed
  all around.
- The PDF page box is 357.165 x 657.638 pt, approximately 126 x 232 mm, matching
  a two-up A/B sheet layout.
- The template lists the no-type critical print area visually; vector
  extraction measured approximately 94.6 mm for that guide and 6.8 mm for the
  center hole.
- The visible spec text states colour as `CYMK`, profile FOGRA39, minimum
  resolution 300Dpi, and accepted formats PDF, Tiff, Jpeg, and PSD. The
  registry normalizes `CYMK` to CMYK in `colorModes` and keeps FOGRA39 in notes
  because there is not yet a structured color-profile field.

Sources:

- https://www.thejunglerecordpress.com/pages/artwork-template
- https://www.thejunglerecordpress.com/pages/contact
- https://cdn.shopify.com/s/files/1/0699/2360/2699/files/12inch_Label_Template_A_B_106mm.pdf?v=1723155688

Contact: info@thejunglerecordpress.com

### United Record Pressing

URP's art guidelines say documents should be built from URP-supplied templates.
The template should be on a separate layer and discarded or turned off before
submission. URP specifies 1/8 in bleed on outside or cut edges, 300 ppi imagery,
CMYK or grayscale, and print-ready PDFs with embedded or outlined fonts.

Source: https://customers.urpressing.com/pdf/URP_ArtGuidelines.pdf

Downloaded and inspected on 2026-06-04:

- `URP_12_Template.zip` from the official customer template page contains
  `12inchtemplate.pdf`, EPS, InDesign, and QuarkXPress files. Text extraction
  from the PDF states a 12 in vinyl label template with 4.25 in bleed and
  3.937 in trim. These convert to 107.95 mm bleed and approximately
  100.0 mm trim. No source-specific center-hole diameter was encoded.
- The current public URP template page also publishes
  `10in12inCenterLabelTemplate.zip`, but the inspected PDF text stream did not
  expose physical guide dimensions. The current archive remains a vector
  geometry extraction target.

Sources:

- https://www.urpressing.com/client-resources/templates/
- https://customers.urpressing.com/templates/
- https://customers.urpressing.com/templates/URP_12_Template.zip
- https://www.urpressing.com/wp-content/uploads/2024/06/Page-4-2024-Art-Guidelines.pdf

Contact: united@urpressing.com

### Hellbender Vinyl

Hellbender says all files must coincide with specific templates and that
dielines should never be embedded or flattened with the artwork. Their specs
include 1/8 in bleed, 1/8 in type safety, minimum 300 dpi raster resolution,
CMYK conversion, embedded or outlined fonts, and a recommendation to keep text,
line art, and flat solids in vector formats.

Downloaded and inspected on 2026-06-04:

- `Vinyl_10inchlabel.pdf` states the 10 in label is the same as the 12 in
  label, with a 0.3125 in center hole and 4 in outer diameter. Combined with
  Hellbender's published 1/8 in bleed and 1/8 in type-safety requirements, this
  encodes to 107.95 mm bleed, 101.6 mm trim, 95.25 mm safety, and 7.9375 mm
  center hole.
- `7InchSMALLTemplate_HELLBENDER.pdf` maps the outer dotted line to bleed, the
  middle solid line to cut, and the inner dotted line to safety. Vector
  extraction measured approximately 98.25 mm bleed, 91.9 mm trim, 85.55 mm
  safety, and 6.97 mm center hole.
- The 12 in and 7 in jacket PDFs expose overall document sizes
  (24.7813 x 13.887 in and 14.5 x 9.125 in respectively), but still need a
  careful fold/flap decomposition pass before registry encoding.

Sources:

- https://hellbendervinyl.com/pages/templates
- https://cdn.shopify.com/s/files/1/0593/3137/9286/files/Vinyl_10inchlabel.pdf?v=1728995647
- https://cdn.shopify.com/s/files/1/0593/3137/9286/files/7InchSMALLTemplate_HELLBENDER.pdf?v=1720725852

Contact: hello@hellbendervinyl.com

### Gotta Groove Records

Gotta Groove publishes label templates for 12 in labels, 7 in small-hole
labels, and 7 in large-hole labels. Their art guide asks clients to use only
Gotta Groove templates, keep template lines and elements intact, and avoid
flattening template elements into artwork. It asks for 300 ppi art, CMYK not
RGB, B&W art as grayscale or K-only, and the
GRACoL2006_Coated1v2_PerfX profile. It also asks for all type to be outlined
before submitting, even in PDF files.

Downloaded and inspected on 2026-06-04:

- `12-INCH-LABELS.eps` contains vector guide circles measuring 107.95 mm
  bleed/artwork, 101.60 mm trim, and 95.25 mm safety. That trim size is normal
  LP/12 in label geometry; it is not a 45-only diameter.
- `7-Inch-Label-small-hole.eps` contains vector guide circles measuring
  98.42 mm bleed/artwork, 92.07 mm trim, and 85.72 mm safety. It includes
  center marks, but no source-specific small spindle-hole diameter is encoded.
- `7-Inch-Label-LARGE-HOLE.eps` contains vector guide circles measuring
  98.42 mm bleed/artwork, 92.07 mm trim, 85.72 mm safety, and a 38.10 mm
  large-hole/dink cut. The EPS also contains a 44.45 mm center safety/marker
  ring; only the 38.10 mm dink is encoded as cut geometry.

Sources:

- https://www.gottagrooverecords.com/art-templates/
- https://www.dropbox.com/s/awbd6qercauvfjx/GGR%20ART%20GUIDE.pdf?dl=1
- https://www.dropbox.com/scl/fi/0dpalz5wy5imbi0wcdv5t/12-INCH-LABELS.eps?rlkey=y7dy9lyv55dfb9r2reuksuffr&dl=1
- https://www.dropbox.com/scl/fi/yl6bct88fecnb3sjyj6by/7-Inch-Label-small-hole.eps?rlkey=glxts3xib04c7lhkduk01le6q&dl=1
- https://www.dropbox.com/scl/fi/d0p13z7fw34jyiqimi93p/7-Inch-Label-LARGE-HOLE.eps?rlkey=7nk5i1u0e2u9p35fhob9rlvoj&dl=1

Contacts: sales@gottagrooverecords.com, customerservice@gottagrooverecords.com

### Standard Vinyl

Standard Vinyl publishes a broad public template library. Their template page
recommends sending layered working files with all fonts and links, plus a
high-resolution PDF reference. The same page includes direct PDF downloads for
12 in labels, 10 in labels, 7 in small-hole labels, 7 in large-hole labels,
picture discs, etched records, jackets, inner sleeves, inserts, posters,
booklets, obi strips, stickers, and record boxes.

Downloaded and inspected on 2026-06-04:

- `12inch_Labels.pdf` rendered text states center hole 0.3125 in and outer
  diameter 4 in. The vector legend maps red dashed to bleed, blue solid to
  crop, green dotted to type safety, and yellow dashed to slight indent.
  Normalized vector rings encode approximately 107.78 mm bleed, 101.6 mm trim,
  95.07 mm safety, and 7.9375 mm center hole.
- `10inch_Labels.pdf` uses the same 4 in outer diameter and 0.3125 in center
  hole as the 12 in label, with nearly identical vector bleed and safety rings.
- `7inch_Small-Hole_Labels.pdf` states center hole 0.3125 in and outer
  diameter 3.622 in. Normalized vector rings encode approximately 98.35 mm
  bleed, 92 mm trim, 85.65 mm safety, and 7.9375 mm center hole.
- `7inch_Large-Hole_Labels.pdf` states center hole 1.535 in and outer diameter
  3.622 in. Normalized vector rings encode approximately 98.35 mm bleed,
  92 mm trim, 85.65 mm safety, and 38.99 mm dink.

Sources:

- https://standardvinyl.com/place-an-order/label-packaging-templates/
- https://standardvinyl.com/wp-content/uploads/2022/06/12inch_Labels.pdf
- https://standardvinyl.com/wp-content/uploads/2018/07/10inch_Labels.pdf
- https://standardvinyl.com/wp-content/uploads/2018/10/7inch_Small-Hole_Labels.pdf
- https://standardvinyl.com/wp-content/uploads/2018/07/7inch_Large-Hole_Labels.pdf

Contact: hello@standardvinyl.com

### Memphis Record Pressing

MRP's 2025 advanced art prep guide prefers PDF layout submission and says all
files should be submitted without template lines. It asks for 300 ppi for
standard images, 800 ppi for bitmap images, CMYK or official Pantone/PMS color,
minimum 1/8 in bleed while strongly recommending 1/4 in, and a 1/8 in safety
area. It also says vinyl center-label art should be submitted as a solid image
with no center-hole knockout.

Source: https://memphisrecordpressing.com/wp-content/uploads/2025/11/MRP-Advanced_Art_File_Prep_Guide_2025.pdf

### Skivtryck

The Skivtryck 12 in label template lists a 100 mm trim size, 7 mm central hole,
and 3 mm bleed. It warns not to flatten or merge template lines with artwork.

Source: https://skivtryck.se/wp-content/uploads/2025/09/label_12_inch.pdf

### Bladud Flies

The Bladud Flies 12 in center-label digital-print template lists 100 x 100 mm
trim, 106 x 106 mm with 3 mm bleed, and a 7.3 x 7.3 mm center hole. It says to
fill the center hole with artwork, delete the template before saving, and save
as PDF/X-1a:2001.

Source: https://bladudflies.com/wp-content/uploads/2025/04/12inch_centre_label_digital-print_template.pdf

### Celebrate Records

Celebrate lists 12 in label requirements of PDF/EPS/TIF/PSD, CMYK/grayscale/
Pantone, minimum 250 dpi, maximum 320 percent ink coverage, and 3 mm bleed. Its
12 in label spec says the final label is 100 mm but artwork should be created
at 106 mm, with a final 7.5 mm center hole. The 12 in picture label is 292 mm
final and 298 mm with bleed. The page also lists several jacket and sleeve
document sizes.

Source: https://www.celebrate.de/en/cover-12inch-vinyl/

The 7 in artwork page lists 7 in label requirements of PDF/EPS/TIF/PSD,
CMYK/grayscale/Pantone, minimum 250 dpi, maximum 320 percent ink coverage, and
3 mm bleed. Its label spec says artwork should be created at 98 mm even though
the final format is 92 mm, and it supports either a 7.5 mm or 38 mm center
hole. The 7 in picture-label spec says artwork should be created at 175 mm even
though the final format is 169 mm, with a 7.5 mm center hole.

Source: https://www.celebrate.de/en/schallplatten-pressen-lassen-7-inch-cover-innenhuellen-drucksachen-templates/

### Polvinyl

Polvinyl publishes template groups for 7 in and 12 in records, including labels,
inner sleeves, disco bags, spined sleeves, gatefolds, slipcases, inlays, folders,
stamps, stickers, and download-code pieces. The visible template index calls out
7 in labels at 92 mm and 12 in labels at 100 mm.

Source: https://polvinyl.com/templates/

### Phono-Press International

Phono-Press publishes public label PDFs for 12 in and 7 in labels. Its official
site says accepted artwork filetypes are PDF, TIFF, EPS, AI, and PNG; color mode
should be CMYK or grayscale; artwork should be multi-layer; and raster
resolution should be at least 300 DPI.

Downloaded and inspected on 2026-06-04:

- `etich12.pdf` shows 101 mm trim, 7 mm center hole, and an orange dashed bleed
  guide described as at least 5 mm per side where marked. The registry encodes
  111 mm bleed/artwork, 101 mm trim, and 7 mm center hole.
- `etich7.pdf` shows a 92 mm outer label, a 7 mm center hole, and a 38 mm
  large-hole reference. The registry encodes this as one small-or-large-hole
  entry with both the 7 mm hole guide and 38 mm dink/reference guide.

Sources:

- https://www.phonopress.it/
- https://www.phonopress.it/download/etich12.pdf
- https://www.phonopress.it/download/etich7.pdf

Contact: info@phonopress.it

### optimal media

optimal media publishes downloadable vinyl specification PDFs. The 12 in label
PDF states 100 x 100 mm label size, 7.5 mm centre hole, and 3 mm bleed area.
The registry encodes 106 mm artwork/bleed, 100 mm trim, and 7.5 mm center hole.
No source-specific safety diameter or accepted final artwork filetype list is
encoded for this entry.

Sources:

- https://www.optimal-media.com/en/specifications/
- https://www.optimal-media.com/wp-content/uploads/12inch_Label_en.pdf
- https://www.optimal-media.com/en/contact-service-center/

Contact: info@optimal-media.com

### Pallas Group

Pallas publishes a current vinyl specifications page with product-specific PDF
templates. The page states that templates must always be removed from the final
print PDF, and the footer lists Schallplattenfabrik Pallas GmbH with
`info@pallas-group.de`.

The general print specification asks for print-ready PDF/X-3, CMYK colour data,
grayscale black-and-white data, PSO Coated v3, maximum 300% total ink coverage
or 280% for picture-disc labels, 300 dpi colour/grayscale images, 1200 dpi
bitmap line art, and embedded or outlined fonts. It says data should include
bleed, either 3 mm or 5 mm all around depending on product, and important
images/text/motifs should sit at least 3 mm from edges, preferably 5 mm.

Downloaded and inspected on 2026-06-04:

- `01.PALLAS_12inch_label.pdf` states 100 mm outer diameter without bleed,
  7 mm inner diameter without bleed, 3 mm bleed all around, and 5 mm distance
  to the cutting edge with 3 mm minimum. The registry encodes 106 mm
  bleed/artwork, 100 mm trim, 90 mm safety, and 7 mm center-hole guide.
- `01.PALLAS_10inch_label.pdf` states the same 100 mm outer diameter, 7 mm
  inner diameter, 3 mm bleed all around, and 5 mm distance to the cutting edge.
  The registry encodes 106 mm bleed/artwork, 100 mm trim, 90 mm safety, and
  7 mm center-hole guide.
- `01.PALLAS_7inch_label.pdf` states small-centre-hole geometry as 92 mm outer
  diameter and 7 mm inner diameter, big-centre-hole geometry as 92 mm outer
  diameter and 38 mm inner diameter, plus 3 mm bleed all around and 5 mm
  distance to the cutting edge. The registry encodes 98 mm bleed/artwork,
  92 mm trim, 82 mm safety, and either a 7 mm hole or 38 mm dink guide.

The label PDFs say not to create a centre hole in submitted artwork. Bitneedle
therefore treats the holes as proof/guide geometry; plant-ready output should
still remove visible guide/template layers.

Sources:

- https://www.pallasgroup.de/vinyl/vinyl-spezifikationen/
- https://www.pallasgroup.de/wp-content/uploads/01.PALLAS_schriftlicheSpecs_LP_12_dt.pdf
- https://www.pallasgroup.de/wp-content/uploads/01.PALLAS_12inch_label.pdf
- https://www.pallasgroup.de/wp-content/uploads/01.PALLAS_10inch_label.pdf
- https://www.pallasgroup.de/wp-content/uploads/01.PALLAS_7inch_label.pdf

Contact: info@pallas-group.de

### GZ Media

GZ Media publishes a print-template catalogue with a Vinyl labels category.
The template catalogue states that customers should contact `info@gzmedia.eu`
if a product is not listed. The GZ contact page lists GZ Media a.s. in
Loděnice, Czech Republic, with `info@gzmedia.cz`.

Downloaded and inspected on 2026-06-04:

- `GD30E.pdf`, listed under 12 in LP label, states `R50`, `R3.5`, and open
  format `100.0 x 100.0`. The registry encodes this as 100 mm trim and a 7 mm
  center-hole guide. No source-specific bleed or safety distance is encoded.
- `GD17E.pdf`, listed under 7 in SP label, states `R42`, `R3.5`, and open
  format `84.0 x 84.0`. The registry encodes this as 84 mm trim and a 7 mm
  center-hole guide. No source-specific bleed or safety distance is encoded.

The same catalogue lists 10 in and picture-disc label files, but the currently
downloaded PDFs did not expose text-backed dimensions. Those remain extraction
targets rather than encoded templates.

Sources:

- https://secure.gzmedia.eu/vykresova-dokumentace/templates.aspx
- https://secure.gzmedia.eu/vykresova-dokumentace/Files/GD30E_97fd16cc9359d8cde7a9f3ae27c42dd7.pdf
- https://secure.gzmedia.eu/vykresova-dokumentace/Files/GD17E_c7b79731d23f7386ed99311237a662c3.pdf
- https://www.gzmedia.com/contact/

Contact: info@gzmedia.cz

### Pitch Beats by Sweden

Pitch Beats publishes a template page for outer envelopes and labels in PDF and
INDD formats. The page lists 12 in labels at 100 mm and 7 in labels at 90 mm.
It asks for images at 300 dpi at 1:1 scale, not below 200 dpi, recommends CMYK
profiles Coated Fogra 39 or Uncoated Fogra 29 depending on finish, and says to
turn off or remove the template layer before creating the printable PDF.

The registry encodes only the published trim diameters for these entries. No
source-specific bleed, safety, center-hole, or dink diameter is encoded for
Pitch Beats until the linked template files are inspected.

Sources:

- https://pitchbeats.com/templates/
- https://pitchbeats.com/en

Contacts: order@pitchbeats.com, info@pitchbeats.com

### VinylPressing.com.au

The VinylPressing.com.au 12 in sleeve with 3 mm spine template says final art
must have 3 mm bleed on each cut edge, be CMYK and 300 DPI, outline all fonts,
and be saved as a print-ready PDF. The visible template geometry includes two
314 mm panels, a 3 mm spine, and a 631 mm total width.

Source: https://vinylpressing.com.au/wp-content/uploads/2019/07/Vinyl-12-Inch-Sleeve-3mm-spine.pdf

### Record Industry / Sony MediaTemplates

The Record Industry / Sony 12 in jacket 3 mm spine template lists document size
667 x 352 mm, a 3 mm spine, and an optional 100 mm punch hole. The registry keeps
this as derived geometry until we validate all flap, bleed, and safe-area boxes.

Source: https://mediatemplates.sonymusic.com/uploads/template/pdf/227/RI_12inch_3mm_v92012.pdf

### Green Lakes Pressing

Green Lakes recommends PDF/X-1a, CMYK with a suitable profile, 300 DPI raster
images, 3-5 mm bleed, 5 mm safe margins, outlined or embedded fonts, and separate
clearly labeled layers for special finishes or dielines. Their template list
includes 12 in labels at 100 mm, 7 in labels at 92 mm, 12 in outer sleeves with
3 mm, 4 mm, and 7 mm spines, inserts, gatefolds, tip-ons, obi strips, and
download cards.

Source: https://greenlakespressing.com/resources/vinyl-cover-artwork/

Downloaded and inspected on 2026-06-04:

- `12inch-center-label.pdf` exposes a 100 mm trim box and 106 mm bleed box.
- `7inch_Vinyl-Center-Label.zip` contains `7inch_Vinyl Center Label.pdf` and
  `.ai`; the template text stream exposes 92 mm and 98 mm dimensions.

Contact: https://greenlakespressing.com/contact/ and hi@greenlakespressing.com

### Sonic Wax Pressing

Sonic Wax publishes a sleeve and label template page with downloadable ZIP
archives for 7 in and 12 in products. The visible template list includes 7 in UK
and US labels, 7 in sleeves, inserts, obi strips, 12 in labels, discobags,
12 in sleeves with 3 mm, 5 mm, and 7 mm spines, gatefolds, inserts, and obi
strips.

Downloaded and inspected on 2026-06-04:

- `12__Label.zip` contains `12inch Lable Template 2up 106mm.pdf` and `.eps`.
  `pdfinfo` reports a 357.165 x 657.638 pt page, approximately 126 x 232 mm.
- `7__UK_Label.zip` contains `7inch UK Label Template 2up 90mm.pdf` and `.eps`.
  `pdfinfo` reports the same approximately 126 x 232 mm page.
- `12__Sleeve_3mm_Spine.zip` contains `12 Sleeve 633x315 (3mm spine).pdf` and
  `.eps`. `pdfinfo` reports a 1858.68 x 957.259 pt page, approximately
  655.7 x 337.7 mm. The filename exposes the 633 x 315 mm named area and 3 mm
  spine.
- `7__Sleeve_3mm_Spine.zip` contains `7inch Sleeve185mm 3mm Spine.pdf` and
  `.eps`. `pdfinfo` reports a 1121.67 x 588.755 pt page, approximately
  395.7 x 207.7 mm. The filename exposes the 185 mm sleeve reference and 3 mm
  spine.

The inspected PDFs are vector-only and did not expose searchable text for final
trim, center holes, safety, bleed, or font rules. The registry therefore records
these Sonic Wax templates as `DerivedFromPlantTemplate` until we manually
validate the art objects or confirm details with the plant.

Source: https://sonicwaxpressing.io/pages/artwork-templates

### UK Market Coverage Audit

The registry does not yet cover the entire UK market. As of 2026-06-04 the
fixture has 31 UK supplier records. Template-backed UK entries are Sonic Wax
Pressing, The Jungle Record Press, Press On Vinyl, Bladud Flies, Cram
Duplication, DMS Vinyl, Mighty Media Discs, and SixtySix Productions. Supplier
records without encoded templates are market-coverage metadata only until source
geometry is extracted or confirmed.

Public UK-market research still found additional operators or brokers that need
source-by-source review before we can claim full coverage:

- Seabass Vinyl: UK/Scotland pressing plant; public packaging/artwork page links
  external artwork templates and publishes `info@seabassvinyl.com`. Template
  assets still need geometry extraction before registry import.
- The Vinyl Factory / Noon Media Solutions: Noon says it is the official partner
  of The Vinyl Factory, publishes `nms@thevinylfactory.com` and
  `sales@noonmediasolutions.com`, and has downloadable 7 in/12 in label
  templates. Those template PDFs need geometry extraction before import.
- Cram Duplication: UK broker/manufacturer with public artwork instructions and
  vinyl templates. The 10/12 in label is now encoded; the 7 in label PDF still
  needs vector extraction.
- DMS Vinyl: public template directory and artwork spec. The 7 in small-hole,
  7 in big-hole, 7 in 90 mm non-standard big-hole, and 12 in label templates are
  now encoded from source PDF text. DMS jackets, sleeves, inserts, stickers, and
  other non-label assets remain extraction targets.
- Mighty Media Discs: public vinyl page and template links are now represented
  in supplier metadata, with 7 in and 12 in label templates encoded from PDF
  text.
- SixtySix Productions: supplier metadata and the public 12 in label PDF are now
  represented. Additional templates/forms remain extraction targets.
- Cutsy: UK record manufacturing page publishes `pressed@cutsy.co.uk` and label
  options for 7 in UK, 7 in US, and 12 in labels, but no source-specific label
  diameters are encoded yet.
- Vinyl Press UK: official site confirms a Yorkshire vinyl record manufacturing
  facility for 7 in and 12 in records and publishes `info@vinylpressuk.com`.
  The supplier record is now encoded, but no label/artwork geometry is encoded
  until official templates or plant-confirmed dimensions are found.
- CD Unity: official vinyl pressing page states the service is based in Glasgow
  and its pressing facility is in the United Kingdom, with `info@cdunity.com`
  on the contact page. The supplier record is now encoded with no template
  geometry.
- Takt Direct UK: official Takt Direct pages confirm UK ordering through the
  High Wycombe branch and a templates/FAQ page. The registry records the UK
  branch as supplier metadata only; the published imprint email is legal contact
  metadata, not a sales/artwork contact, so it is not used as `contactEmail`.
- Hoxton Vinyl, Diamond Black Vinyl, Sound Performance, AGR Manufacturing, VDC
  Group, Mediaplant, Media Hut, Disc Wizards, Eurodisc Manufacturing, Well
  Tempered, Trapeze Music Manufacturing, Duplion, Cyclone Music, Breed Media,
  Key Production, Packaged Sounds Group, and Vinyl Presents now have supplier
  records, but no encoded label geometry yet. Packaged Sounds now resolves to
  the current official `packagedsounds.com` site; the older Aardvark domain is
  treated only as a legacy/source-discovery endpoint.
- Sprint Records redirected to a suspended-site page during this audit, so it is
  not represented as an active supplier record.
- Melody Manufacturing Ltd appears dissolved in Companies House/Gazette records
  in 2025, so it should not be treated as an active UK market supplier.
- FairSound is represented by Press On Vinyl for plant/spec purposes because
  its own FAQ says records are pressed in-house at Press On Vinyl and points to
  Press On artwork templates.
- Blood Records / Blood Recs remains product/campaign research rather than a
  plant/supplier record because the public site does not expose a manufacturer
  service or artwork-template intake for outside projects.
- Curved Pressing, Once Upon A Time Music, legacy lathe-cut services, mastering
  houses, label-paper suppliers, and packaging-only suppliers remain unresolved
  unless we decide the registry should cover non-pressing support businesses.

Sources:

- https://vinyl-pressing-plants.com/all-vinyl-pressing-plants-list/countries/United-Kingdom/
- https://vinylrecordpress.media/collection/united-kingdom/
- https://seabassvinyl.com/our-services/packaging-artwork/
- https://noonmediasolutions.com/vinyl-pressing
- https://noonmediasolutions.com/templates
- https://vinylpressuk.com/
- https://vinylpressuk.com/contact/
- https://www.cdunity.com/cd-unity-vinyl-pressing/
- https://www.cdunity.com/contact-us/
- https://www.takt-direct.com/
- https://www.takt-direct.com/templates-faq/
- https://www.takt-direct.com/contact/
- https://packagedsounds.com/
- https://packagedsounds.com/contact/
- https://www.cramduplication.co.uk/artwork-instructions/
- https://www.discmanufacturingservices.com/vinyl/templates
- https://www.discmanufacturingservices.com/home/artworkspecs
- https://cutsy.co.uk/vinyl-record-pressing
- https://cyclonemusic.co.uk/vinyl-pressing/
- https://www.breedmedia.co.uk/services/vinyl-record-pressing/
- https://www.keyproduction.co.uk/services/vinyl-pressing/
- https://aardvarkpressing.com/
- https://www.vinylpresents.co.uk/
- https://hoxtonvinyl.com/vinyl-pressing
- https://diamondblackvinyl.com/
- https://www.mightymediadiscs.co.uk/vinyl-pressing/
- https://www.soundperformance.co.uk/vinyl-pressing
- https://agrm.co.uk/
- https://www.vdcgroup.com/product-and-services/vinyl-pressing/
- https://www.mediaplant.co.uk/contact-us/
- https://www.mediahut.co.uk/cd-dvd-replication-music/vinyl.shtml
- https://www.discwizards.com/vinyl-pressing
- https://www.discwizards.com/contact-us.htm
- https://www.euro-disc.co.uk/page-1
- https://www.pressingvinyl.co.uk/index.php/contact_vinyl_brokers/
- https://trapezemusicmanufacturing.com/?page_id=28
- https://www.sixtysixproductions.co.uk/blog/sixtysix-productions-music-manufacturing-at-it-s-finest
- https://www.duplion.com/
- https://www.duplion.com/contact/
- https://find-and-update.company-information.service.gov.uk/company/07739468
- https://www.sprintrecords.co.uk/
- https://find-and-update.company-information.service.gov.uk/company/12189782/filing-history

### DMS Vinyl

DMS publishes a vinyl template directory with PDF and IDML paper-part templates
and an artwork spec page. Its artwork spec says printed parts should be supplied
as press quality PDFs, colour images should be CMYK, raster artwork should be at
least 300 dpi, fonts should be embedded or outlined, and maximum ink coverage is
320%.

Downloaded and inspected on 2026-06-04:

- `12-Inch-Vinyl-Labels---SIDE-A-+-B.pdf` states 100 mm trim, 7.24 mm center
  hole, 31.5 mm inner vinyl ridge, 3 mm safety area, and 3 mm required bleed
  area. The registry encodes 106 mm bleed/artwork, 100 mm trim, 94 mm safety,
  and 7.24 mm center hole. The 31.5 mm inner vinyl ridge is documented but not
  encoded as a guide layer.
- `7-Inch-Vinyl-Labels---Small-Centre-Holes---SIDE-A-+-B.pdf` states 84 mm
  trim, 7.24 mm center hole, 31.5 mm inner vinyl ridge, 3 mm safety area, and
  3 mm required bleed area. The registry encodes 90 mm bleed/artwork, 84 mm
  trim, 78 mm safety, and 7.24 mm center hole.
- `7-Inch-Vinyl-Labels---Big-Centre-Holes---SIDE-A-+-B.pdf` states 84 mm trim,
  38 mm big center hole, 3 mm safety area, and 3 mm required bleed area. The
  registry encodes 90 mm bleed/artwork, 84 mm trim, 78 mm safety, and 38 mm
  dink.
- `7-Inch-Vinyl-Labels---Big-Centre-Holes---SIDE-A-+-B---90mm---Non-Standard.pdf`
  states 90 mm trim, 38 mm big center hole, 3 mm safety area, and 3 mm required
  bleed area. The registry encodes 96 mm bleed/artwork, 90 mm trim, 84 mm
  safety, and 38 mm dink.

Sources:

- https://www.discmanufacturingservices.com/vinyl/templates
- https://www.discmanufacturingservices.com/home/artworkspecs
- https://api.discmanufacturingservices.com/storage/templates/17186/12-Inch-Vinyl-Labels---SIDE-A-%2B-B.pdf
- https://api.discmanufacturingservices.com/storage/templates/17195/7-Inch-Vinyl-Labels---Small-Centre-Holes---SIDE-A-%2B-B.pdf
- https://api.discmanufacturingservices.com/storage/templates/17195/7-Inch-Vinyl-Labels---Big-Centre-Holes---SIDE-A-%2B-B.pdf
- https://api.discmanufacturingservices.com/storage/templates/17195/7-Inch-Vinyl-Labels---Big-Centre-Holes---SIDE-A-%2B-B---90mm---Non-Standard.pdf

### Cram Duplication

Cram's artwork instructions say the magenta line is the trim line, important
logos/text should stay at least 3 mm inside trim, bleed is usually 3 mm, artwork
should be 300 ppi, and print-ready files should be CMYK PDFs. They also accept
editable Photoshop, Illustrator, or InDesign working files when print setup
needs checking at their end.

Downloaded and inspected on 2026-06-04:

- `10-12-inch-vinyl-label.pdf` states required 3 mm bleed, 100 mm trim, and
  7.24 mm center hole. The registry encodes this as 106 mm bleed/artwork,
  100 mm trim, and 7.24 mm hole.
- `7-inch-vinyl-label.pdf` downloaded from the public artwork page, but text
  extraction did not expose dimensions. It remains a vector extraction target.

Sources:

- https://www.cramduplication.co.uk/artwork-instructions/
- https://www.cramduplication.co.uk/download/templates/10-12-inch-label/10-12-inch-vinyl-label.pdf
- https://www.cramduplication.co.uk/download/templates/7-inch-label/7-inch-vinyl-label.pdf

Contact: info@cramduplication.co.uk

### Mighty Media Discs

Mighty Media Discs publishes a vinyl pressing page with downloadable template
PDFs. The page states that they provide UK vinyl pressing, that their minimum
quantity is 100, and that they can manufacture 7 in, 10 in, and 12 in records.
It also publishes `hello@mightymediadiscs.co.uk`.

Downloaded and inspected on 2026-06-04:

- `7inch Label Template 2up 90mm.pdf` states final trim size 84 mm, dunkout
  38 mm, bleed 84-90 mm (3 mm all round), 90 mm pair diameter, and plus/minus
  1 mm trim tolerance. The registry encodes the two-up PDF page, two 90 mm
  bleed/artwork guides, two 84 mm trim guides, and two 38 mm dink guides. The
  source does not state a small spindle-hole diameter, so no separate small-hole
  guide is encoded.
- `12inch Lable Template 2up 106mm.pdf` states final trim size 101 mm, bleed
  101-106 mm (2.5 mm all round), no-type critical area 95-7 mm, 106 mm pair
  diameter, and plus/minus 1 mm trim tolerance. The registry encodes the two-up
  PDF page, two 106 mm bleed/artwork guides, two 101 mm trim guides, two 95 mm
  safety guides, and two 7 mm hole guides.

Sources:

- https://www.mightymediadiscs.co.uk/vinyl-pressing/
- https://www.mightymediadiscs.co.uk/templates/vinyl/7inch%20Label%20Template%202up%2090mm.pdf
- https://www.mightymediadiscs.co.uk/templates/vinyl/12inch%20Lable%20Template%202up%20106mm.pdf

Contact: hello@mightymediadiscs.co.uk

### SixtySix Productions

SixtySix Productions publishes music manufacturing services and contact details
for its Brighton office, including `sales@sixtysixproductions.co.uk` and
`production@sixtysixproductions.co.uk`. Its public site links templates and
forms; search-discovered template PDFs are hosted under the same domain.

Downloaded and inspected on 2026-06-04:

- `3a.-12inch-label-spec.pdf` states size 100 x 100 mm, centre hole 7.5 mm, and
  bleed area 3 mm. The registry encodes a 106 mm artwork/bleed guide, 100 mm
  trim guide, and 7.5 mm center-hole guide. Extracted PDF metadata includes
  CMYK/process proof information, but no extracted raster-PPI requirement.

Sources:

- https://www.sixtysixproductions.co.uk/blog/sixtysix-productions-music-manufacturing-at-it-s-finest
- https://www.sixtysixproductions.co.uk/files/2016-11/3a.-12inch-label-spec.pdf

Contact: production@sixtysixproductions.co.uk

### Deepgrooves

Deepgrooves asks for 300 dpi flattened PDF files in CMYK or Pantone, based on
their own templates only. It asks for PDF/X-1a:2001/1.3 without scaling or
compression, FOGRA39, no visible cutting/folding/punching lines, and artwork
that fills cut-out holes rather than leaving them blank. Their artwork
specification says label, sleeve, and insert bleed is 5 mm on all sides, and
illustrates a 100 mm label with 5 mm bleed per side as a 110 mm final file.
Their vinyl template list publishes 12 in and 10 in labels at 100 mm and 7 in
labels at 92 mm.

Downloaded and inspected on 2026-06-04:

- `Deepgrooves-12inch-Label-100-mm-Template.zip` contains a 12 in 100 mm label
  PDF template.
- `Deepgrooves-7inch-Label-92-mm-Template.zip` contains a 7 in 92 mm label PDF
  template.
- `Deepgrooves-7inch-Label-92-mm-Info-Sheet.pdf` states open format 102 mm,
  printing format 92 mm, and punch holes of 7.3 mm or 38 mm.

Sources:

- https://deepgrooves.eu/vinyltemplates/
- https://deepgrooves.eu/specifications/artworkspecifications/
- https://deepgrooves.eu/wp-content/uploads/2018/12/Deepgrooves-7inch-Label-92-mm-Info-Sheet.pdf

Contact: https://deepgrooves.eu/about/contact/ and info@deepgrooves.eu

### R.A.N.D. Muzik

R.A.N.D. publishes direct PDF product specifications for 12 in labels, 7 in
small-hole labels, and 7 in large-hole labels. The 12 in spec states a 106 x
106 mm print file including bleed, 100 x 100 mm final format, 7.3 mm center
hole, and 3 mm bleed. The 7 in small-hole spec states a 98 x 98 mm print file,
92 x 92 mm final format, 7.3 mm center hole, and 3 mm bleed. The 7 in
large-hole spec states the same 98 x 98 mm print file and 92 x 92 mm final
format with a 38 mm center hole. The PDFs ask for PDF/TIFF, 300 dpi,
CMYK/grayscale, no cutting marks or color-control strips, and 2 mm spacing
between text or important information and the cutting line.

Sources:

- https://www.randmuzik.de/media/label_12inch_2019.pdf
- https://www.randmuzik.de/media/label_7inch_km_2019.pdf
- https://www.randmuzik.de/media/label_7inch_gm_2019.pdf

Contact: https://www.randmuzik.de/en/contact/ and info@randmuzik.de

### Pirates Press

Pirates Press technical conditions state that basic labels are produced at
84 mm for 7 in records and 100 mm for 10 in and 12 in records. Picture-disc
labels are listed as 168 mm for 7 in, 242 mm for 10 in, and 292 mm for 12 in.
The same source recommends at least 2 mm between text and the outer label edge,
no text within 10 mm of the center for normal labels, and no text within 40 mm
of the center for 7 in labels with big holes. The contact page lists direct
manufacturing quote emails for the U.S. and Europe.

Sources:

- https://www.piratespress.com/products/vinyl-technical-conditions/
- https://www.piratespress.com/contact/

Contact: quotes@piratespress.com and quotes@piratespresseurope.com

### New Orleans Record Press

New Orleans Record Press publishes a template page with direct PDF links for
12 in center labels, jackets, inner sleeves, inserts, 7 in center labels,
7 in glue pockets, stickers, and download cards. Its artwork guidance asks for
templates to be used at source scale, with art on a separate layer from the
template. It accepts 300-600 DPI unflattened PSD/EPS/AI files or layered/
editable PDFs, says PDF submissions may turn all template guides and labels
off, and says fonts should be converted to shapes or included by email.

Downloaded and inspected on 2026-06-04:

- `NORP-centerlabel.pdf` text extraction states 12 in center labels are
  4 in circles, which converts to 101.6 mm.
- `NORP-seveninch-centerlabels-smallhole.pdf` text extraction states 7 in
  center labels are 3.625 in circles and names the small-hole variant. The
  label circle converts to 92.075 mm; the center-hole diameter is not encoded.
- `NORP-seveninch-centerlabels-largehole.pdf` text extraction states 7 in
  center labels are 3.625 in circles and names the large-hole variant. The
  label circle converts to 92.075 mm; the dink or spindle-hole diameter is not
  encoded.

Sources:

- https://www.neworleansrecordpress.com/templates/
- https://www.neworleansrecordpress.com/resources/NORP-centerlabel.pdf
- https://www.neworleansrecordpress.com/resources/NORP-seveninch-centerlabels-smallhole.pdf
- https://www.neworleansrecordpress.com/resources/NORP-seveninch-centerlabels-largehole.pdf

Contact: info@neworleansrecordpress.com

### Digital Force

Digital Force publishes a direct 12 in record-label PDF template. The template
states a 4 in final cut dimension, 1/8 in bleed including within the center
hole, and a 1/8 in keep-clear area for type and images from the edge including
around the center hole. It asks for artwork on a separate layer and for the
template layer to remain locked, so final guide-layer handling should be
confirmed before treating a generated file as plant-ready.

Downloaded and inspected on 2026-06-04:

- `12-Record-Label-Template.pdf` text extraction states final cut is 4 in,
  bleed is 1/8 in, and type/image safety is 1/8 in. These convert to
  101.6 mm trim, 107.95 mm bleed diameter, and 95.25 mm safety diameter.

Sources:

- https://digitalforce.com/wordpress1/wp-content/uploads/2014/10/12-Record-Label-Template.pdf
- https://digitalforce.com/contact-us/

Contact: frontdesk@digitalforce.com and dfgraphics@digitalforce.com

### Audio Geography Studios

Audio Geography is a Providence, Rhode Island lathe-cut record producer. Its
packaging page states that 7 in size labels are 3.25 in circles, that 5 in and
10 in records also use the 7 in size labels, and that 12 in size labels are
4 in circles. The page links PDF/JPG label templates and center-label
directions, but the downloaded direction PDF is image-only and did not expose
additional searchable bleed, safety, or center-hole dimensions.

Sources:

- https://www.audiogeography.com/packaging
- https://www.audiogeography.com/contact

Contact: tragwag@gmail.com

### VinylPressing.com.au

The `Vinyl-10-or-12-Inch-Label.pdf` template states 100 mm trim, 106 mm with
3 mm bleed, 94 mm safety margin, and a 7.2 mm center hole. It says not to cut
the centre hole out of the artwork, not to include the template or other
non-print information in the final print-ready PDF, to use CMYK and 300 DPI,
to outline fonts, and to save as print-ready PDF 1.5 or higher.

The 7 in label PDF also downloaded successfully, but text extraction exposed
multiple conflicting visible diameters: 92 mm, 87 mm bleed, 85 mm cut, and
77 mm safety. That 7 in template remains an extraction target until we inspect
the vector geometry or confirm the intended product variant with the plant.

Source: https://vinylpressing.com.au/wp-content/uploads/2020/02/Vinyl-10-or-12-Inch-Label.pdf

Contact: print@vinylpressing.com.au

### Pressing Business

Pressing Business publishes a vinyl print templates page with downloadable ZIPs
for record labels and packaging. The page exposes `info@pressingbusiness.co`;
the label ZIPs contain PDF and EPS versions from MonotypePressing America.

Downloaded and inspected on 2026-06-04:

- `12-inch record label (2355).pdf` has a 297 x 208.33 mm page box. The visible
  callouts label a 100 mm final trim circle, 94 mm no-type/safety circle,
  13 mm central clearance circle, and 7 mm center hole. The green outer artwork
  ring measures as 106 mm when the vector drawing is scaled against the
  100 mm trim callout.
- `7-inch record label (2408).pdf` has the same page box. The visible callouts
  label an 84 mm final trim circle and 7 mm center hole. The green outer artwork
  ring measures as 90 mm when scaled against the 84 mm trim callout. No separate
  safety/no-type ring is visible in this source.

Sources:

- https://pressingbusiness.co/vinyl-print-templates
- https://pressingbusiness.co/s/12-inch-record-label-2355.zip
- https://pressingbusiness.co/s/7-inch-record-label-2408.zip
- https://pressingbusiness.co/about-us

Contact: info@pressingbusiness.co

### XVINYLX

XVINYLX publishes an official artwork-template page with direct PDFs for 7 in,
10 in, and 12 in vinyl labels plus packaging. Its public home page describes
vinyl record pressing and says the main office is in Prague, Czech Republic,
with USA and Australia offices. The official wiki manufacturing page says quotes
and order specifications can be sent by email, but the official HTML did not
expose a plain-text email address during inspection. Search-discovered contact
metadata lists `info@xvinylx.com`; the registry keeps that email with the
official contact URL.

Downloaded and inspected on 2026-06-04:

- `12inch_labels.pdf` is an A4 landscape PDF titled 12 in center label GD30E.
  Vector measurement gives approximately 105.69 mm bleed, 99.88 mm cut/trim,
  93.78 mm safety, 12.70 mm center safety, and 7.23 mm center hole.
- `7inch_labels_smallHole.pdf` is an A4 landscape PDF titled 7 in center label
  GD17E. Vector measurement gives approximately 89.90 mm bleed, 83.90 mm
  cut/trim, 77.62 mm safety, 13 mm center safety, and 7.23 mm center hole.
- `7inch_labels_bigHole.pdf` is an A4 landscape PDF titled 7 in center label,
  big hole, GD17E. Vector measurement gives approximately 89.62 mm bleed,
  83.62 mm cut/trim, 77.62 mm safety, and 37.67 mm large-hole cut. The large
  center cut is recorded as a dink guide; this source does not show a separate
  small spindle guide.

The template legends map solid line to cut line, dashed line to bleed line, and
dotted line to safety line. They ask for 3 mm bleed on all sides, text/logos at
least 3 mm from die-lines, CMYK rather than RGB, 300 dpi image resolution,
1200 dpi bitmap resolution, no transparency, maximum 320% total ink coverage,
and guide lines kept in a separate layer or template colour.

Sources:

- https://xvinylx.com/info/Artwork_templates
- https://xvinylx.com/templates/12inch_labels/12inch_labels.pdf
- https://xvinylx.com/templates/7inch_labels/7inch_labels_smallHole.pdf
- https://xvinylx.com/templates/7inch_labels/7inch_labels_bigHole.pdf
- https://xvinylx.com/
- https://xvinylx.com/info/Manufacturing_with_XVINYLX

Contact: info@xvinylx.com (search-discovered contact metadata); official URL
https://xvinylx.com/contact

### elasticStage

elasticStage publishes official artwork templates for two upload routes:
Quick & Easy Upload and Designer Upload. Quick & Easy accepts JPG/PNG and uses
a separate raster vinyl template. Designer Upload is intended for full manual
layout control and requires PDF/X-4:2008 with Coated Fogra39 embedded.

Downloaded and inspected on 2026-06-04:

- `Vinyl_AW_jpg_v1_1.zip` contains Quick & Easy JPG slots for vinyl front,
  back, spine, label, inner sleeve, and booklet imagery. The label JPG is
  2504 x 2504 pixels at 600 ppi; front/back artwork slots are around 12.5 in
  square at 300 ppi.
- `12_Vinyl_v1_6.zip` contains Illustrator `.ai` files and PDF/JPG references
  for 12 in cover, inner sleeve, labels, and booklet.
- `vinyl_12_labels_v1_6_dims.pdf` states document size 106 mm x 212 mm, has
  two stacked labels for SIDE A/C and SIDE B/D, labels 100 mm trim and 3 mm
  bleed, and renders an 8 mm center-hole trim guide in the vector geometry.
- `vinyl_12_cover_v1_6_dims.pdf` states document size 673 mm x 358 mm, with
  315 mm front/back panels, 312 mm panel height, 3 mm spine, 20 mm folds, and
  3 mm bleed.
- `vinyl_12_inner_sleeve_v1_6_dims.pdf` states document size 648 mm x 349 mm,
  with 304 mm front/back panels, 309 mm panel height, 20 mm folds, and 3 mm
  bleed.
- `vinyl_12_booklet_v1_6_dims.pdf` states document size 640 mm x 340 mm for
  the booklet spreads, with 300 mm pages, 20 mm folds, and 3 mm bleed.

Only the Designer labels are normalized as record proof geometry for now. The
Quick & Easy archive is recorded as a packaging-guide source so it is cached
and searchable. The Designer cover, inner sleeve, and booklet formats are
recorded here as source dimensions, but need a dedicated fold/flap
decomposition pass before we encode plant-ready rectangular guides.

Sources:

- https://elasticstage.zendesk.com/hc/en-gb/articles/32126215476241-elasticStage-Vinyl-CD-Artwork-Templates-Download-Dimensions
- https://elasticstage.zendesk.com/hc/en-gb/articles/44277281144849-elasticStage-Artwork-Specifications-Vinyl-CD
- https://es-ew1-prod-artwork-templates.s3.eu-west-1.amazonaws.com/templates/Vinyl_AW_jpg_v1_1.zip
- https://es-ew1-prod-artwork-templates.s3.eu-west-1.amazonaws.com/templates/12_Vinyl_v1_6.zip
- https://elasticstage.com/es/contact

### Additional Published Templates Needing Geometry Extraction

Hellbender, Standard Vinyl, and Hand Drawn Pressing all publish useful public
template pages with direct PDF or ZIP assets. Hellbender and Standard Vinyl
label geometry is now partially encoded where vector measurement was strong
enough; their packaging templates still need careful rectangle/fold/flap
decomposition before registry import.

Inspected on 2026-06-04:

- Hellbender's 10/12 in label and 7 in small-hole label entries are encoded.
  Remaining Hellbender targets are 7 in and 12 in jackets after rect/fold/flap
  decomposition.
- Standard Vinyl's 12 in, 10 in, 7 in small-hole, and 7 in large-hole center
  labels are encoded. Remaining Standard Vinyl targets include picture discs,
  etched records, jackets, gatefolds, sleeves, inserts, posters, booklets, obi
  strips, stickers, and record boxes.
- Hand Drawn Pressing publishes direct PDFs and ZIP files, including a
  7 in small-hole center-label PDF and 12/7 in label ZIPs. The inspected
  7 in PDF exposes only the label name in text, not the dimensions.

Sources:

- https://hellbendervinyl.com/pages/templates
- https://standardvinyl.com/place-an-order/label-packaging-templates/
- https://www.handdrawnpressing.com/art-templates-2

Contacts found:

- Hellbender: hello@hellbendervinyl.com
- Standard Vinyl: hello@standardvinyl.com
- Hand Drawn Pressing: vinyl@handdrawnrecords.com
- Digital Force: frontdesk@digitalforce.com

### Blood Records / Blood Recs

Blood Records is useful as a product and campaign reference, but it does not
appear to publish downloadable plant templates or client-facing dielines on its
public site. Their About page says they produce limited-edition, exclusively
pressed vinyl records, and their product catalog shows recurring special formats
such as zoetrope picture discs, hand-numbered editions, gatefold sleeves, hidden
7 in singles, and artist collaborations.

Their public FAQ gives a practical zoetrope viewing constraint: use a turntable
at 33 rpm and a smartphone at 30 fps, or a strobe app at 30 Hz under strong
light. That is product-relevant for Presser if Bitneedle adds animated picture
disc or zoetrope proof modes, but it is not a plant-ready artwork template.

Sources:

- https://blood-records.co.uk/
- https://blood-records.co.uk/pages/about/
