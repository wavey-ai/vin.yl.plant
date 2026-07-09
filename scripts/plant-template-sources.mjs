#!/usr/bin/env node
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const recordPlantRoot = path.resolve(scriptDir, "..");
const repoRoot = path.resolve(recordPlantRoot, "..");

const defaults = Object.freeze({
  fixtures: path.join(recordPlantRoot, "fixtures", "record-plant-registry.json"),
  templates: path.join(recordPlantRoot, "data", "record-plant-templates.json"),
  downloads: path.join(repoRoot, ".cache", "record-plant", "sources"),
  downloadsJson: path.join(recordPlantRoot, "data", "source-downloads.json"),
  analysisJson: path.join(recordPlantRoot, "data", "source-analysis.json"),
  seedJson: path.join(repoRoot, "workers", "collect-store", "seeds", "record-plant-templates.json"),
  seedSql: path.join(repoRoot, "workers", "collect-store", "seeds", "record-plant-templates.sql"),
  appJson: path.join(repoRoot, "apps", "press", "assets", "plant", "record-plant-templates.json"),
});

const command = process.argv[2] || "";
const options = parseOptions(process.argv.slice(3));

if (!["prepare", "download", "analyze", "analyse", "sync", "validate"].includes(command)) {
  usage();
  process.exit(2);
}

if (command === "prepare") {
  await prepareRegistryJson();
} else if (command === "download") {
  await downloadSources();
} else if (command === "analyze" || command === "analyse") {
  await analyzeSources();
} else if (command === "sync") {
  await syncSeedJson();
} else if (command === "validate") {
  await validateRegistryFiles();
}

async function prepareRegistryJson() {
  const registry = await readRegistry(optionPath("fixtures"));
  await writeJson(optionPath("templates"), registry);
  console.log(
    `prepared ${registry.templates.length} record-plant templates from ${path.relative(repoRoot, optionPath("fixtures"))}`,
  );
}

async function downloadSources() {
  const templates = await readTemplates();
  const sources = uniqueSourcesFromTemplates(templates);
  await mkdir(optionPath("downloads"), { recursive: true });
  const previousDownloads = await readDownloadManifestDownloads(optionPath("downloadsJson"));
  const previousById = new Map(previousDownloads.map((download) => [download.id, download]));
  const forceDownload = optionFlag("forceDownload") || optionFlag("force") || optionFlag("refresh");

  const downloads = [];
  let cached = 0;
  let fetched = 0;
  for (const source of sources) {
    const url = new URL(source.url);
    const ext = extensionFromUrl(url);
    const fileName = `${source.id}${ext}`;
    const target = path.join(optionPath("downloads"), fileName);
    const relativePath = path.relative(repoRoot, target);
    const previous = previousById.get(source.id);
    const cachedBytes = forceDownload ? null : await readFileIfExists(target);
    if (cachedBytes) {
      downloads.push({
        id: source.id,
        title: source.title,
        url: source.url,
        retrievedOn: source.retrievedOn,
        templateIds: source.templateIds,
        path: relativePath,
        contentType: previousContentType(previous, source.url, relativePath, ext),
        bytes: cachedBytes.length,
        sha256: sha256(cachedBytes),
        cacheStatus: "hit",
      });
      cached += 1;
      continue;
    }

    const { bytes, contentType } = await fetchSource(source.url);
    await writeFile(target, bytes);
    downloads.push({
      id: source.id,
      title: source.title,
      url: source.url,
      retrievedOn: source.retrievedOn,
      templateIds: source.templateIds,
      path: relativePath,
      contentType,
      bytes: bytes.length,
      sha256: sha256(bytes),
      cacheStatus: "fetched",
    });
    fetched += 1;
  }

  await writeJson(optionPath("downloadsJson"), {
    schemaVersion: 1,
    generatedAt: new Date().toISOString(),
    cache: {
      directory: path.relative(repoRoot, optionPath("downloads")),
      policy: forceDownload ? "force-download" : "reuse-existing",
      hits: cached,
      misses: fetched,
    },
    downloads,
  });
  console.log(`cached ${cached}, downloaded ${fetched} record-plant sources`);
}

