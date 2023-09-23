use leptos::*;
use rustdoku_sudoku::{candidates::Candidates, generator, grid::Grid, solver::alx_solve};
use web_sys::KeyboardEvent;

#[derive(Copy, Clone, Debug, PartialEq)]
enum InputType {
    Values,
    Candidates,
}

#[derive(Copy, Clone, Debug)]
struct AppState {
    grid: RwSignal<Grid>,
    placemarks: RwSignal<[Candidates; 81]>,
    selected: RwSignal<Option<usize>>,
    givens: RwSignal<usize>,
    assisted: RwSignal<bool>,
    input_type: RwSignal<InputType>,
}

impl Default for AppState {
    fn default() -> Self {
        let default_givens = 28;
        Self {
            grid: create_rw_signal(generator::generate(default_givens)),
            placemarks: create_rw_signal([Candidates::new(false); 81]),
            selected: create_rw_signal(None),
            givens: create_rw_signal(default_givens),
            assisted: create_rw_signal(false),
            input_type: create_rw_signal(InputType::Values),
        }
    }
}

#[component]
fn Cell(index: usize) -> impl IntoView {
    let state = expect_context::<AppState>();

    let on_select = move || {
        state.selected.set(Some(index));
    };

    let on_keydown = move |event: KeyboardEvent| {
        let key = event.key();
        if let Ok(digit) = key.parse::<u8>() {
            if let Some(selected) = state.selected.get() {
                match state.input_type.get() {
                    InputType::Values => {
                        let mut grid = state.grid.get();
                        grid.set(selected, digit, state.assisted.get());
                        state.grid.set(grid);
                    }
                    InputType::Candidates => {
                        if state.assisted.get() {
                            let mut grid = state.grid.get();
                            grid.candidates_mut(selected).toggle(digit as usize - 1);
                            state.grid.set(grid);
                        } else {
                            let mut placemarks = state.placemarks.get();
                            placemarks[selected].toggle(digit as usize - 1);
                            state.placemarks.set(placemarks);
                        }
                    }
                };
            }
        } else if key == "Escape" {
            state.selected.set(None);
        }
    };

    let class = create_memo(move |_| {
        if state.selected.get() == Some(index) {
            "cell selected"
        } else {
            "cell"
        }
    });

    view! {
        {move || if (state.grid.get()).get(index) > 0 {
            let value = char::from_digit((state.grid.get()).get(index).into(), 10)
                .unwrap()
                .to_string();
            view! {
                <div
                    tabindex=0
                    role="button"
                    aria-label=(index.to_string())
                    class=(class)
                    on:click=move |_| on_select()
                    on:focus=move |_| on_select()
                    on:keydown=on_keydown
                >{value}</div>
            }
        } else {
            let candidates = if state.assisted.get() {
                *state.grid.get().candidates(index)
            } else {
                state.placemarks.get()[index]
            };
            let candidates =
                (0..9)
                    .map(|candidate| {
                        if candidates.get(candidate) {
                            view! { <div class="candidate">{candidate + 1}</div> }
                        } else {
                            view! { <div class="candidate"></div> }
                        }
                    })
                    .collect_view();
            view! {
                <div
                    tabindex=0
                    role="button"
                    aria-label=(index.to_string())
                    class=(class)
                    on:click=move |_| on_select()
                    on:focus=move |_| on_select()
                    on:keydown=on_keydown
                >
                    <div class="candidates">{candidates}</div>
                </div>
            }
        }}
    }
}

