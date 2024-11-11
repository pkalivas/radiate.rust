use crate::{architects::schema::node_types::NodeType, operations::op::Ops};

use super::node::Node;


pub struct Tracer<T>
where
    T: Clone
{
    pub input_size: usize,
    pub pending_idx: usize,
    pub args: Vec<T>,
    pub result: Option<T>,
    pub previous_result: Option<T>,
}

impl<T> Tracer<T> 
where
    T: Clone + PartialEq
{
    pub fn new(input_size: usize) -> Self {
        Tracer {
            input_size,
            pending_idx: 0,
            args: Vec::with_capacity(input_size),
            result: None,
            previous_result: None,
        }
    }

    pub fn activate(&mut self, node: &Node<T>) {
        if self.pending_idx != self.input_size {
            panic!("Tracer is not ready to be evaluated.");
        }

        match node.node_type {
            NodeType::Input => {
                self.previous_result = self.result.clone();
                self.result = Some(self.args[0].clone());
            },
            NodeType::Gate | NodeType::Output | NodeType::Weight | NodeType::Link | NodeType::Aggregate => {
                self.previous_result = self.result.clone();
                self.result = match &node.value {
                    Ops::Value(ref value) => Some(value.clone()),
                    Ops::Const(_, ref value) => Some(value.clone()),
                    Ops::Fn(_, _, ref fn_ptr) => Some(fn_ptr(&self.args)),
                    Ops::MutableConst(_, _, ref val, _, fn_ptr) => Some(fn_ptr(&self.args, val)),
                    Ops::Var(_, idx) => Some(self.args[*idx].clone()),
                }
            },
        }

        // if !node.is_enabled {
        //     self.result = None;
        // } else {
        //     match node.node_type {
        //         NodeType::Input | NodeType::Leaf | NodeType::Bias => {
        //             self.previous_result = self.result.clone();
        //             self.result = self.args[0].clone();
        //         },
        //         NodeType::Gate | NodeType::Output | NodeType::Weight | NodeType::Link | NodeType::Root | NodeType::Aggregate | NodeType::Vertex | NodeType::Edge => {
        //             self.previous_result = self.result.clone();
        //             self.result = node.value.apply(&self.args);
        //         },
        //         NodeType::Memory => {
        //             self.result = self.previous_result.clone();
        //             self.previous_result = node.value.apply(&self.args);
        //         },
        //     }
        // }

        self.pending_idx = 0;
    }

    // ublic void Eval()
    // {
    //     if (PendingIndex != InputSize())
    //     {
    //         throw new InvalidOperationException($"{this}\n is not ready to be evaluated. PendingIndex: {PendingIndex}");
    //     }
        
    //     if (!IsEnabled)
    //     {
    //         Result = default!;
    //     }
    //     else switch (NodeType)
    //     {
    //         case NodeTypes.Input or NodeTypes.Leaf or NodeTypes.Bias:
    //             Previous = Result;
    //             Result = Args[0];
    //             break;
    //         case NodeTypes.Gate or NodeTypes.Output or NodeTypes.Weight or NodeTypes.Link or NodeTypes.Root or NodeTypes.Aggregate or NodeTypes.Vertex or NodeTypes.Edge:
    //             Previous = Result;
    //             Result = Value.Apply(Args);
    //             break;
    //         case NodeTypes.Memory:
    //             Result = Previous;
    //             Previous = Value.Apply(Args);
    //             break;
    //         default:
    //             throw new ArgumentOutOfRangeException();
    //     }
        
    //     PendingIndex = 0;
    // }
}

impl<T> Clone for Tracer<T>
where
    T: Clone
{
    fn clone(&self) -> Self {
        Tracer {
            input_size: self.input_size,
            pending_idx: self.pending_idx,
            args: self.args.clone(),
            result: self.result.clone(),
            previous_result: self.previous_result.clone(),
        }
    }
}