async function analyzeSources() {
  requireCommand("unzip");
  const manifest = await readJson(optionPath("downloadsJson"));
  const downloads = Array.isArray(manifest.downloads) ? manifest.downloads : [];
  const sources = [];

  for (const download of downloads) {
    const absolutePath = path.join(repoRoot, download.path);
    const archiveEntries = isZip(download) ? zipEntries(absolutePath) : [];
    const pdfs = [];

    if (isZip(download)) {
      for (const entry of archiveEntries.filter((entry) => /\.pdf$/i.test(entry))) {
        const pdfBytes = unzipEntry(absolutePath, entry);
        pdfs.push({
          entry,
          pageBoxes: pdfMediaBoxes(pdfBytes),
        });
      }
    } else if (/\.pdf$/i.test(download.path)) {
      const pdfBytes = await readFile(absolutePath);
      pdfs.push({
        entry: path.basename(download.path),
        pageBoxes: pdfMediaBoxes(pdfBytes),
      });
    }

    sources.push({
      id: download.id,
      title: download.title,
      url: download.url,
      retrievedOn: download.retrievedOn,
      templateIds: download.templateIds,
      path: download.path,
      contentType: download.contentType,
      bytes: download.bytes,
      sha256: download.sha256,
      archiveEntries,
      pdfs,
    });
  }

  await writeJson(optionPath("analysisJson"), {
    schemaVersion: 1,
    generatedAt: new Date().toISOString(),
    sources,
  });
  console.log(`analyzed ${sources.length} record-plant sources`);
}

