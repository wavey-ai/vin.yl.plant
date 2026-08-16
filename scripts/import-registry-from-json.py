"""Generates the plant registry as Rust source from the curated fixture.

Run once to move the registry into the crate as typed data. After that the
generated file is the source: it is edited directly, and this script is only
useful if the registry ever needs re-importing from JSON.
"""
import json
import sys

KIND = {
    "center-label": "CenterLabel",
    "picture-label": "PictureLabel",
    "outer-sleeve": "OuterSleeve",
    "inner-sleeve": "InnerSleeve",
    "gatefold-sleeve": "GatefoldSleeve",
    "insert": "Insert",
    "packaging-guide": "PackagingGuide",
}
CONFIDENCE = {
    "plant-published": "PlantPublished",
    "derived-from-plant-template": "DerivedFromPlantTemplate",
    "needs-plant-confirmation": "NeedsPlantConfirmation",
}
LAYER = {
    "bleed": "Bleed",
    "trim": "Trim",
    "safety": "Safety",
    "fold": "Fold",
    "spine": "Spine",
    "hole": "Hole",
    "dink": "Dink",
}


def s(value):
    """Rust string literal."""
    if value is None:
        raise ValueError("expected a string")
    escaped = value.replace("\\", "\\\\").replace('"', '\\"')
    return f'"{escaped}"'


def opt_s(value):
    return f"Some({s(value)})" if value is not None else "None"


def f(value):
    """Rust f64 literal that round-trips."""
    out = repr(float(value))
    return out if ("." in out or "e" in out or "E" in out) else out + ".0"


def opt_f(value):
    return f"Some({f(value)})" if value is not None else "None"


def opt_u16(value):
    return f"Some({int(value)})" if value is not None else "None"


def opt_bool(value):
    return "None" if value is None else f"Some({str(bool(value)).lower()})"


def str_slice(values):
    if not values:
        return "&[]"
    inner = ", ".join(s(v) for v in values)
    return f"&[{inner}]"


def geometry(value):
    shape = value["shape"]
    if shape == "circle":
        c = value["circle"]
        return (
            "GuideGeometry::Circle { circle: CircleMm::new("
            f"{f(c['cx'])}, {f(c['cy'])}, {f(c['radius'])}) }}"
        )
    if shape == "rect":
        r = value["rect"]
        return (
            "GuideGeometry::Rect { rect: RectMm::new("
            f"{f(r['x'])}, {f(r['y'])}, {f(r['width'])}, {f(r['height'])}) }}"
        )
    raise ValueError(f"unsupported guide shape: {shape}")


def main():
    registry = json.load(open(sys.argv[1]))
    out = []
    w = out.append

    w("//! The curated manufacturer and template registry.")
    w("//!")
    w("//! This is the source of truth for plant geometry. It lives in Rust so a")
    w("//! bad measurement is a compile error rather than a runtime surprise, and")
    w("//! so every consumer — the web UI and the Apple app both link this crate —")
    w("//! reads exactly the same templates.")
    w("//!")
    w("//! Each entry records where its measurements came from and how confident")
    w("//! they are. Anything not `PlantPublished` must be confirmed with the plant")
    w("//! before a fully automated plant-ready submission.")
    w("")
    w("use crate::{")
    w("    CircleMm, GuideGeometry, GuideLayerKind, MeasurementConfidence, PlantGuide,")
    w("    PlantManufacturer, PlantPrintRequirements, PlantSourceReference, PlantTemplate, RectMm,")
    w("    RecordPlantTemplateKind,")
    w("};")
    w("")

    manufacturers = registry["manufacturers"]
    w(f"/// Every plant the registry knows, by id. {len(manufacturers)} entries.")
    w("pub static PLANT_MANUFACTURERS: &[PlantManufacturer] = &[")
    for m in manufacturers:
        w("    PlantManufacturer {")
        w(f"        id: {s(m['id'])},")
        w(f"        name: {s(m['name'])},")
        w(f"        country_code: {opt_s(m.get('countryCode'))},")
        w(f"        website_url: {opt_s(m.get('websiteUrl'))},")
        w(f"        contact_email: {opt_s(m.get('contactEmail'))},")
        w(f"        contact_url: {opt_s(m.get('contactUrl'))},")
        w("    },")
    w("];")
    w("")

    templates = registry["templates"]
    w(f"/// Every plant template the registry knows. {len(templates)} entries.")
    w("pub static PLANT_TEMPLATES: &[PlantTemplate] = &[")
    for t in templates:
        r = t["requirements"]
        w("    PlantTemplate {")
        w(f"        id: {s(t['id'])},")
        w(f"        name: {s(t['name'])},")
        w(f"        manufacturer_id: {s(t['manufacturerId'])},")
        w(f"        manufacturer: {s(t['manufacturer'])},")
        w(f"        product: {s(t['product'])},")
        w(f"        kind: RecordPlantTemplateKind::{KIND[t['kind']]},")
        w(f"        version: {opt_s(t.get('version'))},")
        d = t["document"]
        w(
            f"        document: RectMm::new({f(d['x'])}, {f(d['y'])}, "
            f"{f(d['width'])}, {f(d['height'])}),"
        )
        w(f"        confidence: MeasurementConfidence::{CONFIDENCE[t['confidence']]},")
        if t["guides"]:
            w("        guides: &[")
            for g in t["guides"]:
                w("            PlantGuide {")
                w(f"                id: {s(g['id'])},")
                w(f"                layer: GuideLayerKind::{LAYER[g['layer']]},")
                w(f"                geometry: {geometry(g['geometry'])},")
                w("            },")
            w("        ],")
        else:
            w("        guides: &[],")
        w("        requirements: PlantPrintRequirements {")
        w(f"            preferred_output: {s(r['preferredOutput'])},")
        w(f"            accepted_formats: {str_slice(r.get('acceptedFormats') or [])},")
        w(f"            color_modes: {str_slice(r.get('colorModes') or [])},")
        w(f"            min_raster_ppi: {opt_u16(r.get('minRasterPpi'))},")
        w(f"            min_bitmap_ppi: {opt_u16(r.get('minBitmapPpi'))},")
        w(f"            bleed_mm: {opt_f(r.get('bleedMm'))},")
        w(f"            safety_mm: {opt_f(r.get('safetyMm'))},")
        w(
            "            keep_template_layer_out_of_final: "
            f"{str(bool(r['keepTemplateLayerOutOfFinal'])).lower()},"
        )
        w(f"            embed_or_outline_fonts: {opt_bool(r.get('embedOrOutlineFonts'))},")
        w(f"            pdf_standard: {opt_s(r.get('pdfStandard'))},")
        w(
            "            output_condition_identifier: "
            f"{opt_s(r.get('outputConditionIdentifier'))},"
        )
        w(f"            notes: {str_slice(r.get('notes') or [])},")
        w("        },")
        src = t["source"]
        w("        source: PlantSourceReference {")
        w(f"            title: {s(src['title'])},")
        w(f"            url: {s(src['url'])},")
        w(f"            retrieved_on: {s(src['retrievedOn'])},")
        w("        },")
        w(f"        source_notes: {str_slice(t.get('sourceNotes') or [])},")
        w("    },")
    w("];")
    w("")

    sys.stdout.write("\n".join(out))


main()
