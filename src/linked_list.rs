// Implement Singly Linked List from scratch

#[derive(Clone, Debug, PartialEq)]
struct Node {
    data: String,
    next: Option<Box<Node>>,
}

#[allow(dead_code)]
impl Node {
    fn new(data: String) -> Self {
        Node {
            data,
            next: None,
        }
    }

    fn new_with_next(data: String, next: Option<Box<Node>>) -> Self {
        Node { data, next }
    }
}

#[derive(Debug, PartialEq)]
struct SinglyLinkedList {
    first: Option<Box<Node>>,
}

#[allow(dead_code)]
impl SinglyLinkedList {
    fn new() -> Self {
        SinglyLinkedList { first: None }
    }

    fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    fn append(&mut self, data: &str) {
        let new_node = Box::new(Node::new(data.to_string()));

        if self.is_empty() {
            self.first = Some(new_node);
        } else {
            let mut current = &mut self.first;

            while let Some(node) = current {
                if node.next.is_none() {
                    node.next = Some(new_node);
                    break;
                }

                current = &mut node.next;
            }
        }
    }

    fn prepend(&mut self, data: &str) {
        let new_node = Box::new(Node::new_with_next(data.to_string(), self.first.take()));
        self.first = Some(new_node);
    }

    fn insert_after_given(&mut self, data: &str, given_data: &str) {
        if self.first.is_none() {
            panic!("List is empty, this action is not possible.");
        }

        let mut current_node = &mut self.first;
        let mut node_with_data_found = false;

        while let Some(node) = current_node {
            if given_data.eq(&node.data) {
                node_with_data_found = true;
                let pointer_to_next = node.next.clone();
                let new_node = Some(
                    Box::new(Node::new_with_next(data.to_string(), pointer_to_next))
                );
                node.next = new_node;
                break;
            }
            current_node = &mut node.next;
        }

        if !node_with_data_found {
            panic!("Given node {:?} not found in the list!!!!", given_data);
        }
    }

    fn insert_before_given(&mut self, data: &str, given_data: &str) {
        if self.first.is_none() {
            panic!("List is empty, this action is not possible.");
        }
        let mut node_with_data_found = false;
        let mut current_node = &mut self.first;

        // We want to check if the node.next is equal to the node made from given_data
        while let Some(node) = current_node {
            let next_data: String = node.next.clone().expect("Next node is not defined").data;
            if given_data == next_data {
                node_with_data_found = true;
                let pointer_to_next = node.next.clone();
                let new_node = Some(
                    Box::new(Node::new_with_next(data.to_string(), pointer_to_next))
                );
                node.next = new_node;
                break;
            }
            current_node = &mut node.next;
        }

        if !node_with_data_found {
            panic!("Given node {:?} not found in the list!!!!", given_data);
        }
    }
}

