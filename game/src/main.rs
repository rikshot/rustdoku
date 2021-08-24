use rustdoku_sudoku::{generator, grid::Grid, solver::alx_solve};

use web_sys::window;
use yew::prelude::*;

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
    InputType(InputType),
    ValueChange(usize),
    CandidateToggle(usize)
}

#[derive(PartialEq)]
enum InputType {
    Values,
    Candidates,
}

struct Model {
    link: ComponentLink<Self>,
    grid: Grid,
    givens: usize,
    candidates: bool,
    input_type: InputType,
    selected: Option<usize>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let default_givens = 28;

        Self {
            link,
            grid: generator::generate(default_givens),
            givens: default_givens,
            candidates: false,
            selected: None,
            input_type: InputType::Values,
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
            Msg::Candidates(data) => match data {
                ChangeData::Value(string) => {
                    self.candidates = string == "false";
                    true
                }
                _ => false,
            },
            Msg::InputType(input_type) => {
                self.input_type = input_type;
                true
            }
            Msg::ValueChange(value) => {
                if let Some(selected) = self.selected {
                    self.grid.set_checked(selected, value as u8);
                    return true;
                }
                false
            }
            Msg::CandidateToggle(candidate) => {
                if let Some(selected) = self.selected {
                    self.grid.candidates_mut(selected).toggle(candidate - 1);
                    return true;
                }
                false
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
            let onfocus = self.link.callback(move |_| Msg::Select(index));
            let onkeydown = self.link.callback(|event: KeyboardEvent| {
                Msg::KeyDown(event.key())
            });

            if cell > 0 {
                let value = char::from_digit(cell.into(), 10).unwrap().to_string();
                html! { <div tabindex=0 role="button" aria-label={index.to_string()} class=classes!("cell", selected) onclick={onclick} onfocus={onfocus} onkeydown={onkeydown}>{value}</div> }
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
                    <div tabindex=0 role="button" aria-label={index.to_string()} class=classes!("cell", selected) onclick={onclick} onfocus={onfocus} onkeydown={onkeydown}>
                        <div class="candidates">{ for candidates }</div>
                    </div>
                }
            } else {
                html! { <div tabindex=0 role="button" aria-label={index.to_string()} class=classes!("cell", selected) onclick={onclick} onfocus={onfocus} onkeydown={onkeydown}></div> }
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
                            <input type="number" id="givens" value={self.givens.to_string()} size=2 min=17 max=81 oninput=self.link.callback(|event: InputData| Msg::Givens(event.value.parse::<usize>().unwrap())) />
                            <label for="givens">{"Givens"}</label>
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
                <section id="grid">
                    { for cells }
                </section>
                <section id="inputs">
                    <div id="input_type">
                        <input type="radio" id="value_input" name="input_type" value="values" checked={self.input_type == InputType::Values} onchange=self.link.callback(|_| Msg::InputType(InputType::Values)) />
                        <label for="value_input">{"Value"}</label>
                        <input type="radio" id="candidate_input" name="input_type" value="candidates" checked={self.input_type == InputType::Candidates} onchange=self.link.callback(|_| Msg::InputType(InputType::Candidates)) />
                        <label for="candidate_input">{"Candidate"}</label>
                    </div>
                    <div id="value_inputs" class={ if self.input_type == InputType::Values { classes!("active") } else { classes!() }}>
                        { for (1..=9_usize).map(|i| {
                            let is = char::from_digit(i as u32, 10).unwrap().to_string();
                            let id = format!("value_{}", i);
                            let checked = if let Some(selected) = self.selected {
                                self.grid.get(selected) == i as u8
                            } else {
                                false
                            };
                            html! { <>
                                <input type="radio" name="value" id={id.clone()} value={is.clone()} onchange=self.link.callback(move |_| Msg::ValueChange(i)) checked={checked} />
                                <label for={id.clone()}>{is.clone()}</label>
                            </> }
                        }) }
                        <input type="radio" name="value" id="value_0" value="0" onchange=self.link.callback(move |_| Msg::ValueChange(0)) checked={ if let Some(selected) = self.selected {
                            self.grid.get(selected) == 0
                        } else {
                            false
                        }}/>
                        <label for="value_0">{"0"}</label>
                    </div>
                    <div id="candidate_inputs" class={ if self.input_type == InputType::Candidates { classes!("active") } else { classes!() }}>
                        { for (1..=9_usize).map(|i| {
                            let is = char::from_digit(i as u32, 10).unwrap().to_string();
                            let id = format!("candidate_{}", i);
                            let checked = if let Some(selected) = self.selected {
                                self.grid.candidates(selected).get(i - 1)
                            } else {
                                false
                            };
                            html! { <>
                                <input type="checkbox" name="candidate" id={id.clone()} value={is.clone()} onchange=self.link.callback(move |_| Msg::CandidateToggle(i)) checked={checked} />
                                <label for={id.clone()}>{is.clone()}</label>
                            </> }
                        }) }
                    </div>
                </section>
            </main>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