async function syncSeedJson() {
  const registry = await readRegistry(optionPath("templates"));
  const templates = registry.templates;
  const analysis = await readJson(optionPath("analysisJson"));
  const generatedAt = new Date().toISOString();
  const sourceAnalysis = new Map((analysis.sources || []).map((source) => [source.id, source]));
  const manufacturers = new Map(registry.manufacturers.map((manufacturer) => [manufacturer.id, manufacturer]));
  const manufacturerTemplateText = new Map();
  const sources = new Map();
  const output = {
    schemaVersion: 1,
    generatedAt,
    manufacturers: [],
    sources: [],
    templates: [],
    templateSources: [],
    guides: [],
    acceptedFormats: [],
    colorModes: [],
    notes: [],
  };

  for (const template of templates) {
    const manufacturer = manufacturerForTemplate(registry, template);
    const manufacturerId = manufacturer.id;
    const searchParts = manufacturerTemplateText.get(manufacturerId) || [];
    searchParts.push(
      template.id,
      template.name,
      template.product,
      template.kind,
      template.version,
      template.confidence,
      template.requirements?.preferredOutput,
      template.requirements?.outputConditionIdentifier,
      ...(template.requirements?.acceptedFormats || []),
      ...(template.requirements?.colorModes || []),
    );
    manufacturerTemplateText.set(manufacturerId, searchParts);

    const sourceId = sourceIdFor(template.source);
    if (!sources.has(sourceId)) {
      const analysisSource = sourceAnalysis.get(sourceId);
      if (!analysisSource) {
        throw new Error(`Missing source analysis for ${sourceId}. Run record-plant-sources-analyze first.`);
      }
      sources.set(sourceId, {
        id: sourceId,
        title: template.source?.title || "",
        url: template.source?.url || "",
        retrievedOn: template.source?.retrievedOn || "",
        sourceKind: "template",
        download: {
          path: analysisSource.path,
          contentType: analysisSource.contentType,
          bytes: analysisSource.bytes,
          sha256: analysisSource.sha256,
          archiveEntries: analysisSource.archiveEntries,
          pdfs: analysisSource.pdfs,
        },
      });
    }

    output.templates.push({
      id: template.id,
      manufacturerId,
      name: template.name,
      product: template.product,
      kind: template.kind,
      version: template.version,
      document: template.document,
      confidence: template.confidence,
      requirements: {
        preferredOutput: template.requirements?.preferredOutput || "",
        minRasterPpi: template.requirements?.minRasterPpi ?? null,
        minBitmapPpi: template.requirements?.minBitmapPpi ?? null,
        bleedMm: template.requirements?.bleedMm ?? null,
        safetyMm: template.requirements?.safetyMm ?? null,
        keepTemplateLayerOutOfFinal: Boolean(template.requirements?.keepTemplateLayerOutOfFinal),
        embedOrOutlineFonts: Boolean(template.requirements?.embedOrOutlineFonts),
        pdfStandard: template.requirements?.pdfStandard ?? null,
        ...(template.requirements?.outputConditionIdentifier
          ? { outputConditionIdentifier: template.requirements.outputConditionIdentifier }
          : {}),
      },
      active: true,
    });
    output.templateSources.push({
      templateId: template.id,
      sourceId,
      role: "primary",
      sortOrder: 0,
    });

    for (const [sortOrder, guide] of (template.guides || []).entries()) {
      output.guides.push({
        templateId: template.id,
        guideId: guide.id,
        layer: guide.layer,
        geometry: guide.geometry,
        sortOrder,
      });
    }
    for (const [sortOrder, format] of (template.requirements?.acceptedFormats || []).entries()) {
      output.acceptedFormats.push({ templateId: template.id, format, sortOrder });
    }
    for (const [sortOrder, colorMode] of (template.requirements?.colorModes || []).entries()) {
      output.colorModes.push({ templateId: template.id, colorMode, sortOrder });
    }
    for (const [sortOrder, note] of (template.requirements?.notes || []).entries()) {
      output.notes.push({ templateId: template.id, noteKind: "requirement", sortOrder, note });
    }
    for (const [sortOrder, note] of (template.sourceNotes || []).entries()) {
      output.notes.push({ templateId: template.id, noteKind: "source", sortOrder, note });
    }
  }

  output.manufacturers = [...manufacturers.values()]
    .map((manufacturer) => ({
      id: manufacturer.id,
      name: manufacturer.name,
      countryCode: manufacturer.countryCode ?? null,
      websiteUrl: manufacturer.websiteUrl ?? null,
      contactEmail: manufacturer.contactEmail ?? null,
      contactUrl: manufacturer.contactUrl ?? null,
      searchText: manufacturerSearchText(manufacturer, manufacturerTemplateText.get(manufacturer.id) || []),
    }))
    .sort((a, b) => a.name.localeCompare(b.name));
  output.sources = [...sources.values()].sort((a, b) => a.title.localeCompare(b.title));
  const outputManufacturers = new Map(output.manufacturers.map((manufacturer) => [manufacturer.id, manufacturer]));
  const appTemplates = templates.map((template) => ({
    ...template,
    manufacturerProfile: outputManufacturers.get(template.manufacturerId),
  }));

  await writeJson(optionPath("seedJson"), output);
  await writeFile(optionPath("seedSql"), seedSql(output));
  await writeJson(optionPath("appJson"), {
    schemaVersion: 1,
    generatedAt,
    manufacturers: output.manufacturers,
    templates: appTemplates,
  });
  console.log(`synced ${output.templates.length} templates to ${path.relative(repoRoot, optionPath("seedJson"))}`);
  console.log(`synced D1 seed SQL to ${path.relative(repoRoot, optionPath("seedSql"))}`);
  console.log(`synced ${templates.length} app templates to ${path.relative(repoRoot, optionPath("appJson"))}`);
}

async function validateRegistryFiles() {
  const fixture = await readRegistry(optionPath("fixtures"));
  const generated = await readRegistry(optionPath("templates"));
  const app = validateAppRegistry(await readJson(optionPath("appJson")), optionPath("appJson"));
  const seed = validateSeedRegistry(await readJson(optionPath("seedJson")), optionPath("seedJson"));

  assertSameIds(
    "manufacturer",
    fixture.manufacturers.map((manufacturer) => manufacturer.id),
    generated.manufacturers.map((manufacturer) => manufacturer.id),
    optionPath("templates"),
  );
  assertSameIds(
    "template",
    fixture.templates.map((template) => template.id),
    generated.templates.map((template) => template.id),
    optionPath("templates"),
  );
  assertSameIds(
    "manufacturer",
    fixture.manufacturers.map((manufacturer) => manufacturer.id),
    app.manufacturers.map((manufacturer) => manufacturer.id),
    optionPath("appJson"),
  );
  assertSameIds(
    "template",
    fixture.templates.map((template) => template.id),
    app.templates.map((template) => template.id),
    optionPath("appJson"),
  );
  assertSameIds(
    "manufacturer",
    fixture.manufacturers.map((manufacturer) => manufacturer.id),
    seed.manufacturers.map((manufacturer) => manufacturer.id),
    optionPath("seedJson"),
  );
  assertSameIds(
    "template",
    fixture.templates.map((template) => template.id),
    seed.templates.map((template) => template.id),
    optionPath("seedJson"),
  );

  console.log(
    `validated record-plant registry shape: ${fixture.manufacturers.length} manufacturers, ${fixture.templates.length} templates`,
  );
}