pub fn run() {
    println!("In Linked Lists");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list_is_empty() {
        let list = SinglyLinkedList::new();

        assert_eq!(list.first, None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_append_single_node() {
        let data = "Data Block 1";

        let mut actual_list = SinglyLinkedList::new();
        actual_list.append(data);

        assert_eq!(actual_list.first, Some(Box::new(Node::new(data.to_string()))));
        assert_eq!(
            actual_list.first.as_ref().map(|node| &node.data),
            Some(&data.to_string())
        );
        assert_eq!(actual_list.first.as_ref().unwrap().next, None);
    }

    #[test]
    fn test_append_multiple_nodes() {
        let a = "A";
        let b = "B";
        let c = "C";
        let d = "D";
        let mut list: SinglyLinkedList = SinglyLinkedList::new();

        list.append(a);
        list.append(b);
        list.append(c);
        list.append(d);

        assert_eq!(
            list.first.as_ref().map(|node| &node.data),
            Some(&a.to_string())
        );
        assert_eq!(
            list.first
                .as_ref()
                .unwrap()
                .next.as_ref()
                .map(|node| &node.data),
            Some(&b.to_string())
        );
        assert_eq!(
            list.first
                .as_ref()
                .unwrap()
                .next.as_ref()
                .unwrap()
                .next.as_ref()
                .map(|node| &node.data),
            Some(&c.to_string())
        );
        assert_eq!(
            list.first
                .as_ref()
                .unwrap()
                .next.as_ref()
                .unwrap()
                .next.as_ref()
                .unwrap()
                .next.as_ref()
                .map(|node| &node.data),
            Some(&d.to_string())
        );
        assert_eq!(
            list.first
                .as_ref()
                .unwrap()
                .next.as_ref()
                .unwrap()
                .next.as_ref()
                .unwrap()
                .next.as_ref()
                .unwrap()
                .next.as_ref()
                .map(|node| &node.data),
            None
        );
    }

    #[test]
    fn test_prepend_empty_list() {
        let a = "A";
        let mut actual_list = SinglyLinkedList::new();
        actual_list.prepend(a);

        assert_eq!(
            actual_list.first.as_ref().map(|node| &node.data),
            Some(&a.to_string())
        );
        assert_eq!(actual_list.first.as_ref().unwrap().next, None);
    }

    #[test]
    fn test_prepend_single_node_to_empty_list() {
        let mut actual_list = SinglyLinkedList::new();
        actual_list.prepend("A");

        assert_eq!(
            actual_list.first.as_ref().map(|node| &node.data),
            Some(&"A".to_string())
        );
        assert_eq!(
            actual_list.first
                .as_ref()
                .unwrap()
                .next.as_ref()
                .map(|node| &node.data),
            None
        )
    }

    #[test]
    fn test_prepend_to_non_empty_list() {
        let mut list = SinglyLinkedList::new();
        list.append("A");
        list.prepend("B");

        assert_eq!(
            list.first.as_ref().map(|node| &node.data),
            Some(&"B".to_string())
        );
        assert_eq!(
            list.first
                .as_ref()
                .unwrap()
                .next.as_ref()
                .map(|node| &node.data),
            Some(&"A".to_string())
        );
    }

    #[test]
    fn test_prepend_adding_multiple_nodes() {
        let mut list = SinglyLinkedList::new();
        list.append("A");
        list.append("B");
        list.prepend("C");

        assert_eq!(
            list.first.as_ref().map(|node| &node.data),
            Some(&"C".to_string())
        );
        assert_eq!(
            list.first
                .as_ref()
                .unwrap()
                .next.as_ref()
                .map(|node| &node.data),
            Some(&"A".to_string())
        );
        assert_eq!(
            list.first
                .as_ref()
                .unwrap()
                .next.as_ref()
                .unwrap()
                .next.as_ref()
                .map(|node| &node.data),
            Some(&"B".to_string())
        );
    }

    #[test]
    #[should_panic]
    fn test_insert_after_empty_list_panics() {
        let mut empty_list = SinglyLinkedList::new();
        empty_list.insert_after_given("A", "B");
    }

    #[test]
    #[should_panic]
    fn test_insert_after_given_data_not_found_panics() {
        let mut actual_list = SinglyLinkedList::new();
        actual_list.append("A");
        actual_list.insert_after_given("C", "B");
    }

    #[test]
    fn test_insert_after_given_two_nodes_inserts_in_between_them() {
        let mut list = SinglyLinkedList::new();
        list.append("A");
        list.append("B");

        list.insert_after_given("C", "A");

        assert_eq!(
            list.first.as_ref().map(|node| &node.data),
            Some(&"A".to_string())
        );
        assert_eq!(
            list.first
                .as_ref()
                .unwrap()
                .next.as_ref()
                .map(|node| &node.data),
            Some(&"C".to_string())
        );
        assert_eq!(
            list.first
                .as_ref()
                .unwrap()
                .next.as_ref()
                .unwrap()
                .next.as_ref()
                .map(|node| &node.data),
            Some(&"B".to_string())
        );
    }

    #[test]
    #[should_panic]
    fn test_that_insert_before_panics_if_empty_list_given() {
        let mut empty_list = SinglyLinkedList::new();
        empty_list.insert_before_given("A", "B")
    }

    #[test]
    #[should_panic]
    fn test_that_insert_before_panics_if_given_node_not_found() {
        let mut list = SinglyLinkedList::new();
        list.append("A");
        list.insert_before_given("C", "B");
    }

    #[test]
    fn test_insert_before_if_two_nodes_already_added_insert_between_them() {
        let mut list = SinglyLinkedList::new();
        list.append("A");
        list.append("B");

        list.insert_before_given("C", "B");

        assert_eq!(
            list.first.as_ref().map(|node| &node.data),
            Some(&"A".to_string())
        );
        assert_eq!(
            list.first
                .as_ref()
                .unwrap()
                .next.as_ref()
                .map(|node| &node.data),
            Some(&"C".to_string())
        );
        assert_eq!(
            list.first
                .as_ref()
                .unwrap()
                .next.as_ref()
                .unwrap()
                .next.as_ref()
                .map(|node| &node.data),
            Some(&"B".to_string())
        );
    }
}