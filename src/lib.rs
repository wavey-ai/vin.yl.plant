mod registry;

pub use registry::{PLANT_MANUFACTURERS, PLANT_TEMPLATES};

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

pub const POINTS_PER_INCH: f64 = 72.0;
pub const MM_PER_INCH: f64 = 25.4;
pub const POINTS_PER_MM: f64 = POINTS_PER_INCH / MM_PER_INCH;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecordPlantTemplateKind {
    CenterLabel,
    PictureLabel,
    OuterSleeve,
    InnerSleeve,
    GatefoldSleeve,
    Insert,
    PackagingGuide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MeasurementConfidence {
    PlantPublished,
    DerivedFromPlantTemplate,
    NeedsPlantConfirmation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecordPlantProofMode {
    Proof,
    PlantReady,
}

impl FromStr for RecordPlantProofMode {
    type Err = RecordPlantError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "" | "proof" => Ok(Self::Proof),
            "plant-ready" | "plant_ready" | "plantready" | "final" | "submission" => {
                Ok(Self::PlantReady)
            }
            other => Err(RecordPlantError::InvalidProofMode(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GuideLayerKind {
    Bleed,
    Trim,
    Safety,
    Fold,
    Spine,
    Hole,
    Dink,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecordPlantArtifactKind {
    Proof,
    PlantReadyArtwork,
    Spec,
    Preflight,
    Readme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecordPlantArtifactStatus {
    Implemented,
    Planned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordPlantArtifact {
    pub path: &'static str,
    pub kind: RecordPlantArtifactKind,
    pub status: RecordPlantArtifactStatus,
    pub visible_guides: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecordPlantPreflightStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordPlantPreflightCheck {
    pub id: &'static str,
    pub status: RecordPlantPreflightStatus,
    pub summary: &'static str,
    pub detail: &'static str,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomRecordPlantTemplateInput {
    pub id: String,
    pub manufacturer: String,
    pub product: String,
    pub kind: RecordPlantTemplateKind,
    pub document_width_mm: f64,
    pub document_height_mm: f64,
    pub bleed_diameter_mm: Option<f64>,
    pub trim_diameter_mm: Option<f64>,
    pub safety_diameter_mm: Option<f64>,
    pub center_hole_diameter_mm: Option<f64>,
    pub dink_diameter_mm: Option<f64>,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalRecordPlantTemplateInput {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub product: String,
    pub kind: RecordPlantTemplateKind,
    #[serde(default)]
    pub version: Option<String>,
    pub document: RectMm,
    pub confidence: MeasurementConfidence,
    #[serde(default)]
    pub guides: Vec<OwnedGuide>,
    pub requirements: OwnedPrintRequirements,
    pub source: OwnedSourceReference,
    #[serde(default)]
    pub source_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomRecordPlantProofBundle {
    pub manifest: OwnedRecordPlantProofManifest,
    pub guide_svg: String,
    pub record_context: Option<RecordPlantRecordContext>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordPlantRecordContext {
    pub record_profile: String,
    pub record_diameter_mm: f64,
    pub label_diameter_mm: Option<f64>,
    pub guide_svg: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnedRecordPlantProofManifest {
    pub template_id: String,
    pub template_name: String,
    pub manufacturer: String,
    pub product: String,
    pub mode: RecordPlantProofMode,
    pub document: RectMm,
    pub source: OwnedSourceReference,
    pub requirements: OwnedPrintRequirements,
    pub artifacts: Vec<RecordPlantArtifact>,
    pub preflight: Vec<RecordPlantPreflightCheck>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnedGuide {
    pub id: String,
    pub layer: GuideLayerKind,
    pub geometry: GuideGeometry,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnedSourceReference {
    pub title: String,
    pub url: String,
    pub retrieved_on: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnedPrintRequirements {
    pub preferred_output: String,
    pub accepted_formats: Vec<String>,
    pub color_modes: Vec<String>,
    pub min_raster_ppi: Option<u16>,
    pub min_bitmap_ppi: Option<u16>,
    pub bleed_mm: Option<f64>,
    pub safety_mm: Option<f64>,
    pub keep_template_layer_out_of_final: bool,
    pub embed_or_outline_fonts: bool,
    pub pdf_standard: Option<String>,
    #[serde(default)]
    pub output_condition_identifier: Option<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RectMm {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl RectMm {
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn right(self) -> f64 {
        self.x + self.width
    }

    pub fn bottom(self) -> f64 {
        self.y + self.height
    }

    pub fn center_x(self) -> f64 {
        self.x + self.width / 2.0
    }

    pub fn center_y(self) -> f64 {
        self.y + self.height / 2.0
    }

    pub fn inset(self, margin_mm: f64) -> Self {
        Self {
            x: self.x + margin_mm,
            y: self.y + margin_mm,
            width: (self.width - margin_mm * 2.0).max(0.0),
            height: (self.height - margin_mm * 2.0).max(0.0),
        }
    }

    pub fn outset(self, margin_mm: f64) -> Self {
        Self {
            x: self.x - margin_mm,
            y: self.y - margin_mm,
            width: self.width + margin_mm * 2.0,
            height: self.height + margin_mm * 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CircleMm {
    pub cx: f64,
    pub cy: f64,
    pub radius: f64,
}

impl CircleMm {
    pub const fn new(cx: f64, cy: f64, radius: f64) -> Self {
        Self { cx, cy, radius }
    }

    pub fn diameter(self) -> f64 {
        self.radius * 2.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "shape", rename_all = "camelCase")]
pub enum GuideGeometry {
    Rect { rect: RectMm },
    Circle { circle: CircleMm },
    VerticalLine { x: f64, y1: f64, y2: f64 },
    HorizontalLine { y: f64, x1: f64, x2: f64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordPlantError {
    UnknownTemplate(String),
    InvalidProofMode(String),
    InvalidCustomTemplate(String),
    Serialize(String),
}

impl fmt::Display for RecordPlantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTemplate(id) => write!(f, "unknown record plant template: {id}"),
            Self::InvalidProofMode(mode) => write!(f, "invalid record plant proof mode: {mode}"),
            Self::InvalidCustomTemplate(message) => {
                write!(f, "invalid custom record plant template: {message}")
            }
            Self::Serialize(message) => {
                write!(f, "failed to serialize record plant data: {message}")
            }
        }
    }
}

impl std::error::Error for RecordPlantError {}

pub fn record_plant_template_proof_bundle_json(
    template_json: &str,
    mode: &str,
) -> Result<String, RecordPlantError> {
    let template: ExternalRecordPlantTemplateInput = serde_json::from_str(template_json)
        .map_err(|error| RecordPlantError::InvalidCustomTemplate(error.to_string()))?;
    let mode = RecordPlantProofMode::from_str(mode)?;
    let bundle = external_record_plant_template_proof_bundle(&template, mode)?;
    serde_json::to_string(&bundle).map_err(|error| RecordPlantError::Serialize(error.to_string()))
}

pub fn custom_record_plant_template_proof_bundle_json(
    template_json: &str,
    mode: &str,
) -> Result<String, RecordPlantError> {
    let template: CustomRecordPlantTemplateInput = serde_json::from_str(template_json)
        .map_err(|error| RecordPlantError::InvalidCustomTemplate(error.to_string()))?;
    let mode = RecordPlantProofMode::from_str(mode)?;
    let bundle = custom_record_plant_template_proof_bundle(&template, mode)?;
    serde_json::to_string(&bundle).map_err(|error| RecordPlantError::Serialize(error.to_string()))
}

pub fn external_record_plant_template_proof_bundle(
    template: &ExternalRecordPlantTemplateInput,
    mode: RecordPlantProofMode,
) -> Result<CustomRecordPlantProofBundle, RecordPlantError> {
    validate_external_template(template)?;
    Ok(CustomRecordPlantProofBundle {
        manifest: OwnedRecordPlantProofManifest {
            template_id: template.id.trim().to_string(),
            template_name: external_template_name(template),
            manufacturer: template.manufacturer.trim().to_string(),
            product: template.product.trim().to_string(),
            mode,
            document: template.document,
            source: template.source.clone(),
            requirements: template.requirements.clone(),
            artifacts: proof_artifacts(mode),
            preflight: external_preflight_checks(template, mode),
        },
        guide_svg: external_template_guide_svg(template, mode),
        record_context: external_template_record_context(template, mode),
    })
}

pub fn custom_record_plant_template_proof_bundle(
    template: &CustomRecordPlantTemplateInput,
    mode: RecordPlantProofMode,
) -> Result<CustomRecordPlantProofBundle, RecordPlantError> {
    validate_custom_template(template)?;
    let document = RectMm::new(
        0.0,
        0.0,
        template.document_width_mm,
        template.document_height_mm,
    );
    let guides = custom_template_guides(template);
    let requirements = custom_template_requirements(template);
    let source = OwnedSourceReference {
        title: "User-provided dimensions".to_string(),
        url: String::new(),
        retrieved_on: String::new(),
    };
    Ok(CustomRecordPlantProofBundle {
        manifest: OwnedRecordPlantProofManifest {
            template_id: template.id.trim().to_string(),
            template_name: custom_template_name(template),
            manufacturer: template.manufacturer.trim().to_string(),
            product: template.product.trim().to_string(),
            mode,
            document,
            source,
            requirements,
            artifacts: proof_artifacts(mode),
            preflight: custom_preflight_checks(mode),
        },
        guide_svg: custom_template_guide_svg(template, &guides, mode),
        record_context: custom_template_record_context(template, &guides, mode),
    })
}

fn validate_custom_template(
    template: &CustomRecordPlantTemplateInput,
) -> Result<(), RecordPlantError> {
    if template.id.trim().is_empty() {
        return Err(RecordPlantError::InvalidCustomTemplate(
            "id is required".to_string(),
        ));
    }
    if template.manufacturer.trim().is_empty() {
        return Err(RecordPlantError::InvalidCustomTemplate(
            "manufacturer is required".to_string(),
        ));
    }
    if template.product.trim().is_empty() {
        return Err(RecordPlantError::InvalidCustomTemplate(
            "product is required".to_string(),
        ));
    }
    validate_positive_mm("documentWidthMm", template.document_width_mm)?;
    validate_positive_mm("documentHeightMm", template.document_height_mm)?;
    for (label, value) in [
        ("bleedDiameterMm", template.bleed_diameter_mm),
        ("trimDiameterMm", template.trim_diameter_mm),
        ("safetyDiameterMm", template.safety_diameter_mm),
        ("centerHoleDiameterMm", template.center_hole_diameter_mm),
        ("dinkDiameterMm", template.dink_diameter_mm),
    ] {
        if let Some(value) = value {
            validate_positive_mm(label, value)?;
        }
    }
    Ok(())
}

fn validate_external_template(
    template: &ExternalRecordPlantTemplateInput,
) -> Result<(), RecordPlantError> {
    if template.id.trim().is_empty() {
        return Err(RecordPlantError::InvalidCustomTemplate(
            "id is required".to_string(),
        ));
    }
    if template.manufacturer.trim().is_empty() {
        return Err(RecordPlantError::InvalidCustomTemplate(
            "manufacturer is required".to_string(),
        ));
    }
    if template.product.trim().is_empty() {
        return Err(RecordPlantError::InvalidCustomTemplate(
            "product is required".to_string(),
        ));
    }
    validate_positive_mm("document.width", template.document.width)?;
    validate_positive_mm("document.height", template.document.height)?;
    if template.guides.is_empty() {
        return Err(RecordPlantError::InvalidCustomTemplate(
            "guides are required".to_string(),
        ));
    }
    Ok(())
}

fn validate_positive_mm(label: &str, value: f64) -> Result<(), RecordPlantError> {
    if value.is_finite() && value > 0.0 {
        return Ok(());
    }
    Err(RecordPlantError::InvalidCustomTemplate(format!(
        "{label} must be a positive finite millimetre value"
    )))
}

fn external_template_name(template: &ExternalRecordPlantTemplateInput) -> String {
    let name = template.name.trim();
    if !name.is_empty() {
        name.to_string()
    } else {
        format!(
            "{} {}",
            template.manufacturer.trim(),
            template.product.trim()
        )
        .trim()
        .to_string()
    }
}

fn custom_template_name(template: &CustomRecordPlantTemplateInput) -> String {
    format!(
        "{} {}",
        template.manufacturer.trim(),
        template.product.trim()
    )
    .trim()
    .to_string()
}

fn external_template_guide_svg(
    template: &ExternalRecordPlantTemplateInput,
    mode: RecordPlantProofMode,
) -> String {
    let mut svg = String::new();
    let doc = template.document;
    let visible_guides = mode != RecordPlantProofMode::PlantReady;
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{:.3}mm" height="{:.3}mm" viewBox="0 0 {:.3} {:.3}" data-record-plant-template="{}" data-record-plant-proof-mode="{}" data-visible-guides="{}">"#,
        doc.width,
        doc.height,
        doc.width,
        doc.height,
        escape_xml(template.id.trim()),
        proof_mode_name(mode),
        visible_guides
    ));
    let name = external_template_name(template);
    svg.push_str(&format!("<title>{}</title>", escape_xml(&name)));
    svg.push_str(&format!(
        "<desc>{} {} template, {:.3} x {:.3} mm.</desc>",
        escape_xml(template.manufacturer.trim()),
        escape_xml(template.product.trim()),
        doc.width,
        doc.height
    ));
    svg.push_str(r#"<rect x="0" y="0" width="100%" height="100%" fill="white"/>"#);
    if visible_guides {
        for guide in &template.guides {
            svg.push_str(&owned_guide_band_svg(guide));
            svg.push_str(&owned_guide_svg(guide));
        }
        svg.push_str(&proof_aids_svg(
            doc,
            &template.guides,
            &template.requirements,
        ));
    }
    svg.push_str("</svg>");
    svg
}

fn custom_template_requirements(
    template: &CustomRecordPlantTemplateInput,
) -> OwnedPrintRequirements {
    let mut notes =
        vec!["Confirm all user-provided dimensions with the selected record plant before plant-ready export.".to_string()];
    let custom_notes = template.notes.trim();
    if !custom_notes.is_empty() {
        notes.push(custom_notes.to_string());
    }
    OwnedPrintRequirements {
        preferred_output: "print-ready PDF".to_string(),
        accepted_formats: vec!["PDF".to_string()],
        color_modes: vec!["CMYK".to_string()],
        min_raster_ppi: Some(300),
        min_bitmap_ppi: None,
        bleed_mm: None,
        safety_mm: None,
        keep_template_layer_out_of_final: true,
        embed_or_outline_fonts: true,
        pdf_standard: None,
        output_condition_identifier: None,
        notes,
    }
}

fn custom_template_guides(template: &CustomRecordPlantTemplateInput) -> Vec<OwnedGuide> {
    let cx = template.document_width_mm / 2.0;
    let cy = template.document_height_mm / 2.0;
    let mut guides = vec![OwnedGuide {
        id: "document-size".to_string(),
        layer: GuideLayerKind::Trim,
        geometry: GuideGeometry::Rect {
            rect: RectMm::new(
                0.0,
                0.0,
                template.document_width_mm,
                template.document_height_mm,
            ),
        },
    }];
    for (layer, diameter) in [
        (GuideLayerKind::Bleed, template.bleed_diameter_mm),
        (GuideLayerKind::Trim, template.trim_diameter_mm),
        (GuideLayerKind::Safety, template.safety_diameter_mm),
        (GuideLayerKind::Hole, template.center_hole_diameter_mm),
        (GuideLayerKind::Dink, template.dink_diameter_mm),
    ] {
        if let Some(diameter) = diameter {
            guides.push(OwnedGuide {
                id: format!(
                    "{}-diameter-{}mm",
                    guide_layer_name(layer),
                    format_mm_id(diameter)
                ),
                layer,
                geometry: GuideGeometry::Circle {
                    circle: CircleMm::new(cx, cy, diameter / 2.0),
                },
            });
        }
    }
    guides
}

fn custom_template_guide_svg(
    template: &CustomRecordPlantTemplateInput,
    guides: &[OwnedGuide],
    mode: RecordPlantProofMode,
) -> String {
    let mut svg = String::new();
    let visible_guides = mode != RecordPlantProofMode::PlantReady;
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{:.3}mm" height="{:.3}mm" viewBox="0 0 {:.3} {:.3}" data-record-plant-template="{}" data-record-plant-proof-mode="{}" data-visible-guides="{}">"#,
        template.document_width_mm,
        template.document_height_mm,
        template.document_width_mm,
        template.document_height_mm,
        escape_xml(template.id.trim()),
        proof_mode_name(mode),
        visible_guides
    ));
    let name = custom_template_name(template);
    svg.push_str(&format!("<title>{}</title>", escape_xml(&name)));
    svg.push_str(&format!(
        "<desc>{} custom template, {:.3} x {:.3} mm.</desc>",
        escape_xml(&name),
        template.document_width_mm,
        template.document_height_mm
    ));
    svg.push_str(r#"<rect x="0" y="0" width="100%" height="100%" fill="white"/>"#);
    if visible_guides {
        for guide in guides {
            svg.push_str(&owned_guide_band_svg(guide));
            svg.push_str(&owned_guide_svg(guide));
        }
        let requirements = custom_template_requirements(template);
        svg.push_str(&proof_aids_svg(
            RectMm::new(
                0.0,
                0.0,
                template.document_width_mm,
                template.document_height_mm,
            ),
            guides,
            &requirements,
        ));
    }
    svg.push_str("</svg>");
    svg
}

fn external_template_record_context(
    template: &ExternalRecordPlantTemplateInput,
    mode: RecordPlantProofMode,
) -> Option<RecordPlantRecordContext> {
    if !is_record_context_kind(template.kind) {
        return None;
    }
    let diameters = record_context_guide_diameters(template.guides.iter().filter_map(|guide| {
        match guide.geometry {
            GuideGeometry::Circle { circle } => Some((guide.layer, circle.diameter())),
            _ => None,
        }
    }));
    if diameters.is_empty() {
        return None;
    }
    let max_diameter = diameters
        .iter()
        .map(|(_, diameter)| *diameter)
        .fold(0.0_f64, f64::max);
    let name = external_template_name(template);
    let record_profile =
        infer_record_context_profile(template.kind, &template.product, &template.id, max_diameter);
    let record_diameter_mm = record_context_diameter_mm(&record_profile);
    let label_diameter_mm = record_context_label_diameter_mm(&diameters);
    Some(RecordPlantRecordContext {
        record_profile,
        record_diameter_mm,
        label_diameter_mm,
        guide_svg: record_context_guide_svg(
            template.id.trim(),
            &name,
            record_diameter_mm,
            &diameters,
            mode,
        ),
    })
}

fn custom_template_record_context(
    template: &CustomRecordPlantTemplateInput,
    guides: &[OwnedGuide],
    mode: RecordPlantProofMode,
) -> Option<RecordPlantRecordContext> {
    if !is_record_context_kind(template.kind) {
        return None;
    }
    let diameters =
        record_context_guide_diameters(guides.iter().filter_map(|guide| match guide.geometry {
            GuideGeometry::Circle { circle } => Some((guide.layer, circle.diameter())),
            _ => None,
        }));
    if diameters.is_empty() {
        return None;
    }
    let max_diameter = diameters
        .iter()
        .map(|(_, diameter)| *diameter)
        .fold(0.0_f64, f64::max);
    let name = custom_template_name(template);
    let record_profile =
        infer_record_context_profile(template.kind, &template.product, &template.id, max_diameter);
    let record_diameter_mm = record_context_diameter_mm(&record_profile);
    let label_diameter_mm = record_context_label_diameter_mm(&diameters);
    Some(RecordPlantRecordContext {
        record_profile,
        record_diameter_mm,
        label_diameter_mm,
        guide_svg: record_context_guide_svg(
            template.id.trim(),
            &name,
            record_diameter_mm,
            &diameters,
            mode,
        ),
    })
}

fn is_record_context_kind(kind: RecordPlantTemplateKind) -> bool {
    matches!(
        kind,
        RecordPlantTemplateKind::CenterLabel | RecordPlantTemplateKind::PictureLabel
    )
}

fn record_context_guide_diameters<I>(items: I) -> Vec<(GuideLayerKind, f64)>
where
    I: IntoIterator<Item = (GuideLayerKind, f64)>,
{
    let mut diameters: Vec<(GuideLayerKind, f64)> = Vec::new();
    for (layer, diameter) in items {
        if !diameter.is_finite() || diameter <= 0.0 {
            continue;
        }
        if diameters.iter().any(|(existing_layer, existing_diameter)| {
            *existing_layer == layer && (*existing_diameter - diameter).abs() < 0.01
        }) {
            continue;
        }
        diameters.push((layer, diameter));
    }
    diameters.sort_by(
        |(left_layer, left_diameter), (right_layer, right_diameter)| {
            left_layer
                .cmp(right_layer)
                .then_with(|| right_diameter.total_cmp(left_diameter))
        },
    );
    diameters
}

fn infer_record_context_profile(
    kind: RecordPlantTemplateKind,
    product: &str,
    template_id: &str,
    max_diameter_mm: f64,
) -> String {
    let text = format!("{} {}", product, template_id).to_ascii_lowercase();
    if text.contains("7 in")
        || text.contains("7-inch")
        || text.contains("7inch")
        || text.contains("-7-")
        || (kind == RecordPlantTemplateKind::CenterLabel && max_diameter_mm <= 96.0)
    {
        "45".to_string()
    } else {
        "lp".to_string()
    }
}

fn record_context_diameter_mm(record_profile: &str) -> f64 {
    match record_profile {
        "45" => 177.8,
        _ => 304.8,
    }
}

fn record_context_label_diameter_mm(diameters: &[(GuideLayerKind, f64)]) -> Option<f64> {
    for preferred_layer in [
        GuideLayerKind::Trim,
        GuideLayerKind::Bleed,
        GuideLayerKind::Safety,
    ] {
        let diameter = diameters
            .iter()
            .filter_map(|(layer, diameter)| (*layer == preferred_layer).then_some(*diameter))
            .fold(0.0_f64, f64::max);
        if diameter > 0.0 {
            return Some(diameter);
        }
    }
    None
}

fn record_context_guide_svg(
    template_id: &str,
    template_name: &str,
    record_diameter_mm: f64,
    diameters: &[(GuideLayerKind, f64)],
    mode: RecordPlantProofMode,
) -> String {
    let visible_guides = mode != RecordPlantProofMode::PlantReady;
    let center = record_diameter_mm / 2.0;
    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{:.3}mm" height="{:.3}mm" viewBox="0 0 {:.3} {:.3}" data-record-plant-template="{}" data-record-plant-proof-mode="{}" data-visible-guides="{}" data-record-context="true">"#,
        record_diameter_mm,
        record_diameter_mm,
        record_diameter_mm,
        record_diameter_mm,
        escape_xml(template_id),
        proof_mode_name(mode),
        visible_guides
    ));
    svg.push_str(&format!(
        "<title>{} record context</title>",
        escape_xml(template_name)
    ));
    svg.push_str("<desc>Record-context label guides centered on a blank Bitneedle record.</desc>");
    if visible_guides {
        svg.push_str(&record_context_fill_layers(center, diameters));
        for (layer, diameter) in diameters {
            let (stroke, dash) = record_context_layer_style(*layer);
            let layer_name = guide_layer_name(*layer);
            svg.push_str(&format!(
                r#"<circle data-layer="{}-guide-band" data-diameter-mm="{:.3}" cx="{:.3}" cy="{:.3}" r="{:.3}" fill="none" stroke="{}" stroke-width="5" stroke-opacity="0.18" vector-effect="non-scaling-stroke" />"#,
                layer_name,
                diameter,
                center,
                center,
                diameter / 2.0,
                stroke
            ));
            svg.push_str(&format!(
                r#"<circle data-layer="{}" data-diameter-mm="{:.3}" cx="{:.3}" cy="{:.3}" r="{:.3}" fill="none" stroke="{}" stroke-width="1" vector-effect="non-scaling-stroke"{} />"#,
                layer_name,
                diameter,
                center,
                center,
                diameter / 2.0,
                stroke,
                dash
            ));
        }
    }
    svg.push_str("</svg>");
    svg
}

fn record_context_fill_layers(center: f64, diameters: &[(GuideLayerKind, f64)]) -> String {
    let mut svg = String::new();
    let bleed = max_record_context_diameter(diameters, GuideLayerKind::Bleed);
    let trim = max_record_context_diameter(diameters, GuideLayerKind::Trim);
    let safety = max_record_context_diameter(diameters, GuideLayerKind::Safety);
    if let (Some(bleed), Some(trim)) = (bleed, trim) {
        if bleed > trim {
            svg.push_str(&record_context_ring_svg(
                center,
                bleed,
                trim,
                "bleed-band",
                "var(--plant-guide-bleed-fill, rgba(255, 0, 102, 0.34))",
            ));
        }
    } else if let Some(bleed) = bleed {
        svg.push_str(&format!(
            r#"<circle data-layer="bleed-band" cx="{:.3}" cy="{:.3}" r="{:.3}" fill="var(--plant-guide-bleed-fill, rgba(255, 0, 102, 0.18))" />"#,
            center,
            center,
            bleed / 2.0
        ));
    }
    let critical_inner = safety.or_else(|| match (bleed, trim) {
        (Some(bleed), Some(trim)) if bleed > trim && trim > bleed - trim => {
            Some(trim - (bleed - trim))
        }
        _ => None,
    });
    if let (Some(trim), Some(critical_inner)) = (trim, critical_inner) {
        if trim > critical_inner {
            svg.push_str(&record_context_ring_svg(
                center,
                trim,
                critical_inner,
                "critical-print-area",
                "var(--plant-guide-critical-fill, rgba(255, 112, 178, 0.22))",
            ));
        }
    }
    if let Some(hole) = max_record_context_diameter(diameters, GuideLayerKind::Hole) {
        svg.push_str(&format!(
            r#"<circle data-layer="spindle-hole-fill" cx="{:.3}" cy="{:.3}" r="{:.3}" fill="var(--plant-guide-hole-fill, rgba(201, 242, 109, 0.96))" />"#,
            center,
            center,
            hole / 2.0
        ));
    }
    svg
}

fn max_record_context_diameter(
    diameters: &[(GuideLayerKind, f64)],
    layer: GuideLayerKind,
) -> Option<f64> {
    let diameter = diameters
        .iter()
        .filter_map(|(guide_layer, diameter)| (*guide_layer == layer).then_some(*diameter))
        .fold(0.0_f64, f64::max);
    (diameter > 0.0).then_some(diameter)
}

fn record_context_ring_svg(
    center: f64,
    outer_diameter: f64,
    inner_diameter: f64,
    layer: &str,
    fill: &str,
) -> String {
    let outer = outer_diameter / 2.0;
    let inner = inner_diameter / 2.0;
    format!(
        r#"<path data-layer="{}" fill-rule="evenodd" fill="{}" d="M {:.3} {:.3} m -{:.3} 0 a {:.3} {:.3} 0 1 0 {:.3} 0 a {:.3} {:.3} 0 1 0 -{:.3} 0 M {:.3} {:.3} m -{:.3} 0 a {:.3} {:.3} 0 1 1 {:.3} 0 a {:.3} {:.3} 0 1 1 -{:.3} 0" />"#,
        escape_xml(layer),
        fill,
        center,
        center,
        outer,
        outer,
        outer,
        outer * 2.0,
        outer,
        outer,
        outer * 2.0,
        center,
        center,
        inner,
        inner,
        inner,
        inner * 2.0,
        inner,
        inner,
        inner * 2.0
    )
}

fn owned_guide_svg(guide: &OwnedGuide) -> String {
    let (stroke, dash) = layer_style(guide.layer);
    match guide.geometry {
        GuideGeometry::Rect { rect } => format!(
            r#"<rect data-guide-id="{}" data-layer="{:?}" x="{:.3}" y="{:.3}" width="{:.3}" height="{:.3}" fill="none" stroke="{}" stroke-width="1" vector-effect="non-scaling-stroke"{} />"#,
            escape_xml(&guide.id),
            guide.layer,
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            stroke,
            dash
        ),
        GuideGeometry::Circle { circle } => format!(
            r#"<circle data-guide-id="{}" data-layer="{:?}" cx="{:.3}" cy="{:.3}" r="{:.3}" fill="none" stroke="{}" stroke-width="1" vector-effect="non-scaling-stroke"{} />"#,
            escape_xml(&guide.id),
            guide.layer,
            circle.cx,
            circle.cy,
            circle.radius,
            stroke,
            dash
        ),
        GuideGeometry::VerticalLine { x, y1, y2 } => format!(
            r#"<line data-guide-id="{}" data-layer="{:?}" x1="{:.3}" y1="{:.3}" x2="{:.3}" y2="{:.3}" stroke="{}" stroke-width="1" vector-effect="non-scaling-stroke"{} />"#,
            escape_xml(&guide.id),
            guide.layer,
            x,
            y1,
            x,
            y2,
            stroke,
            dash
        ),
        GuideGeometry::HorizontalLine { y, x1, x2 } => format!(
            r#"<line data-guide-id="{}" data-layer="{:?}" x1="{:.3}" y1="{:.3}" x2="{:.3}" y2="{:.3}" stroke="{}" stroke-width="1" vector-effect="non-scaling-stroke"{} />"#,
            escape_xml(&guide.id),
            guide.layer,
            x1,
            y,
            x2,
            y,
            stroke,
            dash
        ),
    }
}

fn owned_guide_band_svg(guide: &OwnedGuide) -> String {
    let (stroke, _) = layer_style(guide.layer);
    match guide.geometry {
        GuideGeometry::Rect { rect } => format!(
            r#"<rect data-guide-band-for="{}" data-layer="{:?}-guide-band" x="{:.3}" y="{:.3}" width="{:.3}" height="{:.3}" fill="none" stroke="{}" stroke-width="5" stroke-opacity="0.18" vector-effect="non-scaling-stroke" />"#,
            escape_xml(&guide.id),
            guide.layer,
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            stroke
        ),
        GuideGeometry::Circle { circle } => format!(
            r#"<circle data-guide-band-for="{}" data-layer="{:?}-guide-band" cx="{:.3}" cy="{:.3}" r="{:.3}" fill="none" stroke="{}" stroke-width="5" stroke-opacity="0.18" vector-effect="non-scaling-stroke" />"#,
            escape_xml(&guide.id),
            guide.layer,
            circle.cx,
            circle.cy,
            circle.radius,
            stroke
        ),
        GuideGeometry::VerticalLine { x, y1, y2 } => format!(
            r#"<line data-guide-band-for="{}" data-layer="{:?}-guide-band" x1="{:.3}" y1="{:.3}" x2="{:.3}" y2="{:.3}" stroke="{}" stroke-width="5" stroke-opacity="0.18" vector-effect="non-scaling-stroke" />"#,
            escape_xml(&guide.id),
            guide.layer,
            x,
            y1,
            x,
            y2,
            stroke
        ),
        GuideGeometry::HorizontalLine { y, x1, x2 } => format!(
            r#"<line data-guide-band-for="{}" data-layer="{:?}-guide-band" x1="{:.3}" y1="{:.3}" x2="{:.3}" y2="{:.3}" stroke="{}" stroke-width="5" stroke-opacity="0.18" vector-effect="non-scaling-stroke" />"#,
            escape_xml(&guide.id),
            guide.layer,
            x1,
            y,
            x2,
            y,
            stroke
        ),
    }
}

fn proof_aids_svg(
    doc: RectMm,
    guides: &[OwnedGuide],
    requirements: &OwnedPrintRequirements,
) -> String {
    if !(doc.width > 0.0 && doc.height > 0.0) {
        return String::new();
    }
    let placement_circles = proof_placement_circles(guides);
    let mut svg = String::new();
    svg.push_str(r#"<g data-layer="proof-aids">"#);
    svg.push_str(&proof_registration_targets_svg(doc));
    svg.push_str(&proof_corner_marks_svg(doc));
    svg.push_str(&proof_process_swatch_svg(doc, guides, requirements));
    svg.push_str(&proof_metrics_svg(doc, guides, requirements));
    for (index, circle) in placement_circles.iter().copied().enumerate() {
        svg.push_str(&proof_slot_label_svg(
            index,
            circle,
            doc,
            &placement_circles,
        ));
        svg.push_str(&proof_alignment_marker_svg(circle.cx, circle.cy, doc));
        svg.push_str(&proof_circle_alignment_ticks_svg(circle, doc));
    }
    svg.push_str("</g>");
    svg
}

fn proof_placement_circles(guides: &[OwnedGuide]) -> Vec<CircleMm> {
    let mut circles: Vec<CircleMm> = guides
        .iter()
        .filter_map(|guide| match (guide.layer, guide.geometry) {
            (GuideLayerKind::Bleed, GuideGeometry::Circle { circle }) => Some(circle),
            _ => None,
        })
        .collect();
    if circles.is_empty() {
        circles = guides
            .iter()
            .filter_map(|guide| match (guide.layer, guide.geometry) {
                (GuideLayerKind::Trim, GuideGeometry::Circle { circle }) => Some(circle),
                _ => None,
            })
            .collect();
    }
    circles
}

fn proof_slot_label_svg(
    index: usize,
    circle: CircleMm,
    doc: RectMm,
    circles: &[CircleMm],
) -> String {
    let label = proof_slot_label(index);
    let min_dimension = doc.width.min(doc.height);
    let font_size = (min_dimension * 0.052).clamp(4.0, 7.5);
    let pad_x = (font_size * 0.44).clamp(1.8, 3.4);
    let pad_y = (font_size * 0.26).clamp(1.0, 2.0);
    let width = font_size + pad_x * 2.0;
    let height = font_size + pad_y * 2.0;
    let gap = (min_dimension * 0.018).clamp(1.8, 3.2);
    let fallback_margin = (min_dimension * 0.035).clamp(2.0, 5.0);
    let (x, y) = proof_slot_label_position(doc, circle, circles, width, height, gap).unwrap_or((
        (circle.cx - circle.radius + fallback_margin)
            .clamp(1.0, (doc.width - width - 1.0).max(1.0)),
        (circle.cy - circle.radius + fallback_margin)
            .clamp(1.0, (doc.height - height - 1.0).max(1.0)),
    ));
    format!(
        concat!(
            r#"<g data-layer="slot-label" data-proof-mark="slot-label" data-slot-label="{label}">"#,
            r##"<rect x="{x:.3}" y="{y:.3}" width="{width:.3}" height="{height:.3}" rx="0.700" fill="#ffffff" fill-opacity="0.86" stroke="#111111" stroke-width="0.18" vector-effect="non-scaling-stroke"/>"##,
            r##"<text x="{text_x:.3}" y="{text_y:.3}" fill="#111111" font-family="Arial, Helvetica, sans-serif" font-size="{font_size:.3}" font-weight="700">{label}</text>"##,
            "</g>"
        ),
        label = label,
        x = x,
        y = y,
        width = width,
        height = height,
        text_x = x + pad_x,
        text_y = y + pad_y + font_size * 0.78,
        font_size = font_size,
    )
}

fn proof_slot_label_position(
    doc: RectMm,
    circle: CircleMm,
    circles: &[CircleMm],
    width: f64,
    height: f64,
    gap: f64,
) -> Option<(f64, f64)> {
    let candidates = [
        (circle.cx + circle.radius + gap, circle.cy - height / 2.0),
        (
            circle.cx - circle.radius - gap - width,
            circle.cy - height / 2.0,
        ),
        (
            circle.cx - width / 2.0,
            circle.cy - circle.radius - gap - height,
        ),
        (circle.cx - width / 2.0, circle.cy + circle.radius + gap),
    ];
    candidates.into_iter().find(|(x, y)| {
        proof_rect_fits(doc, *x, *y, width, height)
            && circles
                .iter()
                .all(|other| !proof_rect_intersects_circle(*x, *y, width, height, *other))
    })
}

fn proof_rect_fits(doc: RectMm, x: f64, y: f64, width: f64, height: f64) -> bool {
    x >= 1.0 && y >= 1.0 && x + width <= doc.width - 1.0 && y + height <= doc.height - 1.0
}

fn proof_rect_intersects_circle(x: f64, y: f64, width: f64, height: f64, circle: CircleMm) -> bool {
    let nearest_x = circle.cx.clamp(x, x + width);
    let nearest_y = circle.cy.clamp(y, y + height);
    let dx = circle.cx - nearest_x;
    let dy = circle.cy - nearest_y;
    (dx * dx + dy * dy) <= circle.radius * circle.radius
}

fn proof_slot_label(index: usize) -> String {
    if index < 26 {
        ((b'A' + index as u8) as char).to_string()
    } else {
        format!("{}", index + 1)
    }
}

fn proof_registration_targets_svg(doc: RectMm) -> String {
    let min_dimension = doc.width.min(doc.height);
    let radius = (min_dimension * 0.012).clamp(1.3, 2.4);
    let arm = radius * 2.7;
    let margin = (min_dimension * 0.032).clamp(3.0, 6.0);
    let points = [
        (margin, margin),
        (doc.width - margin, margin),
        (margin, doc.height - margin),
        (doc.width - margin, doc.height - margin),
        (doc.width / 2.0, doc.height - margin),
    ];
    let mut svg = String::new();
    svg.push_str(r#"<g data-layer="alignment-marker" data-proof-mark="registration-target">"#);
    for (cx, cy) in points {
        svg.push_str(&format!(
            concat!(
                r##"<circle cx="{cx:.3}" cy="{cy:.3}" r="{radius:.3}" fill="#111111" stroke="#ffffff" stroke-width="0.24" vector-effect="non-scaling-stroke"/>"##,
                r##"<circle cx="{cx:.3}" cy="{cy:.3}" r="{outer:.3}" fill="none" stroke="#111111" stroke-width="0.18" vector-effect="non-scaling-stroke"/>"##,
                r##"<path d="M {left:.3} {cy:.3} H {right:.3} M {cx:.3} {top:.3} V {bottom:.3}" fill="none" stroke="#111111" stroke-width="0.18" stroke-linecap="square" vector-effect="non-scaling-stroke"/>"##
            ),
            cx = cx,
            cy = cy,
            radius = radius,
            outer = radius * 1.72,
            left = cx - arm,
            right = cx + arm,
            top = cy - arm,
            bottom = cy + arm,
        ));
    }
    svg.push_str("</g>");
    svg
}

fn proof_corner_marks_svg(doc: RectMm) -> String {
    let margin = (doc.width.min(doc.height) * 0.03).clamp(1.5, 5.0);
    let length = (doc.width.min(doc.height) * 0.075).clamp(4.0, 10.0);
    let left = margin;
    let right = (doc.width - margin).max(left);
    let top = margin;
    let bottom = (doc.height - margin).max(top);
    format!(
        concat!(
            r#"<path data-layer="alignment-marker" d=""#,
            "M {left:.3} {top:.3} h {length:.3} M {left:.3} {top:.3} v {length:.3} ",
            "M {right:.3} {top:.3} h -{length:.3} M {right:.3} {top:.3} v {length:.3} ",
            "M {left:.3} {bottom:.3} h {length:.3} M {left:.3} {bottom:.3} v -{length:.3} ",
            "M {right:.3} {bottom:.3} h -{length:.3} M {right:.3} {bottom:.3} v -{length:.3}",
            r##"" fill="none" stroke="#111111" stroke-width="0.28" stroke-linecap="square" vector-effect="non-scaling-stroke"/>"##,
        ),
        left = left,
        right = right,
        top = top,
        bottom = bottom,
        length = length,
    )
}

fn proof_alignment_marker_svg(cx: f64, cy: f64, doc: RectMm) -> String {
    let size = (doc.width.min(doc.height) * 0.055).clamp(4.0, 8.0);
    let radius = size * 0.28;
    let half = size / 2.0;
    format!(
        concat!(
            r#"<g data-layer="alignment-marker" data-proof-mark="placement-center">"#,
            r##"<circle cx="{cx:.3}" cy="{cy:.3}" r="{radius:.3}" fill="none" stroke="#111111" stroke-width="0.24" vector-effect="non-scaling-stroke"/>"##,
            r##"<path d="M {x1:.3} {cy:.3} H {x2:.3} M {cx:.3} {y1:.3} V {y2:.3}" fill="none" stroke="#111111" stroke-width="0.24" stroke-linecap="square" vector-effect="non-scaling-stroke"/>"##,
            "</g>"
        ),
        cx = cx,
        cy = cy,
        radius = radius,
        x1 = cx - half,
        x2 = cx + half,
        y1 = cy - half,
        y2 = cy + half,
    )
}

fn proof_circle_alignment_ticks_svg(circle: CircleMm, doc: RectMm) -> String {
    let tick = (doc.width.min(doc.height) * 0.035).clamp(2.5, 5.5);
    let inner = (circle.radius - tick * 0.45).max(0.0);
    let outer = circle.radius + tick * 0.55;
    let cx = circle.cx;
    let cy = circle.cy;
    format!(
        concat!(
            r#"<path data-layer="alignment-marker" data-proof-mark="placement-edge" d=""#,
            "M {left_inner:.3} {cy:.3} H {left_outer:.3} ",
            "M {right_inner:.3} {cy:.3} H {right_outer:.3} ",
            "M {cx:.3} {top_inner:.3} V {top_outer:.3} ",
            "M {cx:.3} {bottom_inner:.3} V {bottom_outer:.3}",
            r##"" fill="none" stroke="#111111" stroke-width="0.22" stroke-linecap="square" vector-effect="non-scaling-stroke"/>"##,
        ),
        cx = cx,
        cy = cy,
        left_inner = cx - inner,
        left_outer = cx - outer,
        right_inner = cx + inner,
        right_outer = cx + outer,
        top_inner = cy - inner,
        top_outer = cy - outer,
        bottom_inner = cy + inner,
        bottom_outer = cy + outer,
    )
}

fn proof_process_swatch_svg(
    doc: RectMm,
    guides: &[OwnedGuide],
    requirements: &OwnedPrintRequirements,
) -> String {
    let circles = proof_placement_circles(guides);
    let min_dimension = doc.width.min(doc.height);
    let margin = (min_dimension * 0.035).clamp(1.8, 5.0);
    let gap = (min_dimension * 0.008).clamp(0.6, 1.2);
    let swatch = (min_dimension * 0.042).clamp(3.0, 5.5);
    let label_width = (swatch * 1.8).clamp(4.0, 8.0);
    let mut vertical = true;
    let mut x = (doc.width - margin - swatch - label_width).max(margin);
    let mut y = margin;
    let required_width = swatch + label_width;
    let required_height = swatch * 4.0 + gap * 3.0;

    if !circles.is_empty() {
        let min_left = circles
            .iter()
            .map(|circle| circle.cx - circle.radius)
            .fold(doc.width, f64::min)
            .max(0.0);
        let max_right = circles
            .iter()
            .map(|circle| circle.cx + circle.radius)
            .fold(0.0, f64::max)
            .min(doc.width);
        let min_top = circles
            .iter()
            .map(|circle| circle.cy - circle.radius)
            .fold(doc.height, f64::min)
            .max(0.0);
        let max_bottom = circles
            .iter()
            .map(|circle| circle.cy + circle.radius)
            .fold(0.0, f64::max)
            .min(doc.height);
        let right_gap = doc.width - max_right;
        let left_gap = min_left;
        let top_gap = min_top;
        let bottom_gap = doc.height - max_bottom;
        if right_gap >= required_width + margin {
            x = max_right + ((right_gap - required_width) / 2.0).max(0.0);
            y = margin;
        } else if left_gap >= required_width + margin {
            x = ((left_gap - required_width) / 2.0).max(margin * 0.5);
            y = margin;
        } else if top_gap >= swatch + margin {
            vertical = false;
            x = margin;
            y = ((top_gap - swatch) / 2.0).max(margin * 0.5);
        } else if bottom_gap >= swatch + margin {
            vertical = false;
            x = margin;
            y = max_bottom + ((bottom_gap - swatch) / 2.0).max(0.0);
        }
    }

    let mut svg = String::new();
    svg.push_str(r#"<g data-layer="process-swatch" data-proof-mark="cmyk-swatch">"#);
    for (index, (label, fill)) in [
        ("C", "#00AEEF"),
        ("M", "#EC008C"),
        ("Y", "#FFF200"),
        ("K", "#111111"),
    ]
    .iter()
    .enumerate()
    {
        let offset = index as f64 * (swatch + gap);
        let swatch_x = if vertical { x } else { x + offset };
        let swatch_y = if vertical { y + offset } else { y };
        svg.push_str(&format!(
            r##"<rect x="{:.3}" y="{:.3}" width="{:.3}" height="{:.3}" fill="{}" stroke="#111111" stroke-width="0.18" vector-effect="non-scaling-stroke"/>"##,
            swatch_x, swatch_y, swatch, swatch, fill
        ));
        let (label_x, label_y) = if vertical {
            (swatch_x + swatch + 1.1, swatch_y + swatch * 0.76)
        } else {
            (swatch_x + swatch * 0.36, swatch_y + swatch + 2.4)
        };
        svg.push_str(&format!(
            r##"<text x="{:.3}" y="{:.3}" fill="#111111" font-family="Arial, Helvetica, sans-serif" font-size="{:.3}" font-weight="700">{}</text>"##,
            label_x,
            label_y,
            (swatch * 0.58).clamp(1.8, 3.0),
            label
        ));
    }
    let output_condition = requirements
        .output_condition_identifier
        .as_deref()
        .unwrap_or("")
        .trim();
    if !output_condition.is_empty() {
        let label_y = if vertical {
            y + required_height + swatch * 0.82
        } else {
            y + swatch * 2.2
        };
        svg.push_str(&format!(
            r##"<text x="{:.3}" y="{:.3}" fill="#111111" font-family="Arial, Helvetica, sans-serif" font-size="{:.3}" font-weight="700">{}</text>"##,
            x,
            label_y,
            (swatch * 0.48).clamp(1.6, 2.8),
            escape_xml(output_condition)
        ));
    }
    svg.push_str("</g>");
    svg
}

fn proof_metrics_svg(
    doc: RectMm,
    guides: &[OwnedGuide],
    requirements: &OwnedPrintRequirements,
) -> String {
    let min_dimension = doc.width.min(doc.height);
    let font_size = (min_dimension * 0.017).clamp(1.8, 2.5);
    let line_height = font_size * 1.28;
    let rows = proof_metric_rows(guides, requirements);
    if rows.is_empty() {
        return String::new();
    }
    let panel_width = (min_dimension * 0.19).clamp(18.0, 34.0);
    let panel_height = rows.len() as f64 * line_height + font_size * 0.95;
    let margin = (min_dimension * 0.035).clamp(3.0, 5.0);
    let x = margin;
    let y = (doc.height - margin - panel_height).max(margin);
    let mut svg = String::new();
    svg.push_str(r#"<g data-layer="proof-metrics">"#);
    svg.push_str(&format!(
        r##"<rect x="{:.3}" y="{:.3}" width="{:.3}" height="{:.3}" fill="#ffffff" fill-opacity="0.78" stroke="#111111" stroke-width="0.14" vector-effect="non-scaling-stroke"/>"##,
        x - font_size * 0.45,
        y - font_size * 0.55,
        panel_width,
        panel_height,
    ));
    for (index, row) in rows.iter().enumerate() {
        svg.push_str(&format!(
            r##"<text x="{:.3}" y="{:.3}" fill="#111111" font-family="Arial, Helvetica, sans-serif" font-size="{:.3}" font-weight="700">{}</text>"##,
            x,
            y + index as f64 * line_height + font_size,
            font_size,
            escape_xml(row),
        ));
    }
    svg.push_str("</g>");
    svg
}

fn proof_metric_rows(guides: &[OwnedGuide], requirements: &OwnedPrintRequirements) -> Vec<String> {
    let mut rows = Vec::new();
    if let Some(value) = max_circle_diameter(guides, GuideLayerKind::Bleed) {
        rows.push(format!("Artwork edge: {}", format_display_mm(value)));
    }
    if let Some(value) = max_circle_diameter(guides, GuideLayerKind::Trim) {
        rows.push(format!("Final trim: {}", format_display_mm(value)));
    }
    if let Some(value) = max_circle_diameter(guides, GuideLayerKind::Safety) {
        rows.push(format!("No-type area: {}", format_display_mm(value)));
    }
    if let Some(value) = max_circle_diameter(guides, GuideLayerKind::Hole) {
        rows.push(format!("Center hole: {}", format_display_mm(value)));
    }
    if let Some(ppi) = requirements.min_raster_ppi {
        rows.push(format!("Resolution: {ppi} DPI min"));
    }
    if let Some(output_condition) = requirements
        .output_condition_identifier
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        rows.push(format!("Profile: {output_condition}"));
    }
    rows
}

fn max_circle_diameter(guides: &[OwnedGuide], layer: GuideLayerKind) -> Option<f64> {
    guides
        .iter()
        .filter_map(|guide| match (guide.layer, guide.geometry) {
            (guide_layer, GuideGeometry::Circle { circle }) if guide_layer == layer => {
                Some(circle.diameter())
            }
            _ => None,
        })
        .max_by(f64::total_cmp)
}

fn format_display_mm(value: f64) -> String {
    let rounded = (value * 10.0).round() / 10.0;
    if (rounded - rounded.round()).abs() < 0.01 {
        format!("{rounded:.0} mm")
    } else {
        format!("{rounded:.1} mm")
    }
}

fn custom_preflight_checks(mode: RecordPlantProofMode) -> Vec<RecordPlantPreflightCheck> {
    vec![
        RecordPlantPreflightCheck {
            id: "custom-template",
            status: if mode == RecordPlantProofMode::PlantReady {
                RecordPlantPreflightStatus::Warn
            } else {
                RecordPlantPreflightStatus::Pass
            },
            summary: "User template is selected",
            detail: "Dimensions are saved locally and should be confirmed with the manufacturer before final submission.",
        },
        RecordPlantPreflightCheck {
            id: "guide-layer-policy",
            status: RecordPlantPreflightStatus::Pass,
            summary: "Guide layer policy is explicit",
            detail: if mode == RecordPlantProofMode::PlantReady {
                "Plant-ready output omits visible guides."
            } else {
                "Proof output includes visible guides."
            },
        },
        RecordPlantPreflightCheck {
            id: "artwork-preflight-required",
            status: RecordPlantPreflightStatus::Warn,
            summary: "Artwork-specific preflight still required",
            detail: "Placed artwork must still be checked for bleed fill, safe margins, raster resolution, color mode, and fonts.",
        },
    ]
}

fn layer_style(layer: GuideLayerKind) -> (&'static str, &'static str) {
    const SHORT_DASH_BOUNDARY: &str = r#" stroke-dasharray="3px 2px" stroke-linecap="butt""#;
    match layer {
        GuideLayerKind::Bleed => ("#d00000", SHORT_DASH_BOUNDARY),
        GuideLayerKind::Trim => ("#111111", SHORT_DASH_BOUNDARY),
        GuideLayerKind::Safety => ("#0057b8", SHORT_DASH_BOUNDARY),
        GuideLayerKind::Fold => ("#168a3a", SHORT_DASH_BOUNDARY),
        GuideLayerKind::Spine => ("#6b7280", ""),
        GuideLayerKind::Hole => ("#7c3aed", SHORT_DASH_BOUNDARY),
        GuideLayerKind::Dink => ("#00a3ff", SHORT_DASH_BOUNDARY),
    }
}

fn record_context_layer_style(layer: GuideLayerKind) -> (&'static str, &'static str) {
    const SHORT_DASH_BOUNDARY: &str = r#" stroke-dasharray="3px 2px" stroke-linecap="butt""#;
    match layer {
        GuideLayerKind::Bleed => ("var(--plant-guide-bleed, #f00078)", SHORT_DASH_BOUNDARY),
        GuideLayerKind::Trim => ("var(--plant-guide-trim, #00b7ff)", SHORT_DASH_BOUNDARY),
        GuideLayerKind::Safety => ("var(--plant-guide-critical, #f27ab0)", SHORT_DASH_BOUNDARY),
        GuideLayerKind::Fold => ("var(--plant-guide-fold, #2fe36e)", SHORT_DASH_BOUNDARY),
        GuideLayerKind::Spine => ("var(--plant-guide-spine, #d1d5db)", ""),
        GuideLayerKind::Hole => ("var(--plant-guide-hole, #111111)", SHORT_DASH_BOUNDARY),
        GuideLayerKind::Dink => ("var(--plant-guide-dink, #00d4ff)", SHORT_DASH_BOUNDARY),
    }
}

fn guide_layer_name(layer: GuideLayerKind) -> &'static str {
    match layer {
        GuideLayerKind::Bleed => "bleed",
        GuideLayerKind::Trim => "trim",
        GuideLayerKind::Safety => "safety",
        GuideLayerKind::Fold => "fold",
        GuideLayerKind::Spine => "spine",
        GuideLayerKind::Hole => "hole",
        GuideLayerKind::Dink => "dink",
    }
}

fn format_mm_id(value: f64) -> String {
    let formatted = if (value.fract()).abs() < 0.0001 {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    };
    formatted.replace('.', "p")
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn proof_mode_name(mode: RecordPlantProofMode) -> &'static str {
    match mode {
        RecordPlantProofMode::Proof => "proof",
        RecordPlantProofMode::PlantReady => "plant-ready",
    }
}

fn proof_artifacts(mode: RecordPlantProofMode) -> Vec<RecordPlantArtifact> {
    match mode {
        RecordPlantProofMode::Proof => vec![
            RecordPlantArtifact {
                path: "proof.svg",
                kind: RecordPlantArtifactKind::Proof,
                status: RecordPlantArtifactStatus::Implemented,
                visible_guides: true,
            },
            RecordPlantArtifact {
                path: "record-plant-spec.json",
                kind: RecordPlantArtifactKind::Spec,
                status: RecordPlantArtifactStatus::Implemented,
                visible_guides: false,
            },
            RecordPlantArtifact {
                path: "preflight.md",
                kind: RecordPlantArtifactKind::Preflight,
                status: RecordPlantArtifactStatus::Planned,
                visible_guides: false,
            },
            RecordPlantArtifact {
                path: "README-for-plant.md",
                kind: RecordPlantArtifactKind::Readme,
                status: RecordPlantArtifactStatus::Planned,
                visible_guides: false,
            },
        ],
        RecordPlantProofMode::PlantReady => vec![
            RecordPlantArtifact {
                path: "plant-ready.pdf",
                kind: RecordPlantArtifactKind::PlantReadyArtwork,
                status: RecordPlantArtifactStatus::Planned,
                visible_guides: false,
            },
            RecordPlantArtifact {
                path: "record-plant-spec.json",
                kind: RecordPlantArtifactKind::Spec,
                status: RecordPlantArtifactStatus::Implemented,
                visible_guides: false,
            },
            RecordPlantArtifact {
                path: "preflight.md",
                kind: RecordPlantArtifactKind::Preflight,
                status: RecordPlantArtifactStatus::Planned,
                visible_guides: false,
            },
            RecordPlantArtifact {
                path: "README-for-plant.md",
                kind: RecordPlantArtifactKind::Readme,
                status: RecordPlantArtifactStatus::Planned,
                visible_guides: false,
            },
        ],
    }
}

fn confidence_preflight_status(
    confidence: MeasurementConfidence,
    mode: RecordPlantProofMode,
) -> RecordPlantPreflightStatus {
    match (confidence, mode) {
        (MeasurementConfidence::PlantPublished, _) => RecordPlantPreflightStatus::Pass,
        (MeasurementConfidence::DerivedFromPlantTemplate, RecordPlantProofMode::PlantReady) => {
            RecordPlantPreflightStatus::Warn
        }
        (MeasurementConfidence::DerivedFromPlantTemplate, _) => RecordPlantPreflightStatus::Pass,
        (MeasurementConfidence::NeedsPlantConfirmation, RecordPlantProofMode::PlantReady) => {
            RecordPlantPreflightStatus::Fail
        }
        (MeasurementConfidence::NeedsPlantConfirmation, _) => RecordPlantPreflightStatus::Warn,
    }
}

fn external_preflight_checks(
    template: &ExternalRecordPlantTemplateInput,
    mode: RecordPlantProofMode,
) -> Vec<RecordPlantPreflightCheck> {
    template_preflight_checks(
        template.confidence,
        template.requirements.keep_template_layer_out_of_final,
        mode,
    )
}

fn template_preflight_checks(
    confidence: MeasurementConfidence,
    keep_template_layer_out_of_final: bool,
    mode: RecordPlantProofMode,
) -> Vec<RecordPlantPreflightCheck> {
    vec![
        RecordPlantPreflightCheck {
            id: "template-selected",
            status: RecordPlantPreflightStatus::Pass,
            summary: "Template is selected",
            detail: "The manifest names a stable record-plant template ID and version/source reference.",
        },
        RecordPlantPreflightCheck {
            id: "template-confidence",
            status: confidence_preflight_status(confidence, mode),
            summary: "Template confidence is recorded",
            detail: match confidence {
                MeasurementConfidence::PlantPublished => {
                    "The geometry used here is recorded as plant-published source data."
                }
                MeasurementConfidence::DerivedFromPlantTemplate => {
                    "The geometry is derived from a public plant template and should be manually confirmed before fully automated plant-ready submission."
                }
                MeasurementConfidence::NeedsPlantConfirmation => {
                    "The geometry still needs direct plant confirmation before plant-ready export."
                }
            },
        },
        RecordPlantPreflightCheck {
            id: "guide-layer-policy",
            status: if mode == RecordPlantProofMode::PlantReady && keep_template_layer_out_of_final {
                RecordPlantPreflightStatus::Pass
            } else if mode == RecordPlantProofMode::PlantReady {
                RecordPlantPreflightStatus::Warn
            } else {
                RecordPlantPreflightStatus::Pass
            },
            summary: "Guide layer policy is explicit",
            detail: if mode == RecordPlantProofMode::PlantReady {
                "Plant-ready output must omit visible guide/template layers unless a plant explicitly asks for non-printing dielines."
            } else {
                "Proof output may include visible guides for review and design alignment."
            },
        },
        RecordPlantPreflightCheck {
            id: "artwork-preflight-required",
            status: RecordPlantPreflightStatus::Warn,
            summary: "Artwork-specific preflight still required",
            detail: "Placed artwork must still be checked for bleed fill, safe margins, raster resolution, color mode, font embedding/outlining, and final PDF standard.",
        },
    ]
}

/// A plant in the registry.
///
/// Borrowed rather than owned throughout: the registry is compiled in, so
/// nothing needs to be allocated to read it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlantManufacturer {
    pub id: &'static str,
    pub name: &'static str,
    pub country_code: Option<&'static str>,
    pub website_url: Option<&'static str>,
    pub contact_email: Option<&'static str>,
    pub contact_url: Option<&'static str>,
}

/// One guide on a template: where a plant trims, bleeds, or punches.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlantGuide {
    pub id: &'static str,
    pub layer: GuideLayerKind,
    pub geometry: GuideGeometry,
}

/// What a plant requires of the artwork it is sent.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlantPrintRequirements {
    pub preferred_output: &'static str,
    pub accepted_formats: &'static [&'static str],
    pub color_modes: &'static [&'static str],
    pub min_raster_ppi: Option<u16>,
    pub min_bitmap_ppi: Option<u16>,
    pub bleed_mm: Option<f64>,
    pub safety_mm: Option<f64>,
    pub keep_template_layer_out_of_final: bool,
    /// `None` where the plant publishes no font requirement. That is not the
    /// same as permitting unembedded fonts — it means nobody has confirmed it.
    pub embed_or_outline_fonts: Option<bool>,
    pub pdf_standard: Option<&'static str>,
    // Only a handful of plants publish an output condition; the rest omit the
    // field entirely rather than carrying an explicit null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_condition_identifier: Option<&'static str>,
    pub notes: &'static [&'static str],
}

/// Where a template's measurements came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlantSourceReference {
    pub title: &'static str,
    pub url: &'static str,
    pub retrieved_on: &'static str,
}

/// A plant template, as compiled into this crate.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlantTemplate {
    pub id: &'static str,
    pub name: &'static str,
    pub manufacturer_id: &'static str,
    pub manufacturer: &'static str,
    pub product: &'static str,
    pub kind: RecordPlantTemplateKind,
    pub version: Option<&'static str>,
    pub document: RectMm,
    pub confidence: MeasurementConfidence,
    pub guides: &'static [PlantGuide],
    pub requirements: PlantPrintRequirements,
    pub source: PlantSourceReference,
    pub source_notes: &'static [&'static str],
}

impl PlantTemplate {
    /// The diameter of the area this template's artwork has to fill.
    ///
    /// Bleed wins over trim because bleed is what the artwork must cover, and
    /// a template's trim guide may describe the source page rather than the
    /// label. Only circles count for the same reason.
    pub fn artwork_diameter_mm(&self) -> Option<f64> {
        let mut trim: Option<f64> = None;
        let mut bleed: Option<f64> = None;
        for guide in self.guides {
            let GuideGeometry::Circle { circle } = guide.geometry else {
                continue;
            };
            let diameter = circle.diameter();
            match guide.layer {
                GuideLayerKind::Bleed => {
                    bleed = Some(bleed.map_or(diameter, |value: f64| value.max(diameter)))
                }
                GuideLayerKind::Trim => {
                    trim = Some(trim.map_or(diameter, |value: f64| value.max(diameter)))
                }
                _ => {}
            }
        }
        bleed.or(trim)
    }

    /// True for the label templates a record's centre label is proofed against.
    pub fn is_label(&self) -> bool {
        matches!(
            self.kind,
            RecordPlantTemplateKind::CenterLabel | RecordPlantTemplateKind::PictureLabel
        )
    }

    /// The plant that publishes this template.
    pub fn manufacturer_profile(&self) -> Option<&'static PlantManufacturer> {
        PLANT_MANUFACTURERS
            .iter()
            .find(|manufacturer| manufacturer.id == self.manufacturer_id)
    }
}

/// Looks a template up by id.
pub fn plant_template(id: &str) -> Option<&'static PlantTemplate> {
    PLANT_TEMPLATES.iter().find(|template| template.id == id)
}

/// Looks a plant up by id.
pub fn plant_manufacturer(id: &str) -> Option<&'static PlantManufacturer> {
    PLANT_MANUFACTURERS
        .iter()
        .find(|manufacturer| manufacturer.id == id)
}

/// The registry schema version the apps expect.
pub const REGISTRY_SCHEMA_VERSION: u32 = 1;

/// The lowercase text a supplier search matches against.
///
/// It covers the plant and everything pressable from it, so typing a format or
/// a product name finds the supplier as well as the template.
pub fn plant_search_text(manufacturer: &PlantManufacturer) -> String {
    let mut parts: Vec<&str> = vec![manufacturer.id, manufacturer.name];
    parts.extend(
        [
            manufacturer.country_code,
            manufacturer.website_url,
            manufacturer.contact_email,
            manufacturer.contact_url,
        ]
        .into_iter()
        .flatten(),
    );
    for template in PLANT_TEMPLATES
        .iter()
        .filter(|template| template.manufacturer_id == manufacturer.id)
    {
        parts.extend([template.id, template.name, template.product]);
        parts.push(template_kind_name(template.kind));
        if let Some(version) = template.version {
            parts.push(version);
        }
        parts.push(confidence_name(template.confidence));
        parts.push(template.requirements.preferred_output);
        if let Some(identifier) = template.requirements.output_condition_identifier {
            parts.push(identifier);
        }
        parts.extend(template.requirements.accepted_formats.iter().copied());
        parts.extend(template.requirements.color_modes.iter().copied());
    }
    parts
        .into_iter()
        .filter(|part| !part.trim().is_empty())
        .map(|part| part.to_lowercase().split_whitespace().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join(" ")
}

/// The plants in the order an app lists them.
///
/// Case is folded first so "CD Unity" files under C-D rather than ahead of
/// "Cascade", which is where a raw byte comparison would put it.
pub fn plant_manufacturers_sorted() -> Vec<&'static PlantManufacturer> {
    let mut manufacturers: Vec<&'static PlantManufacturer> = PLANT_MANUFACTURERS.iter().collect();
    manufacturers.sort_by(|left, right| {
        (left.name.to_lowercase(), left.name).cmp(&(right.name.to_lowercase(), right.name))
    });
    manufacturers
}

/// Builds the registry the Plant apps consume.
///
/// The registry keeps plants and templates apart. An app resolving a
/// template's plant on every row would repeat that join in each renderer, so
/// this inlines the plant on the template and precomputes the search text.
pub fn hydrated_registry_json(generated_at: &str) -> Result<String, RecordPlantError> {
    let profiles: Vec<serde_json::Value> = plant_manufacturers_sorted()
        .into_iter()
        .map(|manufacturer| {
            let mut value = serde_json::to_value(manufacturer)
                .map_err(|error| RecordPlantError::Serialize(error.to_string()))?;
            if let Some(object) = value.as_object_mut() {
                object.insert(
                    "searchText".to_string(),
                    serde_json::Value::String(plant_search_text(manufacturer)),
                );
            }
            Ok(value)
        })
        .collect::<Result<_, RecordPlantError>>()?;

    let profile_by_id: BTreeMap<&str, &serde_json::Value> = PLANT_MANUFACTURERS
        .iter()
        .filter_map(|manufacturer| {
            profiles
                .iter()
                .find(|profile| profile.get("id").and_then(|id| id.as_str()) == Some(manufacturer.id))
                .map(|profile| (manufacturer.id, profile))
        })
        .collect();

    let mut templates = Vec::with_capacity(PLANT_TEMPLATES.len());
    for template in PLANT_TEMPLATES {
        let Some(profile) = profile_by_id.get(template.manufacturer_id) else {
            return Err(RecordPlantError::InvalidCustomTemplate(format!(
                "template {} references unknown plant {}",
                template.id, template.manufacturer_id
            )));
        };
        let mut value = serde_json::to_value(template)
            .map_err(|error| RecordPlantError::Serialize(error.to_string()))?;
        if let Some(object) = value.as_object_mut() {
            object.insert("manufacturerProfile".to_string(), (*profile).clone());
        }
        templates.push(value);
    }

    let hydrated = serde_json::json!({
        "schemaVersion": REGISTRY_SCHEMA_VERSION,
        "generatedAt": generated_at,
        "manufacturers": profiles,
        "templates": templates,
    });
    serde_json::to_string(&hydrated)
        .map_err(|error| RecordPlantError::Serialize(error.to_string()))
}

fn template_kind_name(kind: RecordPlantTemplateKind) -> &'static str {
    match kind {
        RecordPlantTemplateKind::CenterLabel => "center-label",
        RecordPlantTemplateKind::PictureLabel => "picture-label",
        RecordPlantTemplateKind::OuterSleeve => "outer-sleeve",
        RecordPlantTemplateKind::InnerSleeve => "inner-sleeve",
        RecordPlantTemplateKind::GatefoldSleeve => "gatefold-sleeve",
        RecordPlantTemplateKind::Insert => "insert",
        RecordPlantTemplateKind::PackagingGuide => "packaging-guide",
    }
}

fn confidence_name(confidence: MeasurementConfidence) -> &'static str {
    match confidence {
        MeasurementConfidence::PlantPublished => "plant-published",
        MeasurementConfidence::DerivedFromPlantTemplate => "derived-from-plant-template",
        MeasurementConfidence::NeedsPlantConfirmation => "needs-plant-confirmation",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXTERNAL_7_LABEL_JSON: &str = r#"{
        "id":"fixture-7-label",
        "name":"7 in label",
        "manufacturer":"Fixture Plant",
        "product":"7 in label",
        "kind":"center-label",
        "version":"test",
        "document":{"x":0,"y":0,"width":100,"height":100},
        "confidence":"derived-from-plant-template",
        "guides":[
            {"id":"bleed-diameter-96mm","layer":"bleed","geometry":{"shape":"circle","circle":{"cx":50,"cy":50,"radius":48}}},
            {"id":"trim-diameter-90mm","layer":"trim","geometry":{"shape":"circle","circle":{"cx":50,"cy":50,"radius":45}}},
            {"id":"dink-diameter-38mm","layer":"dink","geometry":{"shape":"circle","circle":{"cx":50,"cy":50,"radius":19}}},
            {"id":"spindle-hole-diameter-7p5mm","layer":"hole","geometry":{"shape":"circle","circle":{"cx":50,"cy":50,"radius":3.75}}}
        ],
        "requirements":{
            "preferredOutput":"PDF",
            "acceptedFormats":["PDF"],
            "colorModes":["CMYK"],
            "minRasterPpi":300,
            "minBitmapPpi":null,
            "bleedMm":3,
            "safetyMm":null,
            "keepTemplateLayerOutOfFinal":true,
            "embedOrOutlineFonts":true,
            "pdfStandard":null,
            "notes":["fixture only"]
        },
        "source":{"title":"Fixture template","url":"https://example.invalid/template.pdf","retrievedOn":"2026-06-04"},
        "sourceNotes":["Fixture source"]
    }"#;

    #[test]
    fn external_template_bundle_renders_supplied_registry_shape() {
        let bundle =
            record_plant_template_proof_bundle_json(EXTERNAL_7_LABEL_JSON, "proof").unwrap();

        assert!(bundle.contains(r#""templateId":"fixture-7-label""#));
        assert!(bundle.contains(r#""recordProfile":"45""#));
        assert!(bundle.contains(r#"data-record-context=\"true\""#));
        assert!(bundle.contains(r#"data-diameter-mm=\"96.000\""#));
        assert!(bundle.contains(r#"data-diameter-mm=\"38.000\""#));
        assert!(bundle.contains(r#"data-diameter-mm=\"7.500\""#));
        assert!(bundle.contains(r#"data-layer=\"process-swatch\""#));
        assert!(bundle.contains(r#"data-layer=\"alignment-marker\""#));
        assert!(bundle.contains(r#"data-layer=\"slot-label\""#));
    }

    #[test]
    fn plant_ready_bundle_omits_visible_guides() {
        let bundle =
            record_plant_template_proof_bundle_json(EXTERNAL_7_LABEL_JSON, "plant-ready").unwrap();

        assert!(bundle.contains(r#"data-record-plant-proof-mode=\"plant-ready\""#));
        assert!(bundle.contains(r#"data-visible-guides=\"false\""#));
        assert!(!bundle.contains("data-guide-id"));
        assert!(!bundle.contains("process-swatch"));
        assert!(!bundle.contains("alignment-marker"));
    }

    #[test]
    fn plant_ready_bundle_warns_for_derived_templates() {
        let bundle =
            record_plant_template_proof_bundle_json(EXTERNAL_7_LABEL_JSON, "plant-ready").unwrap();

        assert!(bundle.contains(r#""path":"plant-ready.pdf""#));
        assert!(bundle.contains(r#""id":"template-confidence""#));
        assert!(bundle.contains(r#""status":"warn""#));
    }

    #[test]
    fn invalid_external_template_is_rejected() {
        let error = record_plant_template_proof_bundle_json(
            r#"{"id":"","manufacturer":"","product":"","kind":"center-label"}"#,
            "proof",
        )
        .unwrap_err();

        assert!(matches!(error, RecordPlantError::InvalidCustomTemplate(_)));
    }

    #[test]
    fn custom_template_bundle_includes_record_context_when_label_dimensions_are_present() {
        let json = custom_record_plant_template_proof_bundle_json(
            r#"{
                "id":"custom-test-7-label",
                "manufacturer":"Test Plant",
                "product":"7 in label",
                "kind":"center-label",
                "documentWidthMm":100,
                "documentHeightMm":100,
                "bleedDiameterMm":96,
                "trimDiameterMm":90,
                "centerHoleDiameterMm":7.2
            }"#,
            "proof",
        )
        .unwrap();
        assert!(json.contains(r#""manufacturer":"Test Plant""#));
        assert!(json.contains(r#""recordProfile":"45""#));
        assert!(json.contains(r#""guideSvg":"#));
    }

    #[test]
    fn every_template_belongs_to_a_plant_in_the_registry() {
        assert!(!PLANT_MANUFACTURERS.is_empty());
        assert!(!PLANT_TEMPLATES.is_empty());
        for template in PLANT_TEMPLATES {
            assert!(
                template.manufacturer_profile().is_some(),
                "template {} references a plant that is not in the registry",
                template.id
            );
            assert_eq!(
                template.manufacturer_profile().unwrap().name,
                template.manufacturer,
                "template {} disagrees with its plant's name",
                template.id
            );
        }
    }

    #[test]
    fn registry_ids_are_unique() {
        // A duplicate id silently shadows a template in every lookup.
        let mut template_ids: Vec<&str> = PLANT_TEMPLATES.iter().map(|t| t.id).collect();
        template_ids.sort_unstable();
        let count = template_ids.len();
        template_ids.dedup();
        assert_eq!(template_ids.len(), count, "duplicate template id");

        let mut plant_ids: Vec<&str> = PLANT_MANUFACTURERS.iter().map(|m| m.id).collect();
        plant_ids.sort_unstable();
        let count = plant_ids.len();
        plant_ids.dedup();
        assert_eq!(plant_ids.len(), count, "duplicate plant id");
    }

    #[test]
    fn label_templates_publish_an_artwork_diameter() {
        // Choosing a template for a proof is a diameter match, so a label with
        // no measurable artwork area can never be selected.
        //
        // The two label kinds are different objects: a centre label is the
        // paper in the middle, while a picture label prints the whole disc
        // face, so its artwork is record-sized.
        for template in PLANT_TEMPLATES.iter().filter(|t| t.is_label()) {
            let diameter = template.artwork_diameter_mm();
            let plausible = match template.kind {
                RecordPlantTemplateKind::CenterLabel => 40.0..=120.0,
                _ => 40.0..=310.0,
            };
            assert!(
                diameter.is_some_and(|value| plausible.contains(&value)),
                "label {} has an implausible artwork diameter: {diameter:?}",
                template.id
            );
        }
    }

    #[test]
    fn a_picture_label_is_never_chosen_for_a_centre_label_proof() {
        // A picture label covers the whole disc face. Proofing a 98.4 mm
        // centre label against one would scale the artwork down by a third.
        let picture_labels: Vec<&PlantTemplate> = PLANT_TEMPLATES
            .iter()
            .filter(|t| t.kind == RecordPlantTemplateKind::PictureLabel)
            .collect();
        assert!(!picture_labels.is_empty());
        for template in picture_labels {
            let diameter = template.artwork_diameter_mm().unwrap();
            assert!(
                diameter > 120.0,
                "{} is picture-label sized but measures {diameter} mm",
                template.id
            );
        }
    }

    #[test]
    fn a_plant_that_publishes_no_font_rule_is_not_recorded_as_permitting_loose_fonts() {
        // Three plants publish no font requirement. Unknown has to stay
        // unknown; false would tell a designer they may ship live text.
        let unknown = PLANT_TEMPLATES
            .iter()
            .filter(|t| t.requirements.embed_or_outline_fonts.is_none())
            .count();
        assert!(unknown > 0);
        assert!(PLANT_TEMPLATES
            .iter()
            .any(|t| t.requirements.embed_or_outline_fonts == Some(true)));
    }

    #[test]
    fn lookups_find_templates_and_plants_by_id() {
        let template = PLANT_TEMPLATES[0];
        assert_eq!(plant_template(template.id).unwrap().id, template.id);
        assert!(plant_template("no-such-template").is_none());
        assert_eq!(
            plant_manufacturer(template.manufacturer_id).unwrap().id,
            template.manufacturer_id
        );
        assert!(plant_manufacturer("no-such-plant").is_none());
    }

    #[test]
    fn hydrated_registry_inlines_every_template_manufacturer_profile() {
        let hydrated: serde_json::Value =
            serde_json::from_str(&hydrated_registry_json("2026-01-01T00:00:00.000Z").unwrap())
                .unwrap();
        assert_eq!(hydrated["generatedAt"], "2026-01-01T00:00:00.000Z");
        assert_eq!(hydrated["schemaVersion"], REGISTRY_SCHEMA_VERSION);

        let templates = hydrated["templates"].as_array().unwrap();
        assert_eq!(templates.len(), PLANT_TEMPLATES.len());
        for template in templates {
            let profile = &template["manufacturerProfile"];
            assert_eq!(
                profile["id"], template["manufacturerId"],
                "template {} carries the wrong plant profile",
                template["id"]
            );
            assert_eq!(profile["name"], template["manufacturer"]);
            assert!(profile["searchText"].as_str().unwrap().contains(
                &template["id"].as_str().unwrap().to_lowercase()
            ));
        }
    }

    #[test]
    fn hydrated_manufacturers_are_sorted_for_stable_supplier_lists() {
        let hydrated: serde_json::Value =
            serde_json::from_str(&hydrated_registry_json("").unwrap()).unwrap();
        let names: Vec<&str> = hydrated["manufacturers"]
            .as_array()
            .unwrap()
            .iter()
            .map(|manufacturer| manufacturer["name"].as_str().unwrap())
            .collect();
        let mut sorted = names.clone();
        sorted.sort_by_key(|name| (name.to_lowercase(), name.to_string()));
        assert_eq!(names, sorted);
        // The web Plant app files this plant under C-D, so the native one must
        // too or the same registry reads as two different supplier lists.
        let cd_unity = names.iter().position(|name| *name == "CD Unity");
        let cascade = names
            .iter()
            .position(|name| *name == "Cascade Record Pressing");
        if let (Some(cd_unity), Some(cascade)) = (cd_unity, cascade) {
            assert!(cascade < cd_unity);
        }
    }

    #[test]
    fn hydrated_search_text_drops_absent_contact_fields() {
        let hydrated: serde_json::Value =
            serde_json::from_str(&hydrated_registry_json("").unwrap()).unwrap();
        for manufacturer in hydrated["manufacturers"].as_array().unwrap() {
            let search_text = manufacturer["searchText"].as_str().unwrap();
            assert!(
                !search_text.contains("null"),
                "{} leaked an absent field into its search text",
                manufacturer["id"]
            );
            assert_eq!(search_text, search_text.to_lowercase());
        }
    }
}