function seedSql(seed) {
  const now = seed.generatedAt;
  const statements = [
    '-- Generated by record-plant/scripts/plant-template-sources.mjs; do not edit by hand.',
    'BEGIN TRANSACTION;',
    `DELETE FROM record_plant_template_notes;`,
    `DELETE FROM record_plant_template_color_modes;`,
    `DELETE FROM record_plant_template_accepted_formats;`,
    `DELETE FROM record_plant_template_guides;`,
    `DELETE FROM record_plant_template_sources;`,
    `DELETE FROM record_plant_templates;`,
    `DELETE FROM record_plant_sources;`,
    `DELETE FROM record_plant_manufacturers;`,
  ];

  for (const manufacturer of seed.manufacturers) {
    statements.push(insertSql('record_plant_manufacturers', {
      id: manufacturer.id,
      name: manufacturer.name,
      country_code: manufacturer.countryCode,
      website_url: manufacturer.websiteUrl,
      contact_email: manufacturer.contactEmail,
      contact_url: manufacturer.contactUrl,
      search_text: manufacturer.searchText,
      created_at: now,
      updated_at: now,
    }));
  }
  for (const source of seed.sources) {
    statements.push(insertSql('record_plant_sources', {
      id: source.id,
      title: source.title,
      url: source.url,
      retrieved_on: source.retrievedOn,
      source_kind: source.sourceKind,
      created_at: now,
      updated_at: now,
    }));
  }
  for (const template of seed.templates) {
    statements.push(insertSql('record_plant_templates', {
      id: template.id,
      manufacturer_id: template.manufacturerId,
      name: template.name,
      product: template.product,
      kind: template.kind,
      version: template.version,
      document_x_mm: template.document.x,
      document_y_mm: template.document.y,
      document_width_mm: template.document.width,
      document_height_mm: template.document.height,
      confidence: template.confidence,
      preferred_output: template.requirements.preferredOutput,
      min_raster_ppi: template.requirements.minRasterPpi,
      min_bitmap_ppi: template.requirements.minBitmapPpi,
      bleed_mm: template.requirements.bleedMm,
      safety_mm: template.requirements.safetyMm,
      keep_template_layer_out_of_final: template.requirements.keepTemplateLayerOutOfFinal ? 1 : 0,
      embed_or_outline_fonts: template.requirements.embedOrOutlineFonts ? 1 : 0,
      pdf_standard: template.requirements.pdfStandard,
      ...(template.requirements.outputConditionIdentifier
        ? { output_condition_identifier: template.requirements.outputConditionIdentifier }
        : {}),
      active: template.active ? 1 : 0,
      created_at: now,
      updated_at: now,
    }));
  }
  for (const item of seed.templateSources) {
    statements.push(insertSql('record_plant_template_sources', {
      template_id: item.templateId,
      source_id: item.sourceId,
      role: item.role,
      sort_order: item.sortOrder,
    }));
  }
  for (const item of seed.guides) {
    statements.push(insertSql('record_plant_template_guides', {
      template_id: item.templateId,
      guide_id: item.guideId,
      layer: item.layer,
      ...guideGeometrySqlValues(item.geometry),
      sort_order: item.sortOrder,
    }));
  }
  for (const item of seed.acceptedFormats) {
    statements.push(insertSql('record_plant_template_accepted_formats', {
      template_id: item.templateId,
      format: item.format,
      sort_order: item.sortOrder,
    }));
  }
  for (const item of seed.colorModes) {
    statements.push(insertSql('record_plant_template_color_modes', {
      template_id: item.templateId,
      color_mode: item.colorMode,
      sort_order: item.sortOrder,
    }));
  }
  for (const item of seed.notes) {
    statements.push(insertSql('record_plant_template_notes', {
      template_id: item.templateId,
      note_kind: item.noteKind,
      sort_order: item.sortOrder,
      note: item.note,
    }));
  }
  statements.push('COMMIT;');
  return `${statements.join('\n')}\n`;
}

