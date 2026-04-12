use crate::contracts::RunRequest;
use crate::paths::{repo_root, resolve_workspace_path};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub(crate) struct SkillCatalog {
    pub loaded: Vec<SkillDescriptor>,
    pub skipped: Vec<SkillSkipRecord>,
}

#[derive(Clone, Debug)]
pub(crate) struct SkillDescriptor {
    pub skill_id: String,
    pub version: String,
    pub entry_path: String,
    pub isolation_scope: String,
}

#[derive(Clone, Debug)]
pub(crate) struct SkillSkipRecord {
    pub skill_id: String,
    pub reason: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct SkillManifest {
    #[serde(default)]
    skills: Vec<SkillManifestItem>,
}

#[derive(Clone, Debug, Deserialize)]
struct SkillManifestItem {
    skill_id: String,
    version: String,
    entry: String,
    #[serde(default)]
    workspace_id: String,
}

pub(crate) fn load_skill_catalog(request: &RunRequest) -> SkillCatalog {
    let manifest_path = skill_manifest_path(request);
    let pins = skill_version_pins(request);
    let mut catalog = SkillCatalog::default();
    match read_manifest(&manifest_path) {
        Ok(manifest) => {
            for item in manifest.skills {
                apply_manifest_item(request, &pins, &mut catalog, item);
            }
            dedupe_loaded_skills(&mut catalog.loaded);
            catalog
        }
        Err(error) => {
            catalog.skipped.push(SkillSkipRecord { skill_id: "_manifest".to_string(), reason: error });
            catalog
        }
    }
}

pub(crate) fn skill_catalog_brief(catalog: &SkillCatalog) -> String {
    let loaded = catalog
        .loaded
        .iter()
        .map(|item| format!("{}@{}@{}@{}", item.skill_id, item.version, item.entry_path, item.isolation_scope))
        .collect::<Vec<_>>()
        .join(",");
    let skipped = catalog
        .skipped
        .iter()
        .map(|item| format!("{}@{}", item.skill_id, item.reason))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "loaded_count={},skipped_count={},loaded=[{}],skipped=[{}]",
        catalog.loaded.len(), catalog.skipped.len(), loaded, skipped
    )
}

fn skill_manifest_path(request: &RunRequest) -> PathBuf {
    if let Some(path) = request.context_hints.get("skill_manifest_path") {
        return PathBuf::from(path);
    }
    repo_root(request)
        .join("data")
        .join("skills")
        .join(format!("{}.json", safe_id(&request.workspace_ref.workspace_id)))
}

fn safe_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' { ch } else { '_' })
        .collect()
}

fn skill_version_pins(request: &RunRequest) -> BTreeMap<String, String> {
    let mut pins = BTreeMap::new();
    let Some(raw) = request.context_hints.get("skill_version_pins") else {
        return pins;
    };
    for item in raw.split(',') {
        let pair = item.trim();
        if pair.is_empty() || !pair.contains('@') {
            continue;
        }
        let mut parts = pair.splitn(2, '@');
        let skill_id = parts.next().unwrap_or("").trim();
        let version = parts.next().unwrap_or("").trim();
        if !skill_id.is_empty() && !version.is_empty() {
            pins.insert(skill_id.to_string(), version.to_string());
        }
    }
    pins
}

fn read_manifest(path: &std::path::Path) -> Result<SkillManifest, String> {
    if !path.exists() {
        return Ok(SkillManifest::default());
    }
    let raw = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str::<SkillManifest>(&raw).map_err(|error| error.to_string())
}

fn apply_manifest_item(
    request: &RunRequest,
    pins: &BTreeMap<String, String>,
    catalog: &mut SkillCatalog,
    item: SkillManifestItem,
) {
    if !workspace_matches(request, &item) {
        return;
    }
    let Some(current) = semver_tuple(&item.version) else {
        push_skip(catalog, &item.skill_id, "版本格式非法，要求 major.minor.patch。");
        return;
    };
    if !pin_matches(pins, &item.skill_id, &item.version) {
        push_skip(catalog, &item.skill_id, "命中版本治理 pin，但版本不匹配。");
        return;
    }
    let Some(entry_path) = isolated_entry_path(request, &item.entry) else {
        push_skip(catalog, &item.skill_id, "技能入口路径越界，未通过隔离校验。");
        return;
    };
    catalog.loaded.push(SkillDescriptor {
        skill_id: item.skill_id,
        version: item.version,
        entry_path,
        isolation_scope: format!("workspace:{}", request.workspace_ref.workspace_id),
    });
    let _ = current;
}

fn workspace_matches(request: &RunRequest, item: &SkillManifestItem) -> bool {
    item.workspace_id.is_empty() || item.workspace_id == request.workspace_ref.workspace_id
}

