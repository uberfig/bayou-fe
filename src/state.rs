#[derive(Clone)]
pub struct State {
    pub domain: String,
    pub limit: usize,
    pub show_src: bool,
}

#[derive(Clone, Copy)]
pub enum Feed {
    Public,
    Home,
}
