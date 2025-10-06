/// The module for the aquarium simulation.
/// Transforms the variants to density of the fish

use crate::ou_process::*;
use yew::prelude::*;
use yew_hooks::use_interval;
use rand::Rng;

// configure
const MAX_FISH :usize = 10;
const MIN_FISH :usize = 0;

#[derive(PartialEq, Properties)]
pub struct AquariumProps {
    pub var: Var,
}

#[function_component(Aquarium)]
pub fn aquarium(props: &AquariumProps) -> Html {
    let var = props.var.clone();
    let width = 800.0;
    let height = 400.0;

    let fish_positions = use_state(|| {
        let mut rng = rand::thread_rng();
        [
            (0..MAX_FISH).map(|_| (rng.gen_range(0.0..width), rng.gen_range(0.0..height))).collect::<Vec<_>>(),
            (0..MAX_FISH).map(|_| (rng.gen_range(0.0..width), rng.gen_range(0.0..height))).collect::<Vec<_>>(),
            (0..MAX_FISH).map(|_| (rng.gen_range(0.0..width), rng.gen_range(0.0..height))).collect::<Vec<_>>(),
        ]
    });

    {
        let fish_positions = fish_positions.clone();
        use_interval(
            move || {
                let mut rng = rand::thread_rng();
                fish_positions.set([
                    fish_positions[0].iter().map(|&(x, y)| {
                        (wrap(x + rng.gen_range(-5.0..5.0), width),
                         wrap(y + rng.gen_range(-5.0..5.0), height))
                    }).collect(),
                    fish_positions[1].iter().map(|&(x, y)| {
                        (wrap(x + rng.gen_range(-5.0..5.0), width),
                         wrap(y + rng.gen_range(-5.0..5.0), height))
                    }).collect(),
                    fish_positions[2].iter().map(|&(x, y)| {
                        (wrap(x + rng.gen_range(-5.0..5.0), width),
                         wrap(y + rng.gen_range(-5.0..5.0), height))
                    }).collect(),
                ]);
            },
            100,
        );
    }

    let num_fish: Vec<usize> = var.val.iter().map(|&v| {
        MIN_FISH + ((v / 100.0) * (MAX_FISH - MIN_FISH) as f64).floor() as usize
    }).collect();

    html! {
        <svg width={width.to_string()} height={height.to_string()} style="border: 2px solid #000;">
            { for fish_positions[0].iter().take(num_fish[0]).map(|&(x, y)| {
                html! { <circle cx={x.to_string()} cy={y.to_string()} r="10" fill="red" /> }
            }) }

            { for fish_positions[1].iter().take(num_fish[1]).map(|&(x, y)| {
                html! {
                    <g stroke="green" stroke-width="2">
                        <line x1={(x-7.0).to_string()} y1={(y-7.0).to_string()} x2={(x+7.0).to_string()} y2={(y+7.0).to_string()} />
                        <line x1={(x-7.0).to_string()} y1={(y+7.0).to_string()} x2={(x+7.0).to_string()} y2={(y-7.0).to_string()} />
                    </g>
                }
            }) }

            { for fish_positions[2].iter().take(num_fish[2]).map(|&(x, y)| {
                html! { <rect x={(x-7.0).to_string()} y={(y-7.0).to_string()} width="14" height="14" fill="blue" /> }
            }) }
        </svg>
    }
}

fn wrap(value: f64, max: f64) -> f64 {
    if value < 0.0 { max } else if value > max { 0.0 } else { value }
}