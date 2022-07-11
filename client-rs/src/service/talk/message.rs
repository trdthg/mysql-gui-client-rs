#[derive(Clone)]
pub enum Message {
    Normal { msg: String, user: String },
    Cmd(Cmd),
}

#[derive(Clone)]
pub enum Cmd {
    Who,
    Secret(String, String),
}
