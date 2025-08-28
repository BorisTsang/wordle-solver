use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! {
        <header>
            <h1>"Wordle Solver"</h1>
            <p>"Enter a 5-letter guess, then mark letters with colors, and get suggestions."</p>
        </header>
        <div class="container">
            <div>
                <div class="wordle-grid" id="wordleGrid">
                    { (0..6).map(|_| view! {
                        <div class="row">
                            { (0..5).map(|_| view! { <div class="tile"></div> }).collect_view() }
                        </div>
                    }).collect_view() }
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
            <div class="control-panel" id="controlPanel" style="display: none">
                <h3>"Mark Letters"</h3>
                <div class="letter-selector" id="letterSelector"></div>
                <div class="status-buttons">
                    <button class="status-button gray">"Gray"</button>
                    <button class="status-button yellow">"Yellow"</button>
                    <button class="status-button green">"Green"</button>
                </div>
            </div>
        </div>
    })
}