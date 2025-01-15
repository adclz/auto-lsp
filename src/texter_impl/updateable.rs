use texter::change::GridIndex;
use texter::error::Error;
use texter::updateables::{ChangeContext, UpdateContext, Updateable};
use tree_sitter::{InputEdit, Point, Tree};

pub struct WrapTree<'a> {
    pub tree: &'a mut Tree,
    edits: Option<Vec<(InputEdit, bool)>>,
}

impl<'a> From<&'a mut Tree> for WrapTree<'a> {
    fn from(tree: &'a mut Tree) -> Self {
        Self { tree, edits: None }
    }
}

impl Updateable for WrapTree<'_> {
    fn update(&mut self, ctx: UpdateContext) -> Result<(), Error> {
        let new_edits = WrapTree::edit_from_ctx(ctx)?;
        self.tree.edit(&new_edits.0);
        match self.edits {
            Some(ref mut edits) => edits.push(new_edits),
            None => self.edits = Some(vec![new_edits]),
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

impl WrapTree<'_> {
    pub(crate) fn get_edits(&mut self) -> Vec<(InputEdit, bool)> {
        self.edits.take().unwrap_or_default()
    }

    fn edit_from_ctx(ctx: UpdateContext) -> anyhow::Result<(InputEdit, bool), Error> {
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
