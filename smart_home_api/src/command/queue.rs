/// Очередь для хранения объектов json rpc - запросов / ответов

#[derive(Default, Clone)]
pub struct RPCQueue<T> {
    pub(crate) messages: Vec<T>,
}

impl<T> RPCQueue<T> {
    pub fn pop(&mut self) -> Option<T> {
        self.messages.pop()
    }

    pub fn push(&mut self, batch: Vec<T>) {
        self.messages.extend(batch);
    }

    pub fn reset(&mut self) {
        self.messages.clear();
    }
}
