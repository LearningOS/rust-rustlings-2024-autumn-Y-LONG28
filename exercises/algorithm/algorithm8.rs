/*
	queue
	This question requires you to use queues to implement the functionality of the stack
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if !self.elements.is_empty() {
            Some(self.elements.remove(0usize))
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.first()
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}

// 用队列实现栈
pub struct myStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    // 将元素压入栈
    pub fn push(&mut self, elem: T) {
        self.q2.enqueue(elem);

        // 将 q1 中所有元素移动到 q2
        while let Some(val) = self.q1.dequeue() {
            self.q2.enqueue(val);
        }

        // 交换 q1 和 q2，使得 q1 始终存储最新的栈状态
        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    // 弹出栈顶元素
    pub fn pop(&mut self) -> Result<T, &str> {
        self.q1.dequeue().ok_or("Stack is empty")
    }

    // 检查栈是否为空
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