#[component]
fn App() -> impl IntoView {
    provide_context(AppState::default());
    let state = expect_context::<AppState>();

    let on_clear = move |_| {
        state.grid.set(Grid::new());
        state.placemarks.set([Candidates::new(false); 81]);
    };

    let on_check = move |_| {
        if state.grid.get().is_valid() {
            gloo_dialogs::alert("Current sudoku is valid!");
        } else {
            gloo_dialogs::alert("Current sudoku is invalid.");
        }
    };

    let on_solve = move |_| {
        let solutions = alx_solve(&state.grid.get(), 2);
        if solutions.is_empty() {
            gloo_dialogs::alert("No solution found");
        } else if solutions.len() == 1 {
            state.grid.set(solutions[0]);
        } else {
            gloo_dialogs::alert("Multiple solutions found");
        }
    };

    let on_generate = move |_| {
        state.selected.set(None);
        state.grid.set(generator::generate(state.givens.get()));
        state.placemarks.set([Candidates::new(false); 81]);
    };

    let on_givens = move |event| {
        state.givens.set(event_target_value(&event).parse().unwrap());
    };

    let on_assisted = move |_| {
        state.assisted.set(!state.assisted.get());
    };

    let on_import = move |_| {
        if let Some(sudoku) = gloo_dialogs::prompt("Insert sudoku in 00001002... format", None) {
            match sudoku.parse() {
                Ok(sudoku) => {
                    state.grid.set(sudoku);
                }
                Err(error) => gloo_dialogs::alert(&format!("{}", error)),
            }
        }
    };

    let on_export = move |_| {
        let clipboard = window().navigator().clipboard().unwrap();
        let _ = clipboard.write_text(&format!("{}", state.grid.get()));
        gloo_dialogs::alert("Sudoku exported to clipboard");
    };

    let on_value_changed = move |value| {
        if let Some(selected) = state.selected.get() {
            let mut temp = state.grid.get();
            temp.set(selected, value as u8, state.assisted.get());
            state.grid.set(temp);
        }
    };

    let on_candidate_changed = move |candidate: usize| {
        if let Some(selected) = state.selected.get() {
            if state.assisted.get() {
                let mut temp = state.grid.get();
                temp.candidates_mut(selected).toggle(candidate - 1);
                state.grid.set(temp);
            } else {
                let mut temp = state.placemarks.get();
                temp[selected].toggle(candidate - 1);
                state.placemarks.set(temp);
            }
        }
    };

    view! {
        <main>
            <header>
                <h1>Rustdoku</h1>
                <div class="controls">
                    <button on:click=on_clear>Clear</button>
                    <button on:click=on_check>Check</button>
                    <button on:click=on_solve>Solve</button>
                    <button on:click=on_generate>Generate</button>
                    <input type="number" id="givens" prop:value=move || state.givens.get() size=3 min=17 max=81 on:input=on_givens />
                    <label for="givens">Givens</label>
                </div>
                <div class="controls">
                    <input type="checkbox" id="assisted" prop:checked=move || state.assisted.get() on:change=on_assisted />
                    <label for="assisted">Assisted</label>
                </div>
                <div class="controls">
                    <button on:click=on_import>Import</button>
                    <button on:click=on_export>Export</button>
                </div>
            </header>
            <section id="grid">
                {(0..81)
                    .map(|index| {
                        view! {
                            <Cell index />
                        }
                    })
                    .collect_view()}
            </section>
            <section id="inputs">
                <div id="input_type">
                    <input
                        type="radio"
                        id="value_input"
                        name="input_type"
                        value="values"
                        prop:checked=move || state.input_type.get() == InputType::Values
                        on:change=move |_| state.input_type.set(InputType::Values)
                    />
                    <label for="value_input">Value</label>
                    <input
                        type="radio"
                        id="candidate_input"
                        name="input_type"
                        value="candidates"
                        prop:checked=move || state.input_type.get() == InputType::Candidates
                        on:change=move |_| state.input_type.set(InputType::Candidates)
                    />
                    <label for="candidate_input">Candidate</label>
                </div>
                <div id="value_inputs" class:active=move || state.input_type.get() == InputType::Values>
                    {move || (1..=9_usize).map(|i| {
                        let checked = if let Some(selected) = state.selected.get() {
                            (state.grid.get()).get(selected) == i as u8
                        } else {
                            false
                        };
                        view! {
                            <input
                                type="radio"
                                name="value"
                                id=format!("value_{}", i)
                                value=i
                                on:change=move |_| on_value_changed(i)
                                prop:checked=checked
                            />
                            <label for=format!("value_{}", i)>{i}</label>
                        }
                    }).collect_view()}
                    <input
                        type="radio"
                        name="value"
                        id="value_0"
                        value="0"
                        on:change=move |_| on_value_changed(0)
                        checked=move || if let Some(selected) = state.selected.get() {
                            (state.grid.get()).get(selected) == 0
                        } else {
                            false
                        }
                    />
                    <label for="value_0">0</label>
                </div>
                <div
                    id="candidate_inputs"
                    class:active=move || state.input_type.get() == InputType::Candidates
                >
                    {move || (1..=9_usize).map(|i| {
                        let checked = if let Some(selected) = state.selected.get() {
                            if state.assisted.get() {
                                (state.grid.get()).candidates(selected).get(i - 1)
                            } else {
                                (state.placemarks.get())[selected].get(i - 1)
                            }
                        } else {
                            false
                        };
                        view! {
                            <input
                                type="checkbox"
                                name="candidate"
                                id=format!("candidate_{}", i)
                                value=i
                                on:change=move |_| on_candidate_changed(i)
                                prop:checked=checked
                            />
                            <label for=format!("candidate_{}", i)>{i}</label>
                        }
                    }).collect_view()}
                </div>
            </section>
        </main>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
