use std::str::FromStr;

use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Event, HtmlInputElement, KeyboardEvent};

use rustdoku_sudoku::{candidates::Candidates, generator, grid::Grid, solver::alx_solve};

#[derive(PartialEq)]
enum InputType {
    Values,
    Candidates,
}

fn parse_event_value<T: FromStr, E: JsCast, F: Fn(&E) -> String>(
    event: &Event,
    extractor: F,
) -> Option<Result<T, T::Err>> {
    let target = event.target().and_then(|target| target.dyn_into::<E>().ok());
    if let Some(target) = target {
        let value = extractor(&target);
        Some(value.parse::<T>())
    } else {
        None
    }
}

#[derive(Prop)]
struct CellProps<'a> {
    grid: &'a Signal<Grid>,
    placemarks: &'a Signal<[Candidates; 81]>,
    assisted: &'a ReadSignal<bool>,
    index: usize,
    selected: &'a Signal<Option<usize>>,
    input_type: &'a ReadSignal<InputType>,
}

#[component]
fn Cell<'a, G: Html>(cx: Scope<'a>, props: CellProps<'a>) -> View<G> {
    let on_select = move |_event| {
        props.selected.set(Some(props.index));
    };

    let on_keydown = |event: Event| {
        let event: KeyboardEvent = event.unchecked_into();
        let key = event.key();
        if let Ok(digit) = key.parse::<u8>() {
            if let Some(selected) = *props.selected.get() {
                match *props.input_type.get() {
                    InputType::Values => {
                        let mut grid = *props.grid.get();
                        grid.set(selected, digit, *props.assisted.get());
                        props.grid.set(grid);
                    }
                    InputType::Candidates => {
                        if *props.assisted.get() {
                            let mut grid = *props.grid.get();
                            grid.candidates_mut(selected).toggle(digit as usize - 1);
                            props.grid.set(grid);
                        } else {
                            let mut placemarks = *props.placemarks.get();
                            placemarks[selected].toggle(digit as usize - 1);
                            props.placemarks.set(placemarks);
                        }
                    }
                };
            }
        } else if key == "Escape" {
            props.selected.set(None);
        }
    };

    let class = create_memo(cx, move || {
        if props.selected.get().is_some() && props.selected.get().unwrap() == props.index {
            "cell selected"
        } else {
            "cell"
        }
    });

    view! { cx,
        (if (*props.grid.get()).get(props.index) > 0 {
            let value = char::from_digit((*props.grid.get()).get(props.index).into(), 10)
                .unwrap()
                .to_string();
            view! { cx,
                div(
                    tabindex=0,
                    role="button",
                    aria-label=(props.index.to_string()),
                    class=(class),
                    on:click=on_select,
                    on:focus=on_select,
                    on:keydown=on_keydown) {
                    (value)
                }
            }
        } else {
            let candidates = if *props.assisted.get() {
                let grid = *props.grid.get();
                *grid.candidates(props.index)
            } else {
                props.placemarks.get()[props.index]
            };
            let candidates = View::new_fragment(
                (0..9)
                    .map(|candidate| {
                        if candidates.get(candidate) {
                            view! { cx, div(class="candidate") {(candidate + 1)} }
                        } else {
                            view! { cx, div(class="candidate") {} }
                        }
                    })
                    .collect(),
            );
            view! { cx,
                div(tabindex=0,
                    role="button",
                    aria-label=(props.index.to_string()),
                    class=(class),
                    on:click=on_select,
                    on:focus=on_select,
                    on:keydown=on_keydown) {
                    div(class="candidates") { (candidates) }
                }
            }
        })
    }
}

