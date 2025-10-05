mod ou_process;
use yew::prelude::*;
use yew_hooks::use_interval;
use rand::{thread_rng, Rng};
use rand_distr::{Normal, Distribution};

#[function_component(ProgressBar)]
fn progress_bar() -> Html {
    let ou_status = use_state(|| ou_process::OUState::default());

    use_interval(
        {
            let ou_status = ou_status.clone();
            move || {
                // the type of ou_status is UseStateHandle<OUState>, which derefs to OUState
                // the function `update_ou_state` takes &OUState as input
                // so there's a deref + ref to get &OUState
                ou_status.set(ou_process::update_ou_state(&*ou_status));
            }
        },
        100,
    );


    html! {
        <div style="width: 100%; padding: 20px;">
            <div style="width: 100%; background-color: #f3f3f3; height: 30px; border-radius: 5px;">
                <div style={format!("width: {}%; height: 100%; background-color: #4caf50; border-radius: 5px;", ou_status.var.val[0])}>
                </div>
            </div>
            <div style="width: 100%; background-color: #f3f3f3; height: 30px; border-radius: 5px;">
                <div style={format!("width: {}%; height: 100%; background-color: #af4c4cff; border-radius: 5px;", ou_status.var.val[1])}>
                </div>
            </div>
            <div style="width: 100%; background-color: #f3f3f3; height: 30px; border-radius: 5px;">
                <div style={format!("width: {}%; height: 100%; background-color: #4c54afff; border-radius: 5px;", ou_status.var.val[2])}>
                </div>
            </div>
            
            <p>{format!("A: {:.0}%, B: {:.0}%, C: {:.0}%", ou_status.var.val[0], ou_status.var.val[1], ou_status.var.val[2])}</p>
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