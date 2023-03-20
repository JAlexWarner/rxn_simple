use leptos::*;
// turn off clippy on autosave/autosave

#[component]
pub fn MainGame(
    cx: Scope,
    #[prop(default=0)]
    initial_stage: i32,
    #[prop(default=0)]
    initial_round: i32
) -> impl IntoView {
    // let starting_time = instant::Instant::now();
    // let (initial_time, set_initial) = create_signal(cx, starting_time);
    // let (time_diff, set_diff) = create_signal(cx, 0);

    // let update_times = move |_| {
    //     let duration = initial_time.get().elapsed();
    //     set_diff(duration.as_millis());
    //     set_initial(instant::Instant::now());
    // };

    let (stage, set_stage) = create_signal(cx, initial_stage);
    let (round, set_round) = create_signal(cx, initial_round);

    view! {cx,
        <div 
            class="rxn-button"
            // style=String::from("background-color:") + match stage() {
            //     0 | 4 => "#9300FF",
            //     1 => "#FF0000",
            //     2 | 3 => "#6CFF00",
            //     _ => "white"
            //     }>
            style=move || String::from("background-color:") + match stage() {
                0 | 4 => "#9300FF",
                1 => "#FF0000",
                2 | 3 => "#6CFF00",
                _ => "white"
                }>
            {move || match stage() {
                0 | 3 => {
                    view! {cx,
                        <div class="instruction-page">
                            <span>
                                "Instructions:" <br/> "Click when the screen turns green"
                            </span>
                            <button 
                                class="ready-button"
                                on:click=move |_| set_stage(1)>
                                "Ready?"
                            </button>
                        </div>
                    }.into_any()},
                _ => {
                    view! {cx,
                        <div
                            style="display=none"></div>}.into_any()
                }
            }}
        </div>
        <p>{stage}</p>
    }}