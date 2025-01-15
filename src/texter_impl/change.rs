use lsp_types::{Position, TextDocumentContentChangeEvent};
use texter::change::{Change, GridIndex};

pub struct WrapChange<'a> {
    pub change: Change<'a>,
}

impl<'a> WrapChange<'a> {
    pub fn new(change: Change<'a>) -> Self {
        Self { change }
    }
}

fn grid_into_position(value: Position) -> GridIndex {
    GridIndex {
        row: value.line as usize,
        col: value.character as usize,
    }
}

impl<'a> From<&'a TextDocumentContentChangeEvent> for WrapChange<'a> {
    fn from(value: &'a TextDocumentContentChangeEvent) -> Self {
        let Some(range) = value.range else {
            return WrapChange::new(Change::ReplaceFull((&value.text).into()));
        };

        if value.text.is_empty() {
            return WrapChange::new(Change::Delete {
                start: grid_into_position(range.start),
                end: grid_into_position(range.end),
            });
        }

        if range.start == range.end {
            return WrapChange::new(Change::Insert {
                at: grid_into_position(range.start),
                text: (&value.text).into(),
            });
        }

        WrapChange::new(Change::Replace {
            start: grid_into_position(range.start),
            end: grid_into_position(range.end),
            text: (&value.text).into(),
        })
    }
}
