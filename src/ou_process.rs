#[derive(Debug, Clone)]
pub struct Var {

}

#[derive(Debug, Clone)]
pub struct Config {

}

#[derive(Debug, Clone)]
pub struct OUState {
    config: Config,
    var: Var,
}

impl OUState {
    pub fn new() -> OUState {
        OUState {
            config : Config{},
            var : Var{},
        }
    }
}