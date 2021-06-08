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
    pub fn find(&self, element: T) -> Option<T>{
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
        let list: SinglyLinkedList<i8> = SinglyLinkedList::new(5);
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
    #[test]
    fn add_element_at_front() {
        let list: SinglyLinkedList<i8> = SinglyLinkedList::new(5);
        list.push_front(7);
        assert_eq!(list.pop_front(), 7);
        list.push_front(6);
        assert_eq!(list.pop_front(), 6);
    }
    #[test]
    fn remove_element_from_front() {
        let list: SinglyLinkedList<i8> = SinglyLinkedList::new(6);
        list.push_back(4);
        list.push_front(3);
        assert_eq!(list.pop_front(), 3);
        assert_eq!(list.pop_front(), 4);
        assert_eq!(list.is_empty(), true);
        list.push_back(3);
        assert_eq!(list.pop_front(), 3);
    }
    #[test]
    fn get_the_first_element() {
        let list: SinglyLinkedList<i8> = SinglyLinkedList::new(6);
        list.push_front(3);
        assert_eq!(list.top_front(), 3);
        list.push_back(4);
        list.push_front(5);
        assert_eq!(list.top_front(), 5);
    }
    #[test]
    fn add_elements_at_back() {
        let list: SinglyLinkedList<i8> = SinglyLinkedList::new(3);
        list.push_back(4);
        assert_eq!(list.pop_back(), 4);
        assert_eq!(list.pop_back(), 3);
        list.push_back(5);
        assert_eq!(list.top_front(), 5);
    }
    #[test]
    fn remove_elements_from_back() {
        let list: SinglyLinkedList<i8> = SinglyLinkedList::new(8);
        list.push_back(3);
        assert_eq!(list.pop_back(), 3);
        assert_eq!(list.pop_back(), 8);
        assert_eq!(list.is_empty(), true);
        list.push_back(4);
        assert_eq!(list.pop_back(), 4);
    }
    #[test]
    fn get_last_element(){
        let list: SinglyLinkedList<i8> = SinglyLinkedList::new(6);
        list.push_front(3);
        assert_eq!(list.top_back(), 3);
        list.pop_back();
        assert_eq!(list.top_back(), 6);
        list.push_front(3);
        list.push_back(7);
        assert_eq!(list.top_back(), 7);
    }
    #[test]
    fn add_before_the_element() {
        let list: SinglyLinkedList<i8> = SinglyLinkedList::new(3);
        let elem = list.top_front();
        list.add_before(elem, 8);
        let elem1 = list.top_front();
        assert_eq!(elem1, 8);
        list.add_before(elem1, 7);
        assert_eq!(list.top_front(), 7);
    }
    #[test]
    fn add_after_the_element() {
        let list: SinglyLinkedList<i8> = SinglyLinkedList::new(5);
        list.push_back(3);
        let elem = list.top_back();
        list.add_after(elem, 6);
        let elem1 = list.top_back();
        assert_eq!(elem1, 6);
        list.add_after(elem1, 4);
        assert_eq!(list.top_back(), 4);
    }
    #[test]
    fn find_the_element() {
        let list: SinglyLinkedList<i8> = SinglyLinkedList::new(35);
        list.push_back(3);
        list.push_back(8);
        // Todo: Confirm if find should return value or index or a boolean.
        assert_eq!(list.find(3), Some(3));
    }
    #[test]
    fn delete_element() {
        let list: SinglyLinkedList<i8> = SinglyLinkedList::new(66);
        list.push_back(5);
        list.push_back(3);
        list.erase(3);
        assert_eq!(list.top_back(), 5);
        list.erase(5);
        list.erase(66);
        assert_eq!(list.is_empty(), true);
    }
}
