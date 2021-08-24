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
    Import,
    Export,
    Givens(usize),
    Candidates(ChangeData),
}

struct Model {
    link: ComponentLink<Self>,
    grid: Grid,
    givens: usize,
    candidates: bool,
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

        let default_givens = 28;

        Self {
            link,
            grid: generator::generate(default_givens),
            givens: default_givens,
            candidates: false,
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
                let solutions = alx_solve(&self.grid, 2);
                if solutions.is_empty() {
                    gloo_dialogs::alert("No solution found");
                    false
                } else if solutions.len() == 1 {
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
                if !self.grid.frozen(index) {
                    self.selected = Some(index);
                    return true;
                }
                false
            }
            Msg::KeyDown(key) => {
                if let Ok(digit) = key.parse::<u8>() {
                    if let Some(selected) = self.selected {
                        self.grid.set_checked(selected, digit);
                        return true;
                    }
                } else if key == "Escape" {
                    self.selected = None;
                    return true;
                }
                false
            }
            Msg::Import => {
                if let Some(sudoku) = gloo_dialogs::prompt("Insert sudoku in 00001002... format", None) {
                    match sudoku.parse() {
                        Ok(sudoku) => {
                            self.grid = sudoku;
                            return true;
                        }
                        Err(error) => gloo_dialogs::alert(&format!("{}", error)),
                    }
                }
                false
            }
            #[allow(unused_must_use)]
            Msg::Export => {
                let clipboard = window().unwrap().navigator().clipboard().unwrap();
                clipboard.write_text(&format!("{}", self.grid));
                gloo_dialogs::alert("Sudoku exported to clipboard");
                false
            }
            Msg::Givens(givens) => {
                self.givens = givens;
                false
            }
            Msg::Candidates(data) => {
                match data {
                    ChangeData::Value(string) => {
                        self.candidates = string == "false";
                        true
                    },
                    _ => false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cells = self.grid.cells().enumerate().map(|(index, cell)| {
            let selected = if self.selected.is_some() && self.selected.unwrap() == index {
                Some("selected")
            } else {
                None
            };
            let onclick = self.link.callback(move |_| Msg::Select(index));

            if cell > 0 {
                let value = char::from_digit(cell.into(), 10).unwrap().to_string();
                html! { <div class=classes!("cell", selected) onclick={onclick}>{value}</div> }
            } else if self.candidates {
                let candidates = self.grid.candidates(index);
                let candidates = (0..9).map(|candidate| {
                    if candidates.get(candidate) {
                        html! { <div class="candidate">{candidate + 1}</div> }
                    } else {
                        html! { <div class="candidate"></div> }
                    }
                });
                html! {
                    <div class=classes!("cell", selected) onclick={onclick}>
                        <div class="candidates">{ for candidates }</div>
                    </div>
                }
            } else {
                html! { <div class=classes!("cell", selected) onclick={onclick}></div> }
            }
        });

        html! {
            <main>
                <header>
                    <h1>{"Rustdoku"}</h1>
                    <div id="controls">
                        <div>
                            <button onclick=self.link.callback(|_| Msg::Clear)>{"Clear"}</button>
                            <button onclick=self.link.callback(|_| Msg
                            ::Solve)>{"Solve"}</button>
                            <button onclick=self.link.callback(|_| Msg
                            ::Generate)>{"Generate"}</button>
                            <input type="number" value={self.givens.to_string()} size=2 min=17 max=81 oninput=self.link.callback(|event: InputData| Msg::Givens(event.value.parse::<usize>().unwrap())) />
                        </div>
                        <div>
                            <input type="checkbox" id="candidates" checked={self.candidates} onchange=self.link.callback(Msg::Candidates) value={self.candidates.to_string()} />
                            <label for="candidates">{"Candidates"}</label>
                        </div>
                        <div>
                            <button onclick=self.link.callback(|_| Msg::Import)>{"Import"}</button>
                            <button onclick=self.link.callback(|_| Msg::Export)>{"Export"}</button>
                        </div>
                    </div>
                </header>
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
