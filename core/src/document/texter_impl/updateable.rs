use texter::change::GridIndex;
use texter::error::Error;
use texter::updateables::{ChangeContext, UpdateContext, Updateable};
use tree_sitter::{InputEdit, Point, Tree};

#[derive(Copy, Clone, Debug)]
pub enum ChangeKind {
    Delete,
    Insert,
    Replace,
}

impl<'a> From<&'a ChangeContext<'_>> for ChangeKind {
    fn from(value: &'a ChangeContext) -> Self {
        match value {
            ChangeContext::Delete { .. } => Self::Delete,
            ChangeContext::Insert { .. } => Self::Insert,
            ChangeContext::Replace { .. } => Self::Replace,
            ChangeContext::ReplaceFull { .. } => Self::Replace,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Change {
    pub kind: ChangeKind,
    pub input_edit: InputEdit,
    pub is_whitespace: bool,
}

impl<'a> From<(&'a ChangeContext<'_>, InputEdit, bool)> for Change {
    fn from(value: (&'a ChangeContext, InputEdit, bool)) -> Self {
        Self {
            kind: value.0.into(),
            input_edit: value.1,
            is_whitespace: value.2,
        }
    }
}

/// A wrapper around a [`Tree`] that keeps track of the edits made to it.
pub struct WrapTree<'a> {
    /// The tree being wrapped.
    pub tree: &'a mut Tree,
    /// The edits made to the tree.
    edits: Option<Vec<Change>>,
}

impl<'a> From<&'a mut Tree> for WrapTree<'a> {
    fn from(tree: &'a mut Tree) -> Self {
        Self { tree, edits: None }
    }
}

impl<'a> Updateable for WrapTree<'a> {
    /// This implementation of `update` keeps track of the edits made to the tree.
    fn update(&mut self, ctx: UpdateContext) -> Result<(), Error> {
        let new_edits = WrapTree::edit_from_ctx(&ctx)?;
        self.tree.edit(&new_edits.0);
        match self.edits {
            Some(ref mut edits) => {
                edits.push(((&ctx.change).into(), new_edits.0, new_edits.1).into())
            }
            None => {
                self.edits = Some(vec![((&ctx.change).into(), new_edits.0, new_edits.1).into()])
            }
        };
        Ok(())
    }
}

fn grid_into_point(value: GridIndex) -> Point {
    Point {
        row: value.row,
        column: value.col,
    }
}

impl<'a> WrapTree<'a> {
    /// Returns the edits made to the tree since the last call to `get_edits`.
    ///
    /// This function consumes the `WrapTree` and returns the edits.
    pub fn get_edits(&'a mut self) -> Vec<Change> {
        self.edits.take().unwrap_or_default()
    }

    /// Taken from the original `edit_from_ctx` in `texter_impl/updateable.rs`.
    ///
    /// The difference here is that this function returns a tuple ([`tree_sitter::InputEdit`], `bool`).
    ///
    /// Where [`tree_sitter::InputEdit`] is the edit to be made to the tree, and `bool` is whether the edit is whitespace.
    fn edit_from_ctx(ctx: &UpdateContext) -> anyhow::Result<(InputEdit, bool), Error> {
        let old_br = ctx.old_breaklines;
        let new_br = ctx.breaklines;
        let ie = match ctx.change {
            ChangeContext::Delete { start, end } => {
                let start_byte = old_br.row_start(start.row).ok_or(Error::OutOfBoundsRow {
                    max: ctx.breaklines.row_count().get() - 1,
                    current: start.row,
                })? + start.col;
                let end_byte = old_br.row_start(end.row).ok_or(Error::OutOfBoundsRow {
                    max: ctx.breaklines.row_count().get() - 1,
                    current: end.row,
                })? + end.col;

                let is_ws = ctx.old_str[start_byte..end_byte]
                    .chars()
                    .all(|c| c.is_whitespace());

                (
                    InputEdit {
                        start_position: grid_into_point(start),
                        old_end_position: grid_into_point(end),
                        new_end_position: grid_into_point(start),
                        start_byte,
                        old_end_byte: end_byte,
                        new_end_byte: start_byte,
                    },
                    is_ws,
                )
            }
            ChangeContext::Insert {
                inserted_br_indexes,
                position,
                text,
            } => {
                let start_byte = old_br
                    .row_start(position.row)
                    .ok_or(Error::OutOfBoundsRow {
                        max: ctx.breaklines.row_count().get() - 1,
                        current: position.row,
                    })?
                    + position.col;
                let new_end_byte = start_byte + text.len();

                let is_ws = text.chars().all(|c| c.is_whitespace());

                (
                    InputEdit {
                        start_byte,
                        old_end_byte: start_byte,
                        new_end_byte,
                        start_position: grid_into_point(position),
                        old_end_position: grid_into_point(position),
                        new_end_position: Point {
                            row: position.row + inserted_br_indexes.len(),
                            // -1 because bri includes the breakline
                            column: inserted_br_indexes
                                .last()
                                .map(|bri| text.len() - (bri - start_byte) - 1)
                                .unwrap_or(text.len() + position.col),
                        },
                    },
                    is_ws,
                )
            }
            ChangeContext::Replace {
                start,
                end,
                text,
                inserted_br_indexes,
            } => {
                let row_count = ctx.breaklines.row_count();
                let start_byte = old_br.row_start(start.row).ok_or(Error::OutOfBoundsRow {
                    max: row_count.get() - 1,
                    current: start.row,
                })? + start.col;
                let old_end_byte = old_br.row_start(end.row).ok_or(Error::OutOfBoundsRow {
                    max: row_count.get() - 1,
                    current: end.row,
                })? + end.col;

                let is_ws = text.chars().all(|c| c.is_whitespace());

                (
                    InputEdit {
                        start_byte,
                        start_position: grid_into_point(start),
                        old_end_position: grid_into_point(end),
                        old_end_byte,
                        new_end_byte: start_byte + text.len(),
                        new_end_position: {
                            if let [.., last] = inserted_br_indexes {
                                Point {
                                    row: start.row + inserted_br_indexes.len(),
                                    // -1 because last includes the breakline
                                    column: text.len() - (last - start_byte) - 1,
                                }
                            } else {
                                Point {
                                    row: start.row,
                                    column: start.col + text.len(),
                                }
                            }
                        },
                    },
                    is_ws,
                )
            }
            ChangeContext::ReplaceFull { text } => (
                InputEdit {
                    start_byte: 0,
                    old_end_byte: ctx.old_str.len(),
                    new_end_byte: text.len(),
                    start_position: Point { row: 0, column: 0 },
                    old_end_position: Point {
                        row: old_br.row_count().get() - 1,
                        column: ctx.old_str.len() - old_br.last_row_start(),
                    },
                    new_end_position: Point {
                        row: new_br.row_count().get() - 1,
                        column: text.len() - new_br.last_row_start(),
                    },
                },
                false,
            ),
        };
        Ok(ie)
    }
}
