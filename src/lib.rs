use leptos::*;
use getrandom;
use std::time::Duration;
// turn off clippy on autosave/autosave

#[component]
pub fn MainGame(
    cx: Scope,
    #[prop(default=0)]
    initial_stage: i32,
    #[prop(default=0)]
    initial_round: i32,
) -> impl IntoView {
    let mut rxn_times = [0u128;5];
    let (average_time, set_average_time) = create_signal(cx, 0_u128);

    fn get_random_buf() -> Result<[u8; 1], getrandom::Error> {
        let mut buf = [0u8];
        getrandom::getrandom(&mut buf)?;
        Ok(buf)
    }

    let starting_time = instant::Instant::now();
    let (initial_time, set_initial) = create_signal(cx, starting_time);
    let (time_diff, set_diff) = create_signal(cx, Duration::new(0, 0).as_millis());

    let (wait_length, set_wait_length) = create_signal(cx, 0.0);
    
    // What stage we are on based on the rxn_time flowchart
    let (stage, set_stage) = create_signal(cx, initial_stage);

    // Round x of 5 
    let (round, set_round) = create_signal(cx, initial_round);

    let commence_round = move || {
        set_stage(2);
        set_initial(instant::Instant::now())
    };

    let mut finish_round = move || {
        set_diff(initial_time.get().elapsed().as_millis());
        rxn_times[round() as usize -1] = time_diff();
        set_stage(3);
        log!("{:?}", rxn_times);
        log!("{}", rxn_times.iter().sum::<u128>()/5);
        set_average_time(rxn_times.iter().sum::<u128>()/5);
    };

    let increment_round = move |_| {
        log!("Stage: {}; Round {}", stage(), round());

        set_round.update(|round| *round += 1);
        if round() == 6 {
            set_stage(4)
        } else {
            set_stage(1);
            set_wait_length((((get_random_buf().unwrap()[0] as f32 * 400.0) / 256.0) /100.0) + 0.5);
            log!("Wait Length: {}", wait_length());
            set_timeout(commence_round, Duration::new(wait_length() as u64, 0));
        }
    };

    view! {cx,
        <div 
            class="main-game"
            style=move || String::from("background-color:") + match stage() {
                0 | 4 => "#9300FF",
                1 => "#FF0000",
                2 | 3 => "#6CFF00",
                _ => "white"
                }
            on:click = move |_| if stage() == 2 { finish_round() }>
            {move || match stage() {
                0 | 3 => {
                    view! {cx,
                        <div class="instruction-page">
                            <span class:no-display = move || stage() == 3>
                                "Click when the screen turns from red to green"
                            </span>
                            <span class:no-display = move || stage() == 0>
                                {time_diff} " ms"
                            </span>
                            <button 
                                class="ready-button"
                                on:click=increment_round>
                                "Ready?"
                            </button>
                        </div>
                    }.into_any()},
                1 => {
                    view! {cx,
                    <span> "Click When Green!" </span>}.into_any()
                },
                2 => {
                    view! {cx,
                    <span> "Click!" </span>}.into_any()
                },
                4 => {
                    view! {cx,
                    <span>
                        "Average Reaction Speed:"
                        <br/>
                        {average_time} " ms"
                    </span>}.into_any()
                },
                _ => {
                    view! {cx,
                        <div style="display=none"></div>}.into_any()
                }
            }}
        </div>
    }}