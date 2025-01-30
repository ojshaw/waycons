use crate::parse::parse;
use crate::Conway;
use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct ConwayProps {
    pub input: String,
}

#[component]
pub fn App(mut hooks: Hooks, props: &ConwayProps) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let conway = parse(&props.input).unwrap();
    let rows = conway.cells.chunks_exact(conway.cols);
    element! {
        View(width: width -1, height: height, flex_direction: FlexDirection::Column, align_items: AlignItems::Center) {
            Text(content: "Hello world")
            #(rows.map(|row| element!{
                ConwayRow(row: row.clone())
            }).into_iter())
        }
    }
}

#[derive(Default, Props)]
pub struct ConwayRowProps {
    pub row: Vec<bool>,
}

const CELL_HEIGHT: u16 = 3;
const ROW_HEIGHT: u16 = 3;

#[component]
pub fn ConwayRow(props: &ConwayRowProps) -> impl Into<AnyElement<'static>> {
    let row = &props.row[..];
    let colors = [Color::Red, Color::Green];
    let cell_width_pct: f32 = 100.0 / row.len() as f32;

    element! {
        View(height: ROW_HEIGHT, flex_direction: FlexDirection::Row) {
            #(row.into_iter().map(|cell| element!{
                View(
                    width: CELL_HEIGHT,
                    height: CELL_HEIGHT,
                    background_color: colors[*cell as usize],
                    border_style: BorderStyle::Single,
                ) { }
            }).into_iter())
        }
    }
}
