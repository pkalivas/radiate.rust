use radiate_rust::Valid;

use crate::{node_collection, Direction, Node, NodeCollection};


pub struct Tree<T> 
where
    T: Clone + PartialEq + Default
{
    pub nodes: Vec<Node<T>>,
}

impl<T> NodeCollection<Tree<T>, T> for Tree<T>
where
    T: Clone + PartialEq + Default
{
    fn from_nodes(nodes: Vec<Node<T>>) -> Self {
        Self { nodes }
    }

    fn get(&self, index: usize) -> Option<&Node<T>> {
        self.nodes.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        self.nodes.get_mut(index)
    }

    fn get_nodes(&self) -> &[Node<T>] {
        &self.nodes
    }

    fn get_nodes_mut(&mut self) -> &mut [Node<T>] {
        &mut self.nodes
    }

    fn add(&mut self, nodes: Vec<Node<T>>) {
        self.nodes.extend(nodes);
    }
    
    fn set_cycles(mut self, indecies: Vec<usize>) -> Tree<T> {
        if indecies.len() == 0 {
            let all_indices = self.get_nodes()
                .iter()
                .map(|node| node.index)
                .collect::<Vec<usize>>();

            return self.set_cycles(all_indices)
        }

        for idx in indecies {
            let node_cycles = node_collection::get_cycles(self.get_nodes(), idx);

            if node_cycles.len() == 0 {
                let node = self.get_mut(idx).unwrap();
                (*node).direction = Direction::Forward;
            } else {
                for cycle_idx in node_cycles {
                    let node = self.get_mut(cycle_idx).unwrap();
                    (*node).direction = Direction::Backward;
                }
            }
        }

        self
    }
}

impl<T> Valid for Tree<T>
where
    T: Clone + PartialEq + Default
{
    fn is_valid(&self) -> bool {
        self.nodes
            .iter()
            .all(|node| node.is_valid())
    }
}

impl<T> Clone for Tree<T>
where
    T: Clone + PartialEq + Default
{
    fn clone(&self) -> Self {
        Tree::from_nodes(self.nodes
            .iter()
            .map(|node| node.clone())
            .collect::<Vec<Node<T>>>())
    }
}

impl<T> Default for Tree<T>
where
    T: Clone + PartialEq + Default
{
    fn default() -> Self {
        Tree { nodes: Vec::new() }
    }
}

impl<T> IntoIterator for Tree<T>
where
    T: Clone + PartialEq + Default
{
    type Item = Node<T>;
    type IntoIter = std::vec::IntoIter<Node<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl<T> FromIterator<Node<T>> for Tree<T>
where
    T: Clone + PartialEq + Default
{
    fn from_iter<I: IntoIterator<Item = Node<T>>>(iter: I) -> Self {
        let nodes = iter.into_iter().collect();
        Tree::from_nodes(nodes)
    }
}