pub struct Array<T> {
    pub length: usize,
    data: Vec<T>,
}
impl<T: Clone> Array<T> {
    pub fn new(length: usize, data: Vec<T>)-> Array<T> {
        return Array { length, data};
    }
    pub fn push(&mut self, elem: T) {
        self.data.push(elem);
        self.calculate_length();
    }
    pub fn pop(&mut self) {
        self.data.pop();
        self.calculate_length();
    }
    pub fn get_element(&self, index: usize)-> &T {
        return &self.data[index];
    }
    pub fn insert_at(&mut self, index: usize, new_element: T) {
        let mut data: Vec<T> = vec![];
        for (position, element) in self.data.iter().enumerate() {
            if position == index {
                data.push(new_element.clone());
            }
            data.push(element.clone());
        }
        self.data = data;
        self.calculate_length();
    }
    pub fn delete_at(&mut self, index: usize){
        let mut data: Vec<T> = vec![];
        for (position, elem) in self.data.iter().enumerate() {
            if position != index {
                data.push(elem.clone());
            }
        }
        self.data = data;
        self.calculate_length();
    }
    fn calculate_length(&mut self) {
        self.length = self.data.len();
    }
}

#[cfg(test)]
mod tests {
    use super::Array;
    #[test]
    fn add_elements_to_array() {
        let data: Vec<i32> = vec![];
        let mut arr = Array::new(0, data);
        arr.push(8);
        arr.push(18);
        assert_eq!(arr.length, 2);
    }
    #[test]
    fn remove_element_from_array() {
        let data: Vec<i32> = vec![1, 4, 5];
        let mut arr = Array::new(0, data);
        arr.pop();
        assert_eq!(arr.length, 2);
    }
    #[test]
    fn get_element_at_index() {
        let data: Vec<i32> = vec![1,2,5];
        let arr = Array::new(0, data);
        let value = arr.get_element(1);
        assert_eq!(*value, 2);
    }
    #[test]
    fn insert_element_at_index() {
        let data: Vec<i32> = vec![1,2,5];
        let mut arr = Array::new(0, data);
        arr.insert_at(1, 42);
        assert_eq!(*arr.get_element(1), 42);
    }
    #[test]
    fn delete_element_at_index() {
        let data: Vec<i32> = vec![1,2,5];
        let mut arr = Array::new(0, data);
        arr.delete_at(1);
        assert_ne!(*arr.get_element(1), 2);
    }
}
