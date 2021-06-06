pub struct SinglyLinkedList<T> {
    value: Box<T>,
    next: Option<Box<T>>
}
impl<T> SinglyLinkedList<T> {
    pub fn new(element: T) -> SinglyLinkedList<T> {
        return SinglyLinkedList {
            value: Box::new(element),
            next: None
        }
    }
    pub fn push_front(&self, element: T) {
        unimplemented!();
    }
    pub fn pop_front(&self) -> T {
        unimplemented!();
    }
    pub fn top_front(&self) -> T {
        unimplemented!();
    }
    pub fn push_back(&self, element: T){
    }
    pub fn pop_back(&self)-> T {
        unimplemented!();
    }
    pub fn top_back(&self)-> T{
        unimplemented!();
    }
    pub fn add_before(&self, element:T, index: i32){
    }
    pub fn add_after(&self, element: T, index: i32){}
    pub fn find(&self, element: T) -> T{
        unimplemented!();
    }
    pub fn erase(&self, element: T){
    }
    pub fn is_empty(&self)-> bool {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::SinglyLinkedList;
    #[test]
    fn check_if_list_empty() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::new(5);
        assert_eq!(list.is_empty(), false);
        let elem = list.pop_front();
        assert_eq!(elem, 5);
        assert_eq!(list.is_empty(), true);
        list.push_front(3);
        list.push_back(3);
        assert_eq!(list.is_empty(), false);
        let back_elem = list.pop_back();
        assert_eq!(back_elem, 3);

    }
}
