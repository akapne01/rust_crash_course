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
                    return;
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

        while let Some(node) = current_node {
            if node.data == given_data {
                let new_node = Some(
                    Box::new(Node::new_with_next(data.to_string(), node.next.take()))
                );
                node.next = new_node;
                return; // Return early after inserting the new node
            }
            current_node = &mut node.next;
        }

        panic!("Given node '{}' not found in the list!", given_data);
    }

    fn insert_before_given(&mut self, data: &str, given_data: &str) {
        if self.first.is_none() {
            panic!("List is empty, this action is not possible.");
        }

        let mut current_node = &mut self.first;

        while let Some(node) = current_node {
            if let Some(next_node) = &mut node.next {
                if next_node.data == given_data {
                    let new_node = Box::new(
                        Node::new_with_next(data.to_string(), Some(next_node.clone()))
                    );
                    node.next = Some(new_node);
                    return; // Return early after inserting the new node
                }
            }
            current_node = &mut node.next;
        }

        panic!("Given node '{}' not found in the list!", given_data);
    }
}

pub fn run() {
    println!("In Linked Lists");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Custom assertion macro to check if the list contains specific data
    macro_rules! assert_list_contains_data {
        ($list:expr, $expected_data:expr) => {
            let mut current = $list.first.as_ref();
            for expected in $expected_data {
                assert_eq!(current.map(|node| &node.data), Some(&expected.to_string()));
                current = current.unwrap().next.as_ref();
            }
            assert!(current.is_none());
        };
    }

    #[test]
    fn test_new_list_is_empty() {
        let list = SinglyLinkedList::new();

        assert_eq!(list.first, None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_append_single_node() {
        let data = "Data Block 1";

        let mut list = SinglyLinkedList::new();
        list.append(data);

        assert_eq!(list.first, Some(Box::new(Node::new(data.to_string()))));
        assert_eq!(
            list.first.as_ref().map(|node| &node.data),
            Some(&data.to_string())
        );
        assert_eq!(list.first.as_ref().unwrap().next, None);
    }

    #[test]
    fn test_append_multiple_nodes() {
        let values = ["A", "B", "C", "D"];
        let mut list: SinglyLinkedList = SinglyLinkedList::new();

        for value in &values {
            list.append(value);
        }

        let mut current = list.first.as_ref();

        for value in &values {
            assert_eq!(
                current.map(|node| &node.data),
                Some(&value.to_string())
            );
            current = current.and_then(|node| node.next.as_ref());
        }
        assert_eq!(
            current.map(|node| &node.data),
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
        let mut list = SinglyLinkedList::new();
        list.prepend("A");

        assert_eq!(
            list.first.as_ref().map(|node| &node.data),
            Some(&"A".to_string())
        );
        assert_eq!(list.first.as_ref().unwrap().next, None);
    }

    #[test]
    fn test_prepend_to_non_empty_list() {
        let values = vec!["A", "B"];
        let mut list = SinglyLinkedList::new();
        list.append(&values[0]);
        list.append(&values[1]);

        let first = list.first.as_ref().map(|node| &node.data);
        let second = list.first.as_ref().and_then(|node| node.next.as_ref().map(|node| &node.data));

        assert_eq!(first, Some(&values[0].to_string()));
        assert_eq!(second, Some(&values[1].to_string()));
    }

    #[test]
    fn test_prepend_adding_multiple_nodes() {
        let values = vec!["A", "B", "C"];
        let mut list = SinglyLinkedList::new();
        for value in values.iter().take(2) {
            list.append(value);
        }

        list.prepend(&values[2]);

        let expected_data = vec!["C", "A", "B"];

        assert_list_contains_data!(&list, &expected_data);
    }

    #[test]
    #[should_panic(expected = "List is empty, this action is not possible.")]
    fn test_insert_after_empty_list_panics() {
        let mut empty_list = SinglyLinkedList::new();
        empty_list.insert_after_given("A", "B");
    }

    #[test]
    #[should_panic(expected = "Given node 'B' not found in the list!")]
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

        let expected_data = vec!["A", "C", "B"];

        assert_list_contains_data!(&list, &expected_data);
    }

    #[test]
    #[should_panic(expected = "List is empty, this action is not possible.")]
    fn test_that_insert_before_panics_if_empty_list_given() {
        let mut empty_list = SinglyLinkedList::new();
        empty_list.insert_before_given("A", "B")
    }

    #[test]
    #[should_panic(expected = "Given node 'B' not found in the list!")]
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

        let expected_data = vec!["A", "C", "B"];

        assert_list_contains_data!(&list, &expected_data);
    }
}