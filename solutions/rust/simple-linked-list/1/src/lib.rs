pub struct SimpleLinkedList<T> {
    size: usize,
    head_node: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self{
        Node{
            element: element,
            next: None,
        }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { 
            size: 0,
            head_node: None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, _element: T) {
        let mut new_node = Box::new(Node::new(_element));
        let current_head = self.head_node.take();
        new_node.next = current_head;
        self.head_node = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let popped_node = self.head_node.take(); // Option<Box<Node<T>>>
        if let Some(node) = popped_node {
            self.head_node = node.next;
            self.size -= 1;
            return Some(node.element);
        } else {
            return None;
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = self.head_node.as_ref() {
            Some(&node.element)
        } else {
            None
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut copy_of_list = self;
        let mut reversed_list = SimpleLinkedList::new();

        while copy_of_list.is_empty() == false {
            if let Some(node) = copy_of_list.pop(){
                reversed_list.push(node);
            }
        }
        
        return reversed_list;

    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut result = SimpleLinkedList::new();
        for i in _iter{
            result.push(i);
        }
        return result;
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        let mut copy_of_list = _linked_list;

        while copy_of_list.is_empty() == false {
            if let Some(element) = copy_of_list.pop(){
                result.push(element);
            }
        }
        
        result.reverse();
        return result;
    }
}
