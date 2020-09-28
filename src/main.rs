struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

fn main() {
    let mut queue: Queue<isize> = Queue::new();
    assert_eq!(queue.is_empty(), true);

    assert_eq!(queue.length(), 0);
    queue.enqueue(1);
    assert_eq!(queue.length(), 1);
    queue.dequeue();
    assert_eq!(queue.length(), 0);



    queue.enqueue(1337);
    queue.enqueue(42);
    assert_eq!(queue.peek(), Some(&1337));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue() {
        let mut queue: Queue<isize> = Queue::new();
        assert_eq!(queue.queue.len(), 0);
        queue.enqueue(1);
        assert_eq!(queue.queue.len(), 1);
    }

    #[test]
    fn dequeue() {
        let mut queue: Queue<isize> = Queue::new();
        assert_eq!(queue.queue.len(), 0);
        queue.enqueue(1);
        assert_eq!(queue.queue.len(), 1);
        queue.dequeue();
        assert_eq!(queue.queue.len(), 0);
    }

    #[test]
    fn length() {
        let mut queue: Queue<isize> = Queue::new();
        assert_eq!(queue.length(), 0);
        queue.enqueue(1);
        assert_eq!(queue.length(), 1);
        queue.dequeue();
        assert_eq!(queue.length(), 0);
    }

    #[test]
    fn is_empty() {
        let mut queue: Queue<isize> = Queue::new();
        assert_eq!(queue.is_empty(), true);
        queue.enqueue(1);
        assert_eq!(queue.is_empty(), false);
        queue.dequeue();
        assert_eq!(queue.is_empty(), true);
    }

    #[test]
    fn peek() {
        let mut queue: Queue<isize> = Queue::new();
        queue.enqueue(1337);
        queue.enqueue(42);
        assert_eq!(queue.peek(), Some(&1337));
    }
}