function guideGeometrySqlValues(geometry = {}) {
  if (geometry.shape === 'circle') {
    return {
      geometry_shape: 'circle',
      circle_cx_mm: geometry.circle?.cx,
      circle_cy_mm: geometry.circle?.cy,
      circle_radius_mm: geometry.circle?.radius,
      rect_x_mm: null,
      rect_y_mm: null,
      rect_width_mm: null,
      rect_height_mm: null,
      line_x_mm: null,
      line_y1_mm: null,
      line_y2_mm: null,
      line_y_mm: null,
      line_x1_mm: null,
      line_x2_mm: null,
    };
  }
  if (geometry.shape === 'rect') {
    return {
      geometry_shape: 'rect',
      circle_cx_mm: null,
      circle_cy_mm: null,
      circle_radius_mm: null,
      rect_x_mm: geometry.rect?.x,
      rect_y_mm: geometry.rect?.y,
      rect_width_mm: geometry.rect?.width,
      rect_height_mm: geometry.rect?.height,
      line_x_mm: null,
      line_y1_mm: null,
      line_y2_mm: null,
      line_y_mm: null,
      line_x1_mm: null,
      line_x2_mm: null,
    };
  }
  if (geometry.shape === 'verticalLine') {
    return {
      geometry_shape: 'vertical-line',
      circle_cx_mm: null,
      circle_cy_mm: null,
      circle_radius_mm: null,
      rect_x_mm: null,
      rect_y_mm: null,
      rect_width_mm: null,
      rect_height_mm: null,
      line_x_mm: geometry.x,
      line_y1_mm: geometry.y1,
      line_y2_mm: geometry.y2,
      line_y_mm: null,
      line_x1_mm: null,
      line_x2_mm: null,
    };
  }
  if (geometry.shape === 'horizontalLine') {
    return {
      geometry_shape: 'horizontal-line',
      circle_cx_mm: null,
      circle_cy_mm: null,
      circle_radius_mm: null,
      rect_x_mm: null,
      rect_y_mm: null,
      rect_width_mm: null,
      rect_height_mm: null,
      line_x_mm: null,
      line_y1_mm: null,
      line_y2_mm: null,
      line_y_mm: geometry.y,
      line_x1_mm: geometry.x1,
      line_x2_mm: geometry.x2,
    };
  }
  throw new Error(`Unsupported guide geometry shape: ${geometry.shape}`);
}

function insertSql(table, values) {
  const columns = Object.keys(values);
  return `INSERT INTO ${table} (${columns.join(', ')}) VALUES (${columns.map((column) => sqlValue(values[column])).join(', ')});`;
}

function manufacturerSearchText(manufacturer, templateParts = []) {
  return [
    manufacturer.id,
    manufacturer.name,
    manufacturer.countryCode,
    manufacturer.websiteUrl,
    manufacturer.contactEmail,
    manufacturer.contactUrl,
    ...templateParts,
  ]
    .filter((value) => value != null && String(value).trim())
    .map((value) => String(value).toLowerCase().replace(/\s+/g, ' ').trim())
    .join(' ');
}

function sqlValue(value) {
  if (value == null) return 'NULL';
  if (typeof value === 'number') {
    if (!Number.isFinite(value)) throw new Error(`Invalid SQL number: ${value}`);
    return String(value);
  }
  return `'${String(value).replace(/'/g, "''")}'`;
}

async function readRegistry(file) {
  const payload = await readJson(file);
  if (!payload || typeof payload !== "object" || Array.isArray(payload)) {
    throw new Error(`Registry JSON must be an object: ${file}`);
  }
  const registry = {
    schemaVersion: payload.schemaVersion,
    manufacturers: payload.manufacturers,
    templates: payload.templates,
  };
  validateRegistry(registry, file);
  return registry;
}

async function readTemplates() {
  return (await readRegistry(optionPath("templates"))).templates;
}

