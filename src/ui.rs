use crate::parse::parse;
use iocraft::prelude::*;

#[derive(Debug, Default, Props)]
pub struct ConwayProps {
    pub input: String,
}

const STR: &'static str = "";

#[component]
pub fn App(mut hooks: Hooks, props: &ConwayProps) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let conway = parse(&props.input).unwrap();
    let rows = conway.cells.chunks_exact(conway.cols);

    element! {
        View(width: width -1, height: height, flex_direction: FlexDirection::Column, align_items: AlignItems::Center, padding: Padding::Length(0), margin: Margin::Length(0)) {
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

const CELL_HEIGHT: u16 = 2;

#[component]
pub fn ConwayRow(props: &ConwayRowProps) -> impl Into<AnyElement<'static>> {
    let row = &props.row[..];
    let colors = [Color::Red, Color::Green];
    // let cell_width_pct: f32 = 100.0 / row.len() as f32;

    let w = CELL_HEIGHT * 2;
    let h = CELL_HEIGHT;

    element! {
        View(
            height: h + 4,
            flex_direction: FlexDirection::Row,
            padding: Padding::Length(0),
            margin: Margin::Length(0)) {
            #(row.into_iter().map(|cell| element!{
                View(padding: Padding::Length(0), border_style: BorderStyle::Single, width: w + 2, height: h + 2){
                View(
                    width: w,
                    height: h,
                    background_color: colors[*cell as usize],

                    // padding: Padding::Length(1),
                ) { }
            }
            }).into_iter())
        }
    }
}