#[component]
fn Game<G: Html>(cx: Scope) -> View<G> {
    let default_givens = 28;
    let grid = create_signal(cx, generator::generate(default_givens));
    let placemarks = create_signal(cx, [Candidates::new(false); 81]);
    let selected: &Signal<Option<usize>> = create_signal(cx, None);
    let givens = create_signal(cx, default_givens);
    let assisted = create_signal(cx, false);
    let input_type = create_signal(cx, InputType::Values);

    let on_clear = |_event| {
        grid.set(Grid::new());
        placemarks.set([Candidates::new(false); 81]);
    };

    let on_check = |_event| {
        if grid.get().is_valid() {
            gloo_dialogs::alert("Current sudoku is valid!");
        } else {
            gloo_dialogs::alert("Current sudoku is invalid.");
        }
    };

    let on_solve = |_event| {
        let solutions = alx_solve(&grid.get(), 2);
        if solutions.is_empty() {
            gloo_dialogs::alert("No solution found");
        } else if solutions.len() == 1 {
            grid.set(solutions[0]);
        } else {
            gloo_dialogs::alert("Multiple solutions found");
        }
    };

    let on_generate = |_event| {
        selected.set(None);
        grid.set(generator::generate(*givens.get()));
        placemarks.set([Candidates::new(false); 81]);
    };

    let on_givens = |event: Event| {
        let value = parse_event_value(&event, HtmlInputElement::value);
        if let Some(Ok(value)) = value {
            givens.set(value);
        }
    };

    let on_assisted = |_event| {
        assisted.set(!*assisted.get());
    };

    let on_import = |_event| {
        if let Some(sudoku) = gloo_dialogs::prompt("Insert sudoku in 00001002... format", None) {
            match sudoku.parse() {
                Ok(sudoku) => {
                    grid.set(sudoku);
                }
                Err(error) => gloo_dialogs::alert(&format!("{}", error)),
            }
        }
    };

    #[allow(unused_must_use)]
    let on_export = |_event| {
        let clipboard = window().unwrap().navigator().clipboard().unwrap();
        clipboard.write_text(&format!("{}", *grid.get()));
        gloo_dialogs::alert("Sudoku exported to clipboard");
    };

    let on_value_changed = |value| {
        if let Some(selected) = *selected.get() {
            let mut temp = *grid.get();
            temp.set(selected, value as u8, *assisted.get());
            grid.set(temp);
        }
    };

    let on_candidate_changed = |candidate| {
        if let Some(selected) = *selected.get() {
            if *assisted.get() {
                let mut temp = *grid.get();
                temp.candidates_mut(selected).toggle(candidate - 1);
                grid.set(temp);
            } else {
                let mut temp = *placemarks.get();
                temp[selected].toggle(candidate - 1);
                placemarks.set(temp);
            }
        }
    };

    let cells = View::new_fragment(
        (0..81)
            .map(|index| {
                view! { cx,
                    Cell(
                        grid=grid,
                        placemarks=placemarks,
                        assisted=assisted,
                        index=index,
                        selected=selected,
                        input_type=input_type
                    )
                }
            })
            .collect(),
    );

    view! { cx,
        main {
            header {
                h1 {"Rustdoku"}
                div(id="controls") {
                    div {
                        button(on:click=on_clear) {"Clear"}
                        button(on:click=on_check) {"Check"}
                        button(on:click=on_solve) {"Solve"}
                        button(on:click=on_generate) {"Generate"}
                        input(
                            type="number",
                            id="givens",
                            value=*givens.get().to_string(),
                            size=2,
                            min=17,
                            max=81,
                            on:input=on_givens
                        )
                        label(for="givens") {"Givens"}
                    }
                    div {
                        input(type="checkbox", id="assisted", checked=*assisted.get(), on:change=on_assisted)
                        label(for="assisted") {"Assisted"}
                    }
                    div {
                        button(on:click=on_import) {"Import"}
                        button(on:click=on_export) {"Export"}
                    }
                }
            }
            section(id="grid") {
                (cells)
            }
            section(id="inputs") {
                div(id="input_type") {
                    input(
                        type="radio",
                        id="value_input",
                        name="input_type",
                        value="values",
                        checked=*input_type.get() == InputType::Values,
                        on:change=|_| input_type.set(InputType::Values)
                    )
                    label(for="value_input") {"Value"}
                    input(
                        type="radio",
                        id="candidate_input",
                        name="input_type",
                        value="candidates",
                        checked=*input_type.get() == InputType::Candidates,
                        on:change=|_| input_type.set(InputType::Candidates)
                    )
                    label(for="candidate_input") {"Candidate"}
                }
                div(id="value_inputs", class=(if *input_type.get() == InputType::Values { "active" } else { "" })) {
                    (View::new_fragment((1..=9_usize).map(|i| {
                        let checked = if let Some(selected) = *selected.get() {
                            (*grid.get()).get(selected) == i as u8
                        } else {
                            false
                        };
                        view! { cx,
                            input(
                                type="radio",
                                name="value",
                                id=format!("value_{}", i),
                                value=i,
                                on:change=move |_| on_value_changed(i),
                                checked=checked
                            )
                            label(for=format!("value_{}", i)) {(i)}
                        }
                    }).collect()))
                    input(
                        type="radio",
                        name="value",
                        id="value_0",
                        value="0",
                        on:change=move |_| on_value_changed(0),
                        checked=if let Some(selected) = *selected.get() {
                            (*grid.get()).get(selected) == 0
                        } else {
                            false
                        }
                    )
                    label(for="value_0") {"0"}
                }
                div(
                    id="candidate_inputs",
                    class=(if *input_type.get() == InputType::Candidates { "active" } else { "" })
                ) {
                    (View::new_fragment((1..=9_usize).map(|i| {
                        let checked = if let Some(selected) = *selected.get() {
                            if *assisted.get() {
                                (*grid.get()).candidates(selected).get(i - 1)
                            } else {
                                (*placemarks.get())[selected].get(i - 1)
                            }
                        } else {
                            false
                        };
                        view! { cx,
                            input(
                                type="checkbox",
                                name="candidate",
                                id=format!("candidate_{}", i),
                                value=i,
                                on:change=move |_| on_candidate_changed(i),
                                checked=checked
                            )
                            label(for=format!("candidate_{}", i)) {(i)}
                        }
                    }).collect()))
                }
            }
        }
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            Game
        }
    })
}
