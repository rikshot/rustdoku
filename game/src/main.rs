use gloo_events::EventListener;

use rustdoku_sudoku::{generator, grid::Grid, solver::alx_solve};
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::{prelude::*, utils::document};

enum Msg {
    Clear,
    Solve,
    Generate,
    Select(usize),
    KeyDown(String),
    Copy,
    Givens(usize),
}

struct Model {
    link: ComponentLink<Self>,
    grid: Grid,
    givens: usize,
    selected: Option<usize>,
    #[allow(dead_code)]
    key_listener: EventListener,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let key_listener = {
            let link = link.clone();
            EventListener::new(&document(), "keydown", move |event| {
                let event: &KeyboardEvent = event.unchecked_ref();
                link.send_message(Msg::KeyDown(event.key()))
            })
        };

        Self {
            link,
            grid: Grid::new(),
            givens: 28,
            selected: None,
            key_listener,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clear => {
                self.grid = Grid::new();
                true
            }
            Msg::Solve => {
                let solutions = alx_solve(&self.grid, 0);
                if solutions.is_empty() {
                    gloo_dialogs::alert("No solution found");
                    false
                } else if solutions.len() == 1 {
                    log::info!("{}", solutions[0]);
                    self.grid = solutions[0];
                    true
                } else {
                    gloo_dialogs::alert("Multiple solutions found");
                    false
                }
            }
            Msg::Generate => {
                self.selected = None;
                self.grid = generator::generate(self.givens);
                true
            }
            Msg::Select(index) => {
                if !self.grid.get(index).frozen() {
                    self.selected = Some(index);
                    return true;
                }
                false
            }
            Msg::KeyDown(key) => {
                if let Ok(digit) = key.parse::<u8>() {
                    if let Some(selected) = self.selected {
                        self.grid.set(selected, digit);
                        return true;
                    }
                } else if key == "Escape" {
                    self.selected = None;
                    return true;
                }
                false
            }
            #[allow(unused_must_use)]
            Msg::Copy => {
                let clipboard = window().unwrap().navigator().clipboard().unwrap();
                clipboard.write_text(&format!("{}", self.grid));
                false
            }
            Msg::Givens(givens) => {
                self.givens = givens;
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cells = self.grid.cells().iter().enumerate().map(|(index,cell)| {
            let value = char::from_digit(cell.value().into(), 10).unwrap();
            let value = if value == '0' { "".to_owned() } else { value.to_string() };
            let selected = if self.selected.is_some() && self.selected.unwrap() == index {
                Some("selected")
            } else {
                None
            };
            let onclick = self.link.callback(move |_| Msg::Select(index));
            html! { <div class=classes!("cell", selected) onclick={onclick}>{value}</div> }
        });

        html! {
            <main>
                <section>
                    <h1>{"Sudoku"}</h1>
                    <div>
                        <button onclick=self.link.callback(|_| Msg::Clear)>{"Clear"}</button>
                        <button onclick=self.link.callback(|_| Msg
                        ::Solve)>{"Solve"}</button>
                        <button onclick=self.link.callback(|_| Msg
                        ::Generate)>{"Generate"}</button>
                        <input type="number" value={self.givens.to_string()} size=2 min=17 max=81 oninput=self.link.callback(|event: InputData| Msg::Givens(event.value.parse::<usize>().unwrap())) />
                    </div>
                    <div>
                        <button onclick=self.link.callback(|_| Msg::Copy)>{"Copy"}</button>
                    </div>
                </section>
                <section class="grid">
                    { for cells }
                </section>
            </main>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
