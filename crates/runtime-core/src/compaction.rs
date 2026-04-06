use crate::session::SessionTurn;
use crate::text::summarize_text;

#[derive(Clone, Debug)]
pub(crate) struct CompactionResult {
    pub summary: String,
}

pub(crate) fn compact_session_turns(turns: &[SessionTurn]) -> CompactionResult {
    let summary = turns
        .iter()
        .enumerate()
        .map(|(index, turn)| compact_turn(index, turn))
        .collect::<Vec<_>>()
        .join(" || ");
    CompactionResult { summary }
}

fn compact_turn(index: usize, turn: &SessionTurn) -> String {
    format!(
        "第{}轮 用户：{} | 智能体：{}",
        index + 1,
        summarize_text(&turn.user_input),
        summarize_text(&turn.final_answer)
    )
}
