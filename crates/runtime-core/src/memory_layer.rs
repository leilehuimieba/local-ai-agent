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

fn metadata_usize(metadata: &BTreeMap<String, String>, key: &str) -> usize {
    metadata
        .get(key)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or_default()
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
        };
        assert_eq!(
            digest_layer_phrase(&digest),
            "system views + current memory object，对象 2 条"
        );
        assert_eq!(
            digest_layer_summary(&digest),
            "system views + current memory object（对象 2 条）"
        );
    }

    #[test]
    fn reasoning_layer_summary_extracts_recall_layer() {
        let layer = reasoning_layer_summary(
            "按查询词检索长期记忆，并返回前几条高相关结果；本次召回层为system views + current memory object，对象 2 条。",
        );
        assert_eq!(layer, "system views + current memory object，对象 2 条");
    }
}