fn pin_matches(pins: &BTreeMap<String, String>, skill_id: &str, version: &str) -> bool {
    match pins.get(skill_id) {
        Some(pinned) => pinned == version,
        None => true,
    }
}

fn semver_tuple(version: &str) -> Option<(u32, u32, u32)> {
    let mut parts = version.trim().split('.');
    let major = parts.next()?.parse::<u32>().ok()?;
    let minor = parts.next()?.parse::<u32>().ok()?;
    let patch = parts.next()?.parse::<u32>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

fn isolated_entry_path(request: &RunRequest, entry: &str) -> Option<String> {
    resolve_workspace_path(&request.workspace_ref.root_path, entry)
        .ok()
        .map(|path| path.display().to_string())
}

fn push_skip(catalog: &mut SkillCatalog, skill_id: &str, reason: &str) {
    catalog.skipped.push(SkillSkipRecord {
        skill_id: skill_id.to_string(),
        reason: reason.to_string(),
    });
}

fn dedupe_loaded_skills(items: &mut Vec<SkillDescriptor>) {
    items.sort_by(|left, right| {
        left.skill_id
            .cmp(&right.skill_id)
            .then_with(|| semver_tuple(&right.version).cmp(&semver_tuple(&left.version)))
    });
    let mut seen = std::collections::BTreeSet::new();
    items.retain(|item| seen.insert(item.skill_id.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{ModelRef, ProviderRef, RunRequest, WorkspaceRef};
    use std::collections::BTreeMap;
    use std::fs;

    #[test]
    fn loads_skill_when_version_and_scope_match() {
        let root = test_root("skill-load");
        write_manifest(
            &root,
            r#"{"skills":[{"skill_id":"compose_ui","version":"1.2.0","entry":"skills/compose/SKILL.md","workspace_id":"workspace-1"}]}"#,
        );
        let request = sample_request(&root, "");
        let catalog = load_skill_catalog(&request);
        assert_eq!(catalog.loaded.len(), 1);
        assert!(catalog.skipped.is_empty());
        assert_eq!(catalog.loaded[0].version, "1.2.0");
    }

    #[test]
    fn skips_skill_when_pin_version_mismatch() {
        let root = test_root("skill-pin-mismatch");
        write_manifest(
            &root,
            r#"{"skills":[{"skill_id":"compose_ui","version":"1.2.0","entry":"skills/compose/SKILL.md"}]}"#,
        );
        let request = sample_request(&root, "compose_ui@1.3.0");
        let catalog = load_skill_catalog(&request);
        assert!(catalog.loaded.is_empty());
        assert_eq!(catalog.skipped.len(), 1);
        assert!(catalog.skipped[0].reason.contains("版本不匹配"));
    }

    #[test]
    fn blocks_skill_when_entry_path_escapes_workspace() {
        let root = test_root("skill-isolation");
        write_manifest(
            &root,
            r#"{"skills":[{"skill_id":"compose_ui","version":"1.2.0","entry":"../outside/SKILL.md"}]}"#,
        );
        let request = sample_request(&root, "");
        let catalog = load_skill_catalog(&request);
        assert!(catalog.loaded.is_empty());
        assert_eq!(catalog.skipped.len(), 1);
        assert!(catalog.skipped[0].reason.contains("隔离"));
    }

    fn sample_request(root: &std::path::Path, pins: &str) -> RunRequest {
        let mut context_hints = BTreeMap::new();
        context_hints.insert("repo_root".to_string(), root.display().to_string());
        if !pins.is_empty() {
            context_hints.insert("skill_version_pins".to_string(), pins.to_string());
        }
        RunRequest {
            request_id: "request-1".to_string(),
            run_id: "run-1".to_string(),
            session_id: "session-1".to_string(),
            trace_id: "trace-1".to_string(),
            user_input: "test".to_string(),
            mode: "standard".to_string(),
            model_ref: ModelRef { provider_id: "provider".to_string(), model_id: "model".to_string(), display_name: "Model".to_string() },
            provider_ref: ProviderRef::default(),
            workspace_ref: WorkspaceRef { workspace_id: "workspace-1".to_string(), name: "Workspace".to_string(), root_path: root.join("workspace").display().to_string(), is_active: true },
            context_hints,
            resume_from_checkpoint_id: String::new(),
            resume_strategy: String::new(),
            confirmation_decision: None,
        }
    }

    fn write_manifest(root: &std::path::Path, content: &str) {
        let dir = root.join("data").join("skills");
        fs::create_dir_all(&dir).unwrap();
        fs::create_dir_all(root.join("workspace")).unwrap();
        fs::write(dir.join("workspace-1.json"), content).unwrap();
    }

    fn test_root(case_name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "runtime-core-skill-catalog-{}-{}",
            case_name,
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        root
    }
}
