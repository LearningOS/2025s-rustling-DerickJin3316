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

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
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
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>,
    which: usize,
    size: usize
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new(),
            which: 0usize,
            size: 0usize
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        if self.which == 0 {
            self.q1.enqueue(elem);
        }
        else {
            self.q2.enqueue(elem);
        }
        self.size += 1;
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        if self.size > 0 {
            self.size -= 1;
            if self.which == 0 {
                self.which = 1;
                for _ in 0..self.q1.size() - 1 {
                    self.q2.enqueue(self.q1.dequeue().unwrap());
                }
                return Ok(self.q1.dequeue().unwrap());
            }
            else {
                self.which = 0;
                for _ in 0..self.q2.size() - 1 {
                    self.q1.enqueue(self.q2.dequeue().unwrap());
                }
                return Ok(self.q2.dequeue().unwrap());
            }
        }
		Err("Stack is empty")
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        if self.size == 0 {true} else {false}
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
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