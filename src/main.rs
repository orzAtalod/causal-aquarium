use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CircleTestProps {}

#[function_component]
pub fn CircleTest(props: &CircleTestProps) -> Html {
    let CircleTestProps {} = props;
    html! {
        <div>
            <svg width="200" height="200">
                <circle cx="100" cy="100" r="50" fill="blue" />
            </svg>
        </div>
    }
}

fn main() {
    yew::Renderer::<CircleTest>::with_props(CircleTestProps {}).render();
}