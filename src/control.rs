/// this file contains the components for controling the param of OU process
/// 
/// # components
/// 
/// * `ParamCtrl`: 3 texts controling sigma(sd^2), theta, delta_t of OU process
/// * `GammaCtrl`: 9-cells table controling the interaction matrix of OU process
/// 
/// last changed: Lin Jinghang, 4 October 2025

use yew::prelude::*;
use crate::ou_process::Config;

#[derive(Properties, PartialEq, Clone)]
pub struct ParamCtrlProps {
    pub config: Config,
    pub on_update: Callback<Config>,
}

#[function_component(ParamCtrl)]
pub fn param_ctrl(props: &ParamCtrlProps) -> Html {
    let oninput = {
        let config = props.config.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<f64>() {
                let new_config = Config { sigma: value, ..config.clone() };
                on_update.emit(new_config);
            }
        })
    };


    html! {
        <div class="mb-4">
            <label class="block font-semibold mb-1">{ "sigma" }</label>
            <input
                type="number"
                step="0.1"
                class="p-2 border rounded-md w-full"
                value={props.config.sigma.to_string()}
                {oninput}
            />
        </div>
    }
}