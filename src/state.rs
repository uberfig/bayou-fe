#[derive(Clone)]
pub struct State {
    pub domain: String,
    pub limit: usize,
    pub show_src: bool,
    pub use_timeline_reply_chains: bool,
    pub reply_chain_depth: u32,
}

#[derive(Clone, Copy)]
pub enum Feed {
    Public,
    Home,
}
