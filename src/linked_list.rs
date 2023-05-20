// Implement Singly Linked List from scratch

#[derive(Clone, Debug, PartialEq)]
struct Node {
    data: String,
    next: Option<Box<Node>>,
}

#[derive(Debug, PartialEq)]
struct SinglyLinkedList {
    first: Option<Box<Node>>,
}

#[allow(dead_code)]
impl Node {
    fn new(data: String) -> Self {
        Node {
            data: data,
            next: None,
        }
    }

    fn new_with_reference(data: String, next: Option<Box<Node>>) -> Self {
        Node { data, next }
    }

    fn update_next(&mut self, next: Option<Box<Node>>) {
        self.next = next;
    }
}

#[allow(dead_code)]
impl SinglyLinkedList {
    fn new() -> Self {
        SinglyLinkedList { first: None }
    }

    fn is_empty(self) -> bool {
        self.first == None
    }

    fn append(&mut self, data: String) {
        let new_node = Some(Box::new(Node::new(data.clone())));

        if self.first.is_none() {
            self.first = new_node.clone();
        } else {
            let mut current = &mut self.first;

            while let Some(node) = current {
                if node.next.is_none() {
                    node.next = Some(Box::new(Node {
                        data: data.clone(),
                        next: None,
                    }));
                    break;
                }

                current = &mut node.next;
            }
        }
    }

    fn prepend(&mut self, data: String) {
        if self.first.is_none() {
            self.first = Some(Box::new(Node::new(data)))
        } else {
            let current_first = self.first.clone();
            let new_node = Some(Box::new(Node::new_with_reference(data, current_first)));
            self.first = new_node;
        }
    }

    fn insert_after_given(&mut self, data: String, given_data: String) {
        if self.first.is_none() {
            panic!("List is empty, this action is not possible.");
        }

        let mut current_node = &mut self.first;
        let mut node_with_data_found = false;

        while let Some(node) = current_node {
            if given_data.eq(&node.data) {
                node_with_data_found = true;
                let pointer_to_next = node.next.clone();
                println!("Pointer to next: {:?}", pointer_to_next);
                let new_node = Some(Box::new(Node::new_with_reference(
                    data.clone(),
                    pointer_to_next,
                )));
                println!("New Node: {:?}", new_node.clone());
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
    fn when_new_list_created_it_is_empty_and_no_first_pointer_assigned() {
        let list = SinglyLinkedList::new();

        assert_eq!(list.first, None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_one_node_added_its_set_as_first() {
        let mut list = SinglyLinkedList::new();
        let data = String::from("Data Block 1");
        let expected_node = Some(Box::new(Node::new(data.clone())));

        list.append(data);

        assert_eq!(list.first, expected_node);
    }

    #[test]
    fn test_when_2_nodes_added() {
        let mut list = SinglyLinkedList::new();
        let data_1 = String::from("Data Block 1");
        let data_2 = String::from("Data Block 2");

        list.append(data_1.clone());
        list.append(data_2.clone());

        // First node points to data block 2
        let expected_pointer_node_1 = Some(Box::new(Node::new(data_2)));
        let actual_pointer_node_1 = list
            .first
            .clone()
            .expect("Actual Pointer Message Node 1")
            .next;

        // Second node points to None
        let expected_pointer_node_2: Option<Box<Node>> = None;
        let actual_pointer_node_2 = list
            .first
            .expect("Actual Pointer Message node 2")
            .next
            .expect("Actual Pointer Message Node 2.2")
            .next;

        assert_eq!(expected_pointer_node_1, actual_pointer_node_1);
        assert_eq!(expected_pointer_node_2, actual_pointer_node_2);
    }

    #[test]
    fn test_when_4_nodes_added() {
        let data_1 = String::from("A");
        let data_2 = String::from("B");
        let data_3 = String::from("C");
        let data_4 = String::from("D");

        let node_4 = Some(Box::new(Node {
            data: data_4.clone(),
            next: None,
        }));
        let node_3 = Some(Box::new(Node {
            data: data_3.clone(),
            next: node_4.clone(),
        }));

        let node_2 = Some(Box::new(Node {
            data: data_2.clone(),
            next: node_3.clone(),
        }));

        let node_1 = Some(Box::new(Node {
            data: data_1.clone(),
            next: node_2.clone(),
        }));

        let expected_list = SinglyLinkedList {
            first: node_1.clone(),
        };

        let mut actual_list = SinglyLinkedList::new();
        actual_list.append(data_1);
        actual_list.append(data_2);
        actual_list.append(data_3);
        actual_list.append(data_4);

        assert_eq!(expected_list, actual_list);
    }

    #[test]
    fn test_prepend_function_if_list_empty() {
        let data = String::from("A");
        let expected_list = SinglyLinkedList {
            first: Some(Box::new(Node::new(data.clone()))),
        };

        let mut actual_list = SinglyLinkedList::new();
        actual_list.prepend(data);

        assert_eq!(expected_list, actual_list);
    }

    #[test]
    fn test_prepend_if_one_node_added() {
        let data = String::from("A");
        let expected_list = SinglyLinkedList {
            first: Some(Box::new(Node::new(data.clone()))),
        };

        let mut actual_list = SinglyLinkedList::new();
        actual_list.prepend(data);

        assert_eq!(expected_list, actual_list);
    }

    #[test]
    fn test_prepend_when_list_already_have_elements_added() {
        let data_1 = String::from("A");
        let data_2 = String::from("B");
        let data_3 = String::from("C");
        let data_4 = String::from("Z");

        let node_3 = Some(Box::new(Node::new(data_3.clone())));
        let node_2 = Some(Box::new(Node {
            data: data_2.clone(),
            next: node_3,
        }));
        let node_1 = Some(Box::new(Node {
            data: data_1.clone(),
            next: node_2,
        }));
        let node_0 = Some(Box::new(Node {
            data: data_4.clone(),
            next: node_1,
        }));

        let expected_list = SinglyLinkedList { first: node_0 };

        let mut actual_list = SinglyLinkedList::new();
        actual_list.append(data_1);
        actual_list.append(data_2);
        actual_list.append(data_3);

        actual_list.prepend(data_4);

        assert_eq!(expected_list, actual_list);
    }

    #[test]
    #[should_panic]
    fn test_that_insert_after_panics_if_empty_list_given() {
        let mut empty_list = SinglyLinkedList::new();
        empty_list.insert_after_given(String::from("A"), String::from("B"))
    }

    #[test]
    #[should_panic]
    fn test_that_insert_after_panics_if_given_node_not_found() {
        let mut actual_list = SinglyLinkedList::new();
        // let given_node = Some(Box::new(Node::new(String::from("B"))));
        actual_list.append(String::from("A"));
        actual_list.insert_after_given(String::from("C"), String::from("B"));
    }

    #[test]
    fn test_insert_after_if_2_nodes_already_added_insert_between_2() {
        let data_1 = String::from("A");
        let data_2 = String::from("C");
        let data_3 = String::from("B");

        let node_3 = Some(Box::new(Node::new(data_3.clone())));
        let node_2 = Some(Box::new(Node::new_with_reference(data_2.clone(), node_3)));
        let node_1 = Some(Box::new(Node::new_with_reference(data_1.clone(), node_2)));
        let expected_list = SinglyLinkedList {
            first: node_1.clone(),
        };

        let mut actual_list = SinglyLinkedList::new();
        actual_list.append(data_1.clone());
        actual_list.append(data_3);

        actual_list.insert_after_given(data_2, data_1);

        assert_eq!(expected_list, actual_list);
    }
}
