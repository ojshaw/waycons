use crate::parse::parse;
use crate::Conway;
use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct ConwayProps {
    pub input: String,
}

#[component]
pub fn App(props: &ConwayProps) -> impl Into<AnyElement<'static>> {
    let conway = parse(&props.input).unwrap();
    let rows = conway.cells.chunks_exact(conway.cols);
    element! {
        Box() {
            Text(content: "Hello world")
            #(rows.map(|row| {
                row.iter().map(|cell| {
                    element! {
                        Box(background_color: if *cell {Color::Green} else {Color::Red}, width: 2, height: 2, border_color: Color::Black, border_style: BorderStyle::Round, margin_right: 2)
                    }
                })
            }).into_iter().flatten())
        }
    }
}