function validateRegistry(registry, file) {
  if (!Number.isInteger(registry.schemaVersion) || registry.schemaVersion < 1) {
    throw new Error(`Registry JSON needs integer schemaVersion: ${file}`);
  }
  if (!Array.isArray(registry.manufacturers) || registry.manufacturers.length === 0) {
    throw new Error(`Registry JSON needs manufacturers[]: ${file}`);
  }
  if (!Array.isArray(registry.templates) || registry.templates.length === 0) {
    throw new Error(`Registry JSON needs templates[]: ${file}`);
  }
  assertUnique("manufacturer", registry.manufacturers.map((manufacturer) => manufacturer.id));
  assertUnique("template", registry.templates.map((template) => template.id));
  const manufacturers = new Map(registry.manufacturers.map((manufacturer) => [manufacturer.id, manufacturer]));
  for (const manufacturer of registry.manufacturers) {
    assertNonEmpty(manufacturer.id, "manufacturer.id");
    assertNonEmpty(manufacturer.name, `manufacturer ${manufacturer.id}.name`);
  }
  for (const template of registry.templates) {
    assertNonEmpty(template.id, "template.id");
    assertNonEmpty(template.manufacturerId, `template ${template.id}.manufacturerId`);
    assertNonEmpty(template.manufacturer, `template ${template.id}.manufacturer`);
    assertNonEmpty(template.source?.url, `template ${template.id}.source.url`);
    if (!manufacturers.has(template.manufacturerId)) {
      throw new Error(`Template ${template.id} references unknown manufacturer ${template.manufacturerId}`);
    }
    const manufacturer = manufacturers.get(template.manufacturerId);
    if (manufacturer.name !== template.manufacturer) {
      throw new Error(
        `Template ${template.id} manufacturer "${template.manufacturer}" does not match ${template.manufacturerId} "${manufacturer.name}"`,
      );
    }
  }
}

function validateAppRegistry(payload, file) {
  if (!payload || typeof payload !== "object" || Array.isArray(payload)) {
    throw new Error(`Plant app registry JSON must be an object: ${file}`);
  }
  const registry = validateGeneratedRegistry(payload, file, "Plant app registry JSON");
  const manufacturers = new Map(registry.manufacturers.map((manufacturer) => [manufacturer.id, manufacturer]));
  for (const template of registry.templates) {
    assertNonEmpty(template.manufacturerId, `template ${template.id}.manufacturerId`);
    assertNonEmpty(template.manufacturer, `template ${template.id}.manufacturer`);
    const profile = template.manufacturerProfile;
    if (!profile || typeof profile !== "object" || Array.isArray(profile)) {
      throw new Error(`Plant app registry template ${template.id} needs manufacturerProfile`);
    }
    if (profile.id !== template.manufacturerId) {
      throw new Error(`Plant app registry template ${template.id} manufacturerProfile.id does not match manufacturerId`);
    }
    const manufacturer = manufacturers.get(profile.id);
    if (!manufacturer) {
      throw new Error(`Plant app registry template ${template.id} references unknown manufacturer ${profile.id}`);
    }
    if (manufacturer.name !== template.manufacturer || profile.name !== manufacturer.name) {
      throw new Error(`Plant app registry template ${template.id} manufacturer name does not match manufacturerProfile`);
    }
  }
  return registry;
}

function validateSeedRegistry(payload, file) {
  if (!payload || typeof payload !== "object" || Array.isArray(payload)) {
    throw new Error(`Collect-store seed JSON must be an object: ${file}`);
  }
  const registry = validateGeneratedRegistry(payload, file, "Collect-store seed JSON");
  const manufacturers = new Set(registry.manufacturers.map((manufacturer) => manufacturer.id));
  for (const template of registry.templates) {
    assertNonEmpty(template.manufacturerId, `template ${template.id}.manufacturerId`);
    if (!manufacturers.has(template.manufacturerId)) {
      throw new Error(`Collect-store seed template ${template.id} references unknown manufacturer ${template.manufacturerId}`);
    }
  }
  return registry;
}

