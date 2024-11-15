use super::super::Node;

pub struct GraphIterator<'a, T>
where
    T: Clone + PartialEq + Default
{
    pub graph: &'a [Node<T>],
    pub completed: Vec<bool>,
    pub index: usize
}

impl<'a, T> GraphIterator<'a, T> 
where
    T: Clone + PartialEq + Default
{
    pub fn new(graph: &'a [Node<T>]) -> Self {
        Self { 
            graph, 
            completed: vec![false; graph.len()],
            index: 0
        }
    }
}

impl<'a, T> Iterator for GraphIterator<'a, T> 
where
    T: Clone + PartialEq + Default
{
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pending_index = 0;
        while self.index < self.graph.len() {
            for index in self.index..self.graph.len() {
                if self.completed[index] {
                    continue;
                }

                let node = &self.graph[index];

                let mut degree = node.incoming.len();
                for incoming_index in node.incoming.iter() {
                    let incoming_node = &self.graph[*incoming_index];
                    if self.completed[incoming_node.index] || incoming_node.is_recurrent() {
                        degree -= 1;
                    }
                }

                if degree == 0 {
                    self.completed[node.index] = true;
                    self.index = index;
                    return Some(node);
                } else {
                    pending_index = std::cmp::max(pending_index, index);
                }
            }

            if pending_index == self.index {
                return None;
            } else {
                self.index = pending_index;
            }
        }

        None
    }
}