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
    pub trust_tier: String,
    pub guard_action: String,
    pub guard_reason: String,
}

#[derive(Clone, Debug)]
pub(crate) struct SkillSkipRecord {
    pub skill_id: String,
    pub trust_tier: String,
    pub guard_action: String,
    pub guard_reason: String,
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
    #[serde(default)]
    trust_tier: String,
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
            catalog.skipped.push(SkillSkipRecord {
                skill_id: "_manifest".to_string(),
                trust_tier: "unknown".to_string(),
                guard_action: "deny".to_string(),
                guard_reason: "manifest_invalid".to_string(),
                reason: error,
            });
            catalog
        }
    }
}

pub(crate) fn skill_catalog_brief(catalog: &SkillCatalog) -> String {
    let loaded = catalog
        .loaded
        .iter()
        .map(|item| {
            format!(
                "{}@{}@{}@{}@{}@{}@{}",
                item.skill_id,
                item.version,
                item.entry_path,
                item.isolation_scope,
                item.trust_tier,
                item.guard_action,
                item.guard_reason
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let skipped = catalog
        .skipped
        .iter()
        .map(|item| {
            format!(
                "{}@{}@{}@{}@{}",
                item.skill_id,
                item.trust_tier,
                item.guard_action,
                item.guard_reason,
                item.reason
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "loaded_count={},skipped_count={},loaded=[{}],skipped=[{}]",
        catalog.loaded.len(),
        catalog.skipped.len(),
        loaded,
        skipped
    )
}

fn skill_manifest_path(request: &RunRequest) -> PathBuf {
    if let Some(path) = request.context_hints.get("skill_manifest_path") {
        return PathBuf::from(path);
    }
    repo_root(request).join("data").join("skills").join(format!(
        "{}.json",
        safe_id(&request.workspace_ref.workspace_id)
    ))
}

fn safe_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
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
    if semver_tuple(&item.version).is_none() {
        push_skip(catalog, &item.skill_id, &resolved_trust_tier(&item), "deny", "版本格式非法，要求 major.minor.patch。");
        return;
    }
    if !pin_matches(pins, &item.skill_id, &item.version) {
        push_skip(catalog, &item.skill_id, &resolved_trust_tier(&item), "deny", "命中版本治理 pin，但版本不匹配。");
        return;
    }
    let Some(entry_path) = isolated_entry_path(request, &item.entry) else {
        push_skip(catalog, &item.skill_id, &resolved_trust_tier(&item), "deny", "技能入口路径越界，未通过隔离校验。");
        return;
    };
    let trust_tier = resolved_trust_tier(&item);
    let guard_action = guard_action_for(&trust_tier);
    if guard_action == "deny" {
        push_skip(catalog, &item.skill_id, &trust_tier, &guard_action, &guard_reason_for(&guard_action, &trust_tier));
        return;
    }
    catalog.loaded.push(build_skill_descriptor(request, item, entry_path, trust_tier, guard_action));
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

fn resolved_trust_tier(item: &SkillManifestItem) -> String {
    match item.trust_tier.trim() {
        "builtin" => "builtin".to_string(),
        "local_generated" => "local_generated".to_string(),
        "external_imported" => "external_imported".to_string(),
        "community" => "community".to_string(),
        _ => "project_trusted".to_string(),
    }
}

fn guard_action_for(trust_tier: &str) -> String {
    match trust_tier {
        "builtin" | "project_trusted" => "allow".to_string(),
        "local_generated" | "external_imported" => "review".to_string(),
        "community" => "deny".to_string(),
        _ => "review".to_string(),
    }
}

fn guard_reason_for(action: &str, trust_tier: &str) -> String {
    match action {
        "allow" => format!("trust_tier={trust_tier}，允许进入当前默认加载级别。"),
        "review" => format!("trust_tier={trust_tier}，仅允许索引或受控展开。"),
        _ => format!("trust_tier={trust_tier}，默认禁止直接注入执行上下文。"),
    }
}

fn build_skill_descriptor(
    request: &RunRequest,
    item: SkillManifestItem,
    entry_path: String,
    trust_tier: String,
    guard_action: String,
) -> SkillDescriptor {
    let guard_reason = guard_reason_for(&guard_action, &trust_tier);
    SkillDescriptor {
        skill_id: item.skill_id,
        version: item.version,
        entry_path,
        isolation_scope: format!("workspace:{}", request.workspace_ref.workspace_id),
        trust_tier,
        guard_action,
        guard_reason,
    }
}

fn push_skip(
    catalog: &mut SkillCatalog,
    skill_id: &str,
    trust_tier: &str,
    guard_action: &str,
    reason: &str,
) {
    catalog.skipped.push(SkillSkipRecord {
        skill_id: skill_id.to_string(),
        trust_tier: trust_tier.to_string(),
        guard_action: guard_action.to_string(),
        guard_reason: guard_reason_for(guard_action, trust_tier),
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
        assert_eq!(catalog.loaded[0].trust_tier, "project_trusted");
        assert_eq!(catalog.loaded[0].guard_action, "allow");
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
        assert_eq!(catalog.skipped[0].guard_action, "deny");
    }

    #[test]
    fn marks_local_generated_skill_as_review() {
        let root = test_root("skill-review");
        write_manifest(
            &root,
            r#"{"skills":[{"skill_id":"compose_ui","version":"1.2.0","entry":"skills/compose/SKILL.md","trust_tier":"local_generated"}]}"#,
        );
        let request = sample_request(&root, "");
        let catalog = load_skill_catalog(&request);
        assert_eq!(catalog.loaded.len(), 1);
        assert_eq!(catalog.loaded[0].trust_tier, "local_generated");
        assert_eq!(catalog.loaded[0].guard_action, "review");
    }

    #[test]
    fn denies_community_skill_by_default() {
        let root = test_root("skill-community");
        write_manifest(
            &root,
            r#"{"skills":[{"skill_id":"compose_ui","version":"1.2.0","entry":"skills/compose/SKILL.md","trust_tier":"community"}]}"#,
        );
        let request = sample_request(&root, "");
        let catalog = load_skill_catalog(&request);
        assert!(catalog.loaded.is_empty());
        assert_eq!(catalog.skipped[0].trust_tier, "community");
        assert_eq!(catalog.skipped[0].guard_action, "deny");
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
            model_ref: ModelRef {
                provider_id: "provider".to_string(),
                model_id: "model".to_string(),
                display_name: "Model".to_string(),
            },
            provider_ref: ProviderRef::default(),
            workspace_ref: WorkspaceRef {
                workspace_id: "workspace-1".to_string(),
                name: "Workspace".to_string(),
                root_path: root.join("workspace").display().to_string(),
                is_active: true,
            },
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