function validateGeneratedRegistry(payload, file, label) {
  if (!Number.isInteger(payload.schemaVersion) || payload.schemaVersion < 1) {
    throw new Error(`${label} needs integer schemaVersion: ${file}`);
  }
  if (!Array.isArray(payload.manufacturers) || payload.manufacturers.length === 0) {
    throw new Error(`${label} needs manufacturers[]: ${file}`);
  }
  if (!Array.isArray(payload.templates) || payload.templates.length === 0) {
    throw new Error(`${label} needs templates[]: ${file}`);
  }
  assertUnique("manufacturer", payload.manufacturers.map((manufacturer) => manufacturer.id));
  assertUnique("template", payload.templates.map((template) => template.id));
  for (const manufacturer of payload.manufacturers) {
    assertNonEmpty(manufacturer.id, "manufacturer.id");
    assertNonEmpty(manufacturer.name, `manufacturer ${manufacturer.id}.name`);
  }
  for (const template of payload.templates) {
    assertNonEmpty(template.id, "template.id");
  }
  return {
    manufacturers: payload.manufacturers,
    templates: payload.templates,
  };
}

function assertUnique(label, values) {
  const seen = new Set();
  for (const value of values) {
    assertNonEmpty(value, `${label}.id`);
    if (seen.has(value)) {
      throw new Error(`Duplicate ${label} id: ${value}`);
    }
    seen.add(value);
  }
}

function assertSameIds(label, left, right, file) {
  const leftIds = [...left].sort();
  const rightIds = [...right].sort();
  if (leftIds.length !== rightIds.length || leftIds.some((id, index) => id !== rightIds[index])) {
    throw new Error(`${file} ${label} ids do not match fixture registry`);
  }
}

function assertNonEmpty(value, label) {
  if (typeof value !== "string" || !value.trim()) {
    throw new Error(`${label} is required`);
  }
}

function manufacturerForTemplate(registry, template) {
  const manufacturer = registry.manufacturers.find((candidate) => candidate.id === template.manufacturerId);
  if (!manufacturer) {
    throw new Error(`Missing manufacturer profile for template ${template.id}: ${template.manufacturerId}`);
  }
  return manufacturer;
}

function uniqueSourcesFromTemplates(templates) {
  const sources = new Map();
  for (const template of templates) {
    if (!template.source?.url) continue;
    const id = sourceIdFor(template.source);
    const existing = sources.get(id) || {
      id,
      title: template.source.title || id,
      url: template.source.url,
      retrievedOn: template.source.retrievedOn || "",
      templateIds: [],
    };
    existing.templateIds.push(template.id);
    sources.set(id, existing);
  }
  return [...sources.values()].sort((a, b) => a.id.localeCompare(b.id));
}

function sourceIdFor(source = {}) {
  const base = source.title || new URL(source.url).pathname.split("/").filter(Boolean).pop() || source.url;
  return slugify(base).slice(0, 96);
}

function slugify(value) {
  return String(value || "source")
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "") || "source";
}

function extensionFromUrl(url) {
  const ext = path.extname(url.pathname).toLowerCase();
  return ext || ".bin";
}

function isZip(download) {
  return /\.zip$/i.test(download.path) || /zip/i.test(download.contentType || "");
}

function zipEntries(file) {
  return run("unzip", ["-Z1", file])
    .split(/\r?\n/)
    .map((entry) => entry.trim())
    .filter(Boolean);
}

function unzipEntry(file, entry) {
  const result = spawnSync("unzip", ["-p", file, entry], {
    cwd: repoRoot,
    encoding: null,
    maxBuffer: 64 * 1024 * 1024,
  });
  if (result.status !== 0) {
    throw new Error(`unzip -p failed for ${entry} in ${file}: ${String(result.stderr || "")}`);
  }
  return Buffer.from(result.stdout);
}

function pdfMediaBoxes(bytes) {
  const text = Buffer.from(bytes).toString("latin1");
  const boxes = [];
  const pattern = /\/(?:MediaBox|CropBox)\s*\[\s*(-?\d+(?:\.\d+)?)\s+(-?\d+(?:\.\d+)?)\s+(-?\d+(?:\.\d+)?)\s+(-?\d+(?:\.\d+)?)\s*\]/g;
  const seen = new Set();
  for (const match of text.matchAll(pattern)) {
    const [, x1, y1, x2, y2] = match.map(Number);
    const widthPt = Math.abs(x2 - x1);
    const heightPt = Math.abs(y2 - y1);
    const key = `${widthPt}:${heightPt}`;
    if (seen.has(key)) continue;
    seen.add(key);
    boxes.push({
      widthPt: round(widthPt, 3),
      heightPt: round(heightPt, 3),
      widthMm: round((widthPt / 72) * 25.4, 3),
      heightMm: round((heightPt / 72) * 25.4, 3),
    });
  }
  return boxes;
}

