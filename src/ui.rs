use crate::parse::parse;
use iocraft::prelude::*;
use std::time::Duration;

#[derive(Debug, Default, Props)]
pub struct ConwayProps {
    pub input: String,
}

const STR: &'static str = "";

#[component]
pub fn App(mut hooks: Hooks, props: &ConwayProps) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();

    let mut conway = hooks.use_state(|| parse(&props.input).unwrap());
    let mut offset = hooks.use_state(|| 0); 

    hooks.use_terminal_events(
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind,.. }) if kind != KeyEventKind::Release => {
                match code {
                    // KeyCode::Char('q') => should_exit.set(true), 
                    KeyCode::Up => offset.set((offset.get() - 1).max(0)),
                    KeyCode::Down => offset.set(offset.get() + 1),
                    _ => {}
                }
            }
            TerminalEvent::FullscreenMouse(FullscreenMouseEvent {kind, ..})  => {
                match kind {
                    MouseEventKind::ScrollDown => offset.set(offset.get() + 1),
                    MouseEventKind::ScrollUp =>  offset.set((offset.get() - 1).max(0)),
                    _ => {}
                }
            }
            _ => {}
        }
    );

    hooks.use_future(async move {
        loop {
            smol::Timer::after(Duration::from_millis(500)).await;
            let new = conway.read().update();
            *conway.write() = new;
        }
    });
    // let rows = conway.cells.chunks_exact(conway.cols);

    element! {
        View(width: width -1, height: height, flex_direction: FlexDirection::Column, align_items: AlignItems::Center, padding: Padding::Length(0), margin: Margin::Length(0)) {
            Text(content: "Hello world")
            Text(content: "Use arrow keys to scroll. Press \"q\" to exit. (Under construction)")
            View( width: width -1, height: height - 2 ) {
                View(position: Position::Absolute, height: height * 2, top: -offset.get()) {
                    #(conway.read().cells.chunks_exact(conway.read().cols).map(|row| element!{
                        ConwayRow(row: row.clone())
                    }).into_iter())
                }
            }
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
