/// this file contains the components for controling the parameters of OU process
/// 
/// # components
/// 
/// + `ParamCtrl`: 3 texts controling sigma(sd^2), theta, delta_t of OU process
/// + `GammaCtrl`: 9-cells table controling the interaction matrix of OU process
/// 
/// # TODO
/// 
/// [ ] this file contains ton of repetitive code, which could be optimized by macro or function to improve readability
/// [ ] add boundary check & non-number input check

use yew::prelude::*;
use crate::ou_process::Config;

#[derive(Properties, PartialEq, Clone)]
pub struct ParamCtrlProps {
    pub config: Config,
    pub on_update: Callback<Config>,
}

#[function_component(ParamCtrl)]
pub fn param_ctrl(props: &ParamCtrlProps) -> Html {

    let oninput_sigma = {
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

    let oninput_theta = {
        let config = props.config.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<f64>() {
                let new_config = Config { theta: value, ..config.clone() };
                on_update.emit(new_config);
            }
        })
    };

    let oninput_delta_t = {
        let config = props.config.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<f64>() {
                let new_config = Config { delta_t: value, ..config.clone() };
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
                oninput={oninput_sigma}
            />

            <label class="block font-semibold mb-1">{ "theta" }</label>
            <input
                type="number"
                step="0.1"
                class="p-2 border rounded-md w-full"
                value={props.config.theta.to_string()}
                oninput={oninput_theta}
            />

            <label class="block font-semibold mb-1">{ "delta_t" }</label>
            <input
                type="number"
                step="0.1"
                class="p-2 border rounded-md w-full"
                value={props.config.delta_t.to_string()}
                oninput={oninput_delta_t}
            />
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct GammaCtrlProps {
    pub config: Config,
    pub on_update: Callback<Config>,
}

#[function_component(GammaCtrl)]
pub fn gamma_control(props: &GammaCtrlProps) -> Html {
    
    let grid_callback = |i: usize, j: usize| {
        let config = props.config.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<f64>() {
                let mut new_gamma = config.gamma;
                new_gamma[i][j] = value;
                let new_config = Config { gamma: new_gamma, ..config.clone() };
                on_update.emit(new_config);
            }
        })
    };

    html! {
        <div>
            <table class="border-collapse border border-gray-400">
                <caption class="caption-top font-bold mb-2">{ "Gamma Matrix" }</caption>
                <tbody>
                    { for (0..3).map(|i| {
                        html! {
                            <tr>
                                { for (0..3).map(|j| {
                                    let oninput = grid_callback(i, j);
                                    html! {
                                        <td class="border border-gray-300 p-1">
                                            <input
                                                type="number"
                                                step="0.1"
                                                class="w-16 p-1 border rounded"
                                                value={props.config.gamma[i][j].to_string()}
                                                {oninput}
                                            />
                                        </td>
                                    }
                                })}
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </div>
    }
}