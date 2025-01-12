use lsp_types::{Position, TextDocumentContentChangeEvent};
use texter::change::{Change, GridIndex};

pub struct NewChange<'a> {
    pub change: Change<'a>,
}

impl<'a> NewChange<'a> {
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

impl<'a> From<&'a TextDocumentContentChangeEvent> for NewChange<'a> {
    fn from(value: &'a TextDocumentContentChangeEvent) -> Self {
        let Some(range) = value.range else {
            return NewChange::new(Change::ReplaceFull((&value.text).into()));
        };

        if value.text.is_empty() {
            return NewChange::new(Change::Delete {
                start: grid_into_position(range.start),
                end: grid_into_position(range.end),
            });
        }

        if range.start == range.end {
            return NewChange::new(Change::Insert {
                at: grid_into_position(range.start),
                text: (&value.text).into(),
            });
        }

        NewChange::new(Change::Replace {
            start: grid_into_position(range.start),
            end: grid_into_position(range.end),
            text: (&value.text).into(),
        })
    }
}
