use std::fmt;

#[derive(Debug)]
pub struct Document<T> {
    id: usize,
    content: T,
    created_at: u64,
    modified_at: Option<u64>,
    version: u32,
}

impl<T: fmt::Display> fmt::Display for Document<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Document {{ id: {}, content: {}, created_at: {}, modified_at: {:?}, version: {} }}",
            self.id, self.content, self.created_at, self.modified_at, self.version
        )
    }
}

impl<T> Document<T> {
    pub fn new(id: usize, content: T, created_at: u64) -> Document<T> {
        Document {
            id,
            content,
            created_at,
            modified_at: None,
            version: 0,
        }
    }

    pub fn update(&self, content: T, modified_at: u64)->Document<T> {
       Document{
              id: self.id,
              content,
              created_at: self.created_at,
              modified_at: Some(modified_at),
              version: self.version + 1,
       }
    }
}

pub trait Storage<T> {
    fn add(&mut self, document: Document<T>);
    fn get(&self, id: usize) -> Option<&Document<T>>;
    fn update(&mut self, id: usize, content: T, modified_at: u64) -> bool;
    fn delete(&mut self, id: usize) -> bool;
}

impl<T> Storage<T> for Vec<Document<T>> {
    fn add(&mut self, document: Document<T>) {
        self.push(document);
    }

    fn get(&self, id: usize) -> Option<&Document<T>> {
        self.iter().find(|doc| doc.id == id)
    }

    fn update(&mut self, id: usize, content: T, modified_at: u64) -> bool {
        if let Some(doc) = self.iter_mut().find(|doc| doc.id == id) {
            *doc = doc.update(content, modified_at);
            true
        } else {
            false
        }
    }

    fn delete(&mut self, id: usize) -> bool {
        if let Some(index) = self.iter().position(|doc| doc.id == id) {
            self.remove(index);
            true
        } else {
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document() {
        let doc = Document::new(1, "Hello, world!", 0);
        assert_eq!(
            format!("{}", doc),
            "Document { id: 1, content: Hello, world!, created_at: 0, modified_at: None, version: 0 }"
        );

        let doc = doc.update("Hello, Rust!", 1);
        assert_eq!(
            format!("{}", doc),
            "Document { id: 1, content: Hello, Rust!, created_at: 0, modified_at: Some(1), version: 1 }"
        );

    }

    #[test]
    fn storage(){
        let mut storage: Vec<Document<&str>> = Vec::new();
        storage.add(Document::new(1, "Hello, world!", 0));
        storage.add(Document::new(2, "Hello, Rust!", 1));

        assert_eq!(storage.get(1).unwrap().content, "Hello, world!");
        assert_eq!(storage.get(2).unwrap().content, "Hello, Rust!");

        assert_eq!(storage.update(1, "Hello, Rust!", 2), true);
        assert_eq!(storage.get(1).unwrap().content, "Hello, Rust!");

        assert_eq!(storage.delete(1), true);
        assert_eq!(storage.delete(1), false);
    }
}
