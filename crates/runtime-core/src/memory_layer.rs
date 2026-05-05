use crate::memory_recall::MemoryDigest;
use std::collections::BTreeMap;

pub(crate) fn digest_layer_phrase(digest: &MemoryDigest) -> String {
    let mut layers = Vec::new();
    if digest.has_system_views {
        layers.push("system views");
    }
    if digest.has_current_objects {
        layers.push("current memory object");
    }
    if layers.is_empty() {
        "普通长期记忆".to_string()
    } else {
        format!("{}，对象 {} 条", layers.join(" + "), digest.current_object_count)
    }
}

pub(crate) fn digest_route_summary(digest: &MemoryDigest) -> String {
    route_summary(&digest.memory_route, &digest.selected_layers, &digest.skipped_layers)
}

pub(crate) fn digest_layer_summary(digest: &MemoryDigest) -> String {
    let mut layers = Vec::new();
    if digest.has_system_views {
        layers.push("system views");
    }
    if digest.has_current_objects {
        layers.push("current memory object");
    }
    if layers.is_empty() {
        "普通长期记忆".to_string()
    } else {
        format!("{}（对象 {} 条）", layers.join(" + "), digest.current_object_count)
    }
}

pub(crate) fn metadata_route_summary(metadata: &BTreeMap<String, String>) -> String {
    let route = metadata_value(metadata, "memory_route");
    let selected = split_csv(&metadata_value(metadata, "memory_selected_layers"));
    let skipped = split_csv(&metadata_value(metadata, "memory_skipped_layers"));
    route_summary(&route, &selected, &skipped)
}

pub(crate) fn metadata_layer_summary(metadata: &BTreeMap<String, String>) -> String {
    let mut layers = Vec::new();
    if metadata_flag(metadata, "memory_has_system_views") {
        layers.push("system views");
    }
    if metadata_flag(metadata, "memory_has_current_objects") {
        layers.push("current memory object");
    }
    if layers.is_empty() {
        "普通长期记忆".to_string()
    } else {
        format!(
            "{}（对象 {} 条）",
            layers.join(" + "),
            metadata_usize(metadata, "memory_current_object_count")
        )
    }
}

pub(crate) fn reasoning_layer_summary(reasoning: &str) -> String {
    let marker = "本次召回层为";
    let Some(index) = reasoning.find(marker) else {
        return String::new();
    };
    let layer = &reasoning[index + marker.len()..];
    layer.trim().trim_end_matches('。').to_string()
}

fn metadata_flag(metadata: &BTreeMap<String, String>, key: &str) -> bool {
    metadata.get(key).is_some_and(|value| value == "true")
}

fn metadata_value(metadata: &BTreeMap<String, String>, key: &str) -> String {
    metadata.get(key).cloned().unwrap_or_default()
}

fn metadata_usize(metadata: &BTreeMap<String, String>, key: &str) -> usize {
    metadata
        .get(key)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or_default()
}

fn split_csv(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn route_summary(route: &str, selected: &[String], skipped: &[String]) -> String {
    if route.is_empty() && selected.is_empty() {
        return String::new();
    }
    if route.is_empty() {
        return selected.join(" + ");
    }
    if selected.is_empty() {
        return route.to_string();
    }
    format!(
        "{} -> {}（跳过：{}）",
        route,
        selected.join(" + "),
        skipped_layers_text(skipped)
    )
}

fn skipped_layers_text(layers: &[String]) -> String {
    if layers.is_empty() {
        "none".to_string()
    } else {
        layers.join(" + ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_layer_helpers_keep_object_aware_labels() {
        let digest = MemoryDigest {
            summary: "digest".to_string(),
            has_system_views: true,
            has_current_objects: true,
            current_object_count: 2,
            memory_route: String::new(),
            selected_layers: Vec::new(),
            match_reason: String::new(),
            reuse_confidence: String::new(),
            skipped_layers: Vec::new(),
        };
        assert_eq!(
            digest_layer_phrase(&digest),
            "system views + current memory object，对象 2 条"
        );
        assert_eq!(
            digest_layer_summary(&digest),
            "system views + current memory object（对象 2 条）"
        );
        assert_eq!(digest_route_summary(&digest), "");
    }

    #[test]
    fn metadata_route_summary_falls_back_to_selected_layers() {
        let metadata = BTreeMap::from([(
            "memory_selected_layers".to_string(),
            "current memory object,history entries".to_string(),
        )]);
        assert_eq!(
            metadata_route_summary(&metadata),
            "current memory object + history entries"
        );
    }

    #[test]
    fn reasoning_layer_summary_extracts_recall_layer() {
        let layer = reasoning_layer_summary(
            "按查询词检索长期记忆，并返回前几条高相关结果；本次召回层为system views + current memory object，对象 2 条。",
        );
        assert_eq!(layer, "system views + current memory object，对象 2 条");
    }

    #[test]
    fn metadata_route_summary_reads_route_fields() {
        let metadata = BTreeMap::from([
            ("memory_route".to_string(), "repair_route".to_string()),
            (
                "memory_selected_layers".to_string(),
                "current memory object,history entries".to_string(),
            ),
            ("memory_skipped_layers".to_string(), "system views".to_string()),
        ]);
        assert_eq!(
            metadata_route_summary(&metadata),
            "repair_route -> current memory object + history entries（跳过：system views）"
        );
    }
}
