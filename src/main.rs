mod ou_process;
mod control;
mod interventions;
mod aquarium;
mod recorder;

use yew::prelude::*;
use yew_hooks::use_interval;
use control::*;
use interventions::*;
use aquarium::*;
use recorder::*;
use ou_process::*;

#[function_component(App)]
fn app() -> Html {
    let ou_status = use_state(|| OUState::default());
    let ou_history = use_state(|| Recorder::new(OUState::default()));

    use_interval(
        {
            let ou_status = ou_status.clone();
            move || {
                // the type of ou_status is UseStateHandle<OUState>, which derefs to OUState
                // the function `update_ou_state` and `record` takes &OUState as input
                // so there's a deref + ref to get &OUState
                ou_status.set(update_ou_state(&*ou_status));
                ou_history.set(ou_history.record(&*ou_status));
            }
        },
        100,
    );


    html! {
        <div>
            <h1 class="text-2xl font-bold mb-4">{ "Causal Aquarium" }</h1>

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

                <Aquarium var={ou_status.var.clone()} />

                <ParamCtrl config={ou_status.config.clone()} on_update={
                    {
                        let ou_status = ou_status.clone();
                        Callback::from(move |new_config: Config| {
                            let mut new_state = (*ou_status).clone();
                            new_state.config = new_config;
                            ou_status.set(new_state);
                        })
                    }
                } />

                <GammaCtrl config={ou_status.config.clone()} on_update={
                    {
                        let ou_status = ou_status.clone();
                        Callback::from(move |new_config: Config| {
                            let mut new_state = (*ou_status).clone();
                            new_state.config = new_config;
                            ou_status.set(new_state);
                        })
                    }
                } />

                <Interventions var={ou_status.var.clone()} on_update={
                    {
                        let ou_status = ou_status.clone();
                        Callback::from(move |new_intervention: Intervention| {
                            let mut new_state = (*ou_status).clone();
                            new_state.intervention = new_intervention;
                            ou_status.set(new_state);
                        })
                    }
                } />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}