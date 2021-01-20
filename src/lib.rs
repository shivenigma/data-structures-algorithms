mod ds {

    #[derive(PartialEq)]
    pub struct Array<T> {
        pub length: usize,
        data: Vec<T>,
    }

    impl<T> Array<T> {
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
            for (position, &element) in self.data.iter().enumerate() {
                if position == index {
                    data.push(new_element);
                }
                data.push(element);
            }
            self.data = data;
        }
        pub fn delete_at(&mut self, index: usize){
        }
        fn calculate_length(&mut self) {
            self.length = self.data.len();
        }

    }
}
mod ds_tests {
    use super::ds;

    #[test]
    fn add_elements_to_array() {
        let data: Vec<i32> = vec![];
        let mut arr = ds::Array::new(0, data);
        arr.push(8);
        arr.push(18);
        assert_eq!(arr.length, 2);
    }
    #[test]
    fn remove_element_from_array() {
        let data: Vec<i32> = vec![1, 4, 5];
        let mut arr = ds::Array::new(0, data);
        arr.pop();
        assert_eq!(arr.length, 2);
    }
    #[test]
    fn get_element_at_index() {
        let data: Vec<i32> = vec![1,2,5];
        let arr = ds::Array::new(0, data);
        let value = arr.get_element(1);
        assert_eq!(*value, 2);
    }
    #[test]
    fn insert_element_at_index() {
        let data: Vec<i32> = vec![1,2,5];
        let mut arr = ds::Array::new(0, data);
        arr.insert_at(1, 42);
        assert_eq!(*arr.get_element(1), 42);
    }
    #[test]
    fn delete_element_at_index() {
        let data: Vec<i32> = vec![1,2,5];
        let mut arr = ds::Array::new(0, data);
        arr.delete_at(1);
        assert_ne!(*arr.get_element(1), 2);
    }
}