function round(value, places) {
  const factor = 10 ** places;
  return Math.round(value * factor) / factor;
}

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

async function fetchSource(url) {
  const response = await fetch(url, { redirect: "follow" });
  if (!response.ok) {
    throw new Error(`Failed to download ${url}: HTTP ${response.status}`);
  }
  return {
    bytes: Buffer.from(await response.arrayBuffer()),
    contentType: response.headers.get("content-type") || contentTypeFromExtension(extensionFromUrl(new URL(url))),
  };
}

async function readDownloadManifestDownloads(file) {
  try {
    const manifest = await readJson(file);
    return Array.isArray(manifest.downloads) ? manifest.downloads : [];
  } catch (error) {
    if (error?.code === "ENOENT") return [];
    throw error;
  }
}

async function readFileIfExists(file) {
  try {
    return await readFile(file);
  } catch (error) {
    if (error?.code === "ENOENT") return null;
    throw error;
  }
}

function previousContentType(previous, url, relativePath, ext) {
  if (previous?.url === url && previous?.path === relativePath && previous.contentType) {
    return previous.contentType;
  }
  return contentTypeFromExtension(ext);
}

function contentTypeFromExtension(ext) {
  if (/\.pdf$/i.test(ext)) return "application/pdf";
  if (/\.zip$/i.test(ext)) return "application/zip";
  if (/\.eps$/i.test(ext)) return "application/postscript";
  return "application/octet-stream";
}

function requireCommand(name) {
  const result = spawnSync("sh", ["-lc", `command -v ${shellQuote(name)}`], {
    cwd: repoRoot,
    encoding: "utf8",
  });
  if (result.status !== 0) {
    throw new Error(`${name} not found`);
  }
}

function run(cmd, args) {
  const result = spawnSync(cmd, args, {
    cwd: repoRoot,
    encoding: "utf8",
    maxBuffer: 64 * 1024 * 1024,
  });
  if (result.status !== 0) {
    throw new Error(`${cmd} ${args.join(" ")} failed: ${result.stderr || result.stdout}`);
  }
  return result.stdout;
}

function shellQuote(value) {
  return `'${String(value).replace(/'/g, "'\\''")}'`;
}

async function readJson(file) {
  return JSON.parse(await readFile(file, "utf8"));
}

async function writeJson(file, value) {
  await mkdir(path.dirname(file), { recursive: true });
  await writeFile(file, `${JSON.stringify(value, null, 2)}\n`);
}

function optionPath(name) {
  return path.resolve(repoRoot, options[name] || defaults[name]);
}

function optionFlag(name) {
  return ["1", "true", "yes", "on"].includes(String(options[name] || "").toLowerCase());
}

function parseOptions(args) {
  const parsed = {};
  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    if (!arg.startsWith("--")) continue;
    const key = arg.slice(2).replace(/-([a-z])/g, (_match, ch) => ch.toUpperCase());
    const value = args[index + 1];
    if (!value || value.startsWith("--")) {
      parsed[key] = "1";
    } else {
      parsed[key] = value;
      index += 1;
    }
  }
  return parsed;
}

function usage() {
  console.error(`Usage: node record-plant/scripts/plant-template-sources.mjs <prepare|download|analyze|sync|validate> [options]

Options:
  --fixtures <path>       Curated registry fixture JSON
  --templates <path>      Generated registry JSON used by tooling
  --downloads <dir>       Download directory
  --downloads-json <path> Download manifest
  --force-download        Re-fetch sources even when cached files exist
  --analysis-json <path>  Analysis JSON
  --seed-json <path>      Normalized collect-store seed JSON
  --seed-sql <path>       D1 seed SQL
  --app-json <path>       Hydrated Plant app registry JSON
`);
}
