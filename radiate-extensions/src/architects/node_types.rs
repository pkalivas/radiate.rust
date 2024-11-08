
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Input,
    Output,
    Gate,
    Aggregate,
    Weight
}