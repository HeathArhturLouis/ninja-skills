use core::fmt::Debug;
use std::collections::HashMap;

fn main() {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeID(usize);

#[derive(Debug)]
struct Node<T>
where
    T: Debug,
{
    next: Option<NodeID>,
    data: T,
}

#[derive(Debug)]
struct SList<T>
where
    T: Debug,
{
    free_id: NodeID,
    head: Option<NodeID>,
    nodes: HashMap<NodeID, Node<T>>,
}

impl<T> SList<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        Self {
            free_id: NodeID(0),
            head: None,
            nodes: HashMap::new(),
        }
    }

    // Returns the NodeID of the item.
    pub fn push(&mut self, item: T) -> NodeID {
        let new_head = Node {
            next: self.head,
            data: item,
        };
        self.head = Some(self.free_id);
        self.free_id.0 += 1;
        self.nodes.insert(self.head.unwrap(), new_head);
        self.head.unwrap()
    }

    pub fn get(&self, node_id: NodeID) -> Option<&T> {
        self.nodes.get(&node_id).map(|node| &node.data)
    }

    pub fn top(&self) -> Option<&T> {
        self.head.map(|head_id| self.get(head_id).unwrap())
    }

    pub fn find_prior(&self, node_id: NodeID) -> Option<NodeID> {
        let mut current_id = self.head;
        let mut prior_id: Option<NodeID> = None;
        while let Some(curr_id) = current_id {
            if curr_id == node_id {
                break;
            } else {
                prior_id = current_id;
                current_id = self.nodes.get(&curr_id).unwrap().next;
            }
        }
        prior_id
    }

    pub fn delete_middle(&mut self, node_id: NodeID) -> Result<(), ()> {
        // Check that node_id is in there and not the end.
        let node = self.nodes.get(&node_id).ok_or(())?;
        if node.next.is_none() {
            return Err(());
        }
        
        /*
        let prior_id = self.find_prior(node_id);
        match prior_id {
            Some(prev_id) => {
                self.nodes.get_mut(&prev_id).unwrap().next = self.nodes.get(&node_id).unwrap().next;
                self.nodes.remove(&node_id);
                Ok(())
            }
            None => Err(()),
        }
        */
        
        let prior_id = self.find_prior(node_id).ok_or(())?;
        self.nodes.get_mut(&prior_id).unwrap().next = node.next;
        self.nodes.remove(&node_id);
        Ok(())
    }
}

pub mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_empty_top() {
        let list = SList::<f64>::new();
        assert_eq!(list.top(), None);
    }

    #[test]
    fn test_push_top() {
        let mut list = SList::<i32>::new();
        let test_vals = [0, 4, 6, 3, -3716, 48];
        for test_val in test_vals.iter() {
            list.push(test_val.clone());
            assert_eq!(list.top(), Some(test_val));
        }
    }

    #[test]
    fn test_get_empty() {
        let list = SList::<Box<Option<String>>>::new();
        for i in 0..1000 {
            assert_eq!(list.get(NodeID(i)), None);
        }
    }

    #[test]
    fn test_push_id() {
        let mut list = SList::<i32>::new();
        let id_0 = list.push(0);
        let id_8 = list.push(8);
        let id_m28 = list.push(-28);
        let id_1000 = list.push(1000);

        assert_eq!(list.get(id_0), Some(&0));
        assert_eq!(list.get(id_8), Some(&8));
        assert_eq!(list.get(id_m28), Some(&-28));
        assert_eq!(list.get(id_1000), Some(&1000));
    }

    #[test]
    fn test_find_prior() {
        let mut list = SList::<i32>::new();
        let id_2 = list.push(2);
        let id_3 = list.push(3);
        let id_5 = list.push(5);
        let id_7 = list.push(7);
        let id_11 = list.push(11);
        let id_13 = list.push(13);

        // It pushes in reverse order.
        assert_eq!(list.find_prior(id_13), None);
        assert_eq!(list.find_prior(id_11), Some(id_13));
        assert_eq!(list.find_prior(id_7), Some(id_11));
        assert_eq!(list.find_prior(id_5), Some(id_7));
        assert_eq!(list.find_prior(id_3), Some(id_5));
        assert_eq!(list.find_prior(id_2), Some(id_3));
    }

    #[test]
    fn test_del_middle() {
        let mut list = SList::<u16>::new();
        let id_0 = list.push(0);
        let id_1 = list.push(1);
        let id_2 = list.push(2);
        let id_3 = list.push(3);
        let id_4 = list.push(4);

        assert_eq!(list.delete_middle(id_3), Ok(()));

        assert_eq!(list.get(id_0), Some(&0));
        assert_eq!(list.get(id_1), Some(&1));
        assert_eq!(list.get(id_2), Some(&2));
        assert_eq!(list.get(id_3), None);
        assert_eq!(list.get(id_4), Some(&4));
    }
    
    #[test]
    fn test_del_middle_end() {
        let mut ctci = SList::<Rc<str>>::new();
        let id_d = ctci.push(Rc::from("Daniel"));
        let id_m = ctci.push(Rc::from("Marcell"));
        let id_l = ctci.push(Rc::from("Louis"));
        let id_g = ctci.push(Rc::from("George"));
        
        assert_eq!(ctci.delete_middle(id_d), Err(()));
        assert_eq!(ctci.get(id_d), Some(&Rc::from("Daniel")));
    }
    
    #[test]
    fn test_del_middle_start() {
        let mut ctci = SList::<Rc<str>>::new();
        let id_d = ctci.push(Rc::from("Daniel"));
        let id_m = ctci.push(Rc::from("Marcell"));
        let id_l = ctci.push(Rc::from("Louis"));
        let id_g = ctci.push(Rc::from("George"));
        
        assert_eq!(ctci.delete_middle(id_g), Err(()));
        assert_eq!(ctci.get(id_g), Some(&Rc::from("George")));
    }
}
