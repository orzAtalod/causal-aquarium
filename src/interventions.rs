/// this file produces the intervention component
/// the intervention component is 3 sliders to set the intervention value of each variant
/// it's designed similar to the original experiment to make sure the things changed are as few as possible

use crate::{control::_GammaCtrlProps::on_update, interventions::_InterventionsProps::var, ou_process::*};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct InterventionsProps {
    pub var: Var,
    pub on_update: Callback<Intervention>,
}

#[function_component(Interventions)]
pub fn interventions(props: &InterventionsProps) -> Html {
    
    let oninput = |var_index: usize| {
        let intervent_update = props.on_update.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value_as_number();
            let new_intervention = Intervention { index: Some(var_index), value };
            intervent_update.emit(new_intervention);
        })
    };

    let onmouseup = {
        let intervent_update = props.on_update.clone();
        Callback::from(move |_| {
            let new_intervention = Intervention { index: None, value: 0.0 };
            intervent_update.emit(new_intervention);
        })
    };

    html! {
        <div>
            <p class="font-semibold mb-1">{ "A" }</p>
            <input
                type="range"
                min="0"
                max="100"
                value={props.var.val[0].to_string()}
                oninput={oninput(0)}
                onmouseup={onmouseup.clone()}
            />
            <p class="font-semibold mb-1">{ "B" }</p>
            <input
                type="range"
                min="0"
                max="100"
                value={props.var.val[1].to_string()}
                oninput={oninput(1)}
                onmouseup={onmouseup.clone()}
            />
            <p class="font-semibold mb-1">{ "C" }</p>
            <input
                type="range"
                min="0"
                max="100"
                value={props.var.val[2].to_string()}
                oninput={oninput(2)}
                onmouseup={onmouseup.clone()}
            />
        </div>
    }
}