use yew::prelude::*;
use yew_hooks::use_interval;
use rand::{thread_rng, Rng};
use rand_distr::{Normal, Distribution};

#[function_component(ProgressBar)]
fn progress_bar() -> Html {
    let progress = use_state(|| 0.0);
    let normal = Normal::new(1.0, 0.3).unwrap();
    let mut rng = thread_rng();

    use_interval(
        {
            let progress = progress.clone();
            move || {
                let new_progress = if *progress < 100.0 {
                    *progress + normal.sample(&mut rng)
                } else {
                    *progress
                };
                progress.set(new_progress);
            }
        },
        100,
    );


    html! {
        <div style="width: 100%; padding: 20px;">
            <div style="width: 100%; background-color: #f3f3f3; height: 30px; border-radius: 5px;">
                <div style={format!("width: {}%; height: 100%; background-color: #4caf50; border-radius: 5px;", *progress)}>
                </div>
            </div>
            <p>{format!("Progress: {:.0}%", *progress)}</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Progress Bar Example" }</h1>
            <ProgressBar />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}