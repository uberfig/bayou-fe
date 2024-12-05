#[derive(Clone)]
pub struct State {
    pub domain: String,
    pub limit: usize,
}

#[derive(Clone, Copy)]
pub enum Feed {
    Public,
    Home,
}
