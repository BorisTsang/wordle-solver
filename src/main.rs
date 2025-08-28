use leptos::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
enum TileStatus {
    #[default]
    Neutral,
    Absent,
    Misplaced,
    Correct,
}

fn main() {
    mount_to_body(App);
}

#[component]
fn Tile(#[prop(into)] status: Signal<TileStatus>) -> impl IntoView {
    view! {
        <div
            class="tile"
            class:neutral=move || status.get() == TileStatus::Neutral
            class:absent=move || status.get() == TileStatus::Absent
            class:misplaced=move || status.get() == TileStatus::Misplaced
            class:correct=move || status.get() == TileStatus::Correct
        ></div>
    }
}
#[component]
fn App() -> impl IntoView {
    let grid_state = RwSignal::new(
        (0..6)
            .map(|_| {
                (0..5)
                    .map(|_| RwSignal::new(TileStatus::default()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    let show_overlay = RwSignal::new(false);

    view! {
        <div class="container">
            <div>
                <div class="wordle-grid" id="wordleGrid">
                    {move || grid_state.get().into_iter().map(|row| {
                        view! {
                            <div class="row">
                                {row.into_iter().map(|cell| {
                                    view! {
                                        <div on:click=move |_| show_overlay.set(true)>
                                            <Tile status=cell.read_only() />
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }
                    }).collect_view()}
                </div>
                <div class="suggestions" id="suggestions">
                    <div class="input-section">
                        <input type="text" id="guessInput" maxlength="5" placeholder="Enter guess..."/>
                        <button id="guessSubmit">">"</button>
                    </div>
                    <h3>"Suggested Words"</h3>
                    <div class="suggested-words" id="suggestedWords">
                        <div class="suggestion-item">"CRANE"</div>
                        <div class="suggestion-item">"SLATE"</div>
                        <div class="suggestion-item">"AUDIO"</div>
                        <div class="suggestion-item">"TRACE"</div>
                    </div>
                </div>
            </div>
            <div
                class="control-overlay"
                id="controlOverlay"
                style:display=move || if show_overlay.get() { "flex" } else { "none" }
                on:click=move |_| show_overlay.set(false)
            >
                <div class="control-panel" id="controlPanel" on:click=|ev| ev.stop_propagation()>
                    <h3>"Mark Letters"</h3>
                    <div class="letter-selector" id="letterSelector"></div>
                    <div class="status-buttons">
                        <button class="status-button gray">"Gray"</button>
                        <button class="status-button yellow">"Yellow"</button>
                        <button class="status-button green">"Green"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}