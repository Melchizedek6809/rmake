use crate::MakeGraph;

#[derive(Clone, Debug, Default)]
pub enum MakeAtom {
    #[default]
    Nil,
    Literal(String),
}

impl MakeAtom {
    pub fn new(str: &str) -> Self {
        MakeAtom::Literal(str.into())
    }

    pub fn eval(&self, _graph: &MakeGraph) -> String {
        match self {
            MakeAtom::Nil => "".to_owned(),
            MakeAtom::Literal(str) => str.clone(),
        }
    }
}
