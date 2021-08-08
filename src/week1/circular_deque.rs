//641. 设计循环双端队列
//设计实现双端队列。
// 你的实现需要支持以下操作：
//
// MyCircularDeque(k)：构造函数,双端队列的大小为k。
// insertFront()：将一个元素添加到双端队列头部。 如果操作成功返回 true。
// insertLast()：将一个元素添加到双端队列尾部。如果操作成功返回 true。
// deleteFront()：从双端队列头部删除一个元素。 如果操作成功返回 true。
// deleteLast()：从双端队列尾部删除一个元素。如果操作成功返回 true。
// getFront()：从双端队列头部获得一个元素。如果双端队列为空，返回 -1。
// getRear()：获得双端队列的最后一个元素。 如果双端队列为空，返回 -1。
// isEmpty()：检查双端队列是否为空。
// isFull()：检查双端队列是否满了。
use std::collections::VecDeque;

struct MyCircularDeque {
    cache: VecDeque<i32>,
    capacity: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    /** Initialize your data structure here. Set the size of the deque to be k. */
    fn new(k: i32) -> Self {
        MyCircularDeque {
            cache: VecDeque::with_capacity(k as usize),
            capacity: k as usize,
        }
    }

    /** Adds an item at the front of Deque. Return true if the operation is successful. */
    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.cache.push_front(value);
            true
        }
    }

    /** Adds an item at the rear of Deque. Return true if the operation is successful. */
    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.cache.push_back(value);
            true
        }
    }

    /** Deletes an item from the front of Deque. Return true if the operation is successful. */
    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.cache.pop_front();
            true
        }
    }

    /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.cache.pop_back();
            true
        }
    }

    /** Get the front item from the deque. */
    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            *self.cache.front().unwrap()
        }
    }

    /** Get the last item from the deque. */
    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            *self.cache.back().unwrap()
        }
    }

    /** Checks whether the circular deque is empty or not. */
    fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /** Checks whether the circular deque is full or not. */
    fn is_full(&self) -> bool {
        self.cache.len() == self.capacity
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let k = 20;
        let value = 3;
        let mut obj = MyCircularDeque::new(k);
        let ret_1: bool = obj.insert_front(value);
        let ret_2: bool = obj.insert_last(value);
        let ret_3: bool = obj.delete_front();
        let ret_4: bool = obj.delete_last();
        let ret_5: i32 = obj.get_front();
        let ret_6: i32 = obj.get_rear();
        let ret_7: bool = obj.is_empty();
        let ret_8: bool = obj.is_full();
    }
}