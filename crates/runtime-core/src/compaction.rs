use crate::session::SessionTurn;
use crate::text::summarize_text;

#[derive(Clone, Debug)]
pub(crate) struct CompactionResult {
    pub summary: String,
}

const COMPACTION_AGGREGATE_CHAR_BUDGET: usize = 900;
const COMPACTION_MAX_TURNS: usize = 4;

pub(crate) fn compact_session_turns(turns: &[SessionTurn]) -> CompactionResult {
    let selected = select_turns_with_budget(turns);
    let mut summary = selected
        .items
        .iter()
        .map(|item| compact_turn(item.index, item.turn))
        .collect::<Vec<_>>()
        .join(" || ");
    if selected.dropped > 0 {
        summary = append_boundary_hint(summary, selected.dropped);
    }
    CompactionResult { summary }
}

fn select_turns_with_budget(turns: &[SessionTurn]) -> SelectedTurns<'_> {
    let mut picked = Vec::new();
    let mut chars = 0;
    for (index, turn) in turns.iter().enumerate().rev() {
        if picked.len() >= COMPACTION_MAX_TURNS {
            break;
        }
        let candidate = compact_turn(index, turn);
        let cost = candidate.chars().count() + separator_cost(picked.len());
        if !picked.is_empty() && chars + cost > COMPACTION_AGGREGATE_CHAR_BUDGET {
            break;
        }
        chars += cost;
        picked.push(SelectedTurn { index, turn });
    }
    picked.reverse();
    SelectedTurns {
        dropped: turns.len().saturating_sub(picked.len()),
        items: picked,
    }
}

fn compact_turn(index: usize, turn: &SessionTurn) -> String {
    format!(
        "第{}轮 用户：{} | 智能体：{}",
        index + 1,
        summarize_text(&turn.user_input),
        summarize_text(&turn.final_answer)
    )
}

fn separator_cost(current_len: usize) -> usize {
    if current_len == 0 { 0 } else { 4 }
}

fn append_boundary_hint(summary: String, dropped: usize) -> String {
    let hint = format!(
        "边界提示：已省略更早 {} 轮（聚合预算 {} 字符）。",
        dropped, COMPACTION_AGGREGATE_CHAR_BUDGET
    );
    if summary.is_empty() {
        hint
    } else {
        format!("{summary} || {hint}")
    }
}

struct SelectedTurn<'a> {
    index: usize,
    turn: &'a SessionTurn,
}

struct SelectedTurns<'a> {
    dropped: usize,
    items: Vec<SelectedTurn<'a>>,
}

#[cfg(test)]
mod tests {
    use super::compact_session_turns;
    use crate::session::SessionTurn;

    #[test]
    fn keeps_recent_turns_with_boundary_hint_when_over_budget() {
        let turns = (0..6).map(build_turn).collect::<Vec<_>>();
        let summary = compact_session_turns(&turns).summary;
        assert!(summary.contains("边界提示：已省略更早 2 轮"));
        assert!(summary.contains("第3轮"));
        assert!(summary.contains("第6轮"));
        assert!(!summary.contains("第1轮"));
    }

    #[test]
    fn keeps_all_turns_without_boundary_hint_when_within_budget() {
        let turns = vec![build_turn(0), build_turn(1)];
        let summary = compact_session_turns(&turns).summary;
        assert!(summary.contains("第1轮"));
        assert!(summary.contains("第2轮"));
        assert!(!summary.contains("边界提示"));
    }

    fn build_turn(index: usize) -> SessionTurn {
        SessionTurn {
            user_input: format!("第{}轮用户输入：{}", index + 1, "任务描述".repeat(10)),
            final_answer: format!("第{}轮智能体输出：{}", index + 1, "执行结果".repeat(10)),
            summary: String::new(),
            timestamp: "0".to_string(),
        }
    }
}
