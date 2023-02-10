trait Dummy {
    fn dummy() -> Self;
}

impl Dummy for Entry {
    fn dummy() -> Self {
        Self {
            book: Book::dummy(),
            annotations: vec![
                Annotation::dummy(),
                Annotation::dummy(),
                Annotation::dummy(),
            ],
        }
    }
}

impl Dummy for Book {
    fn dummy() -> Self {
        Self {
            title: String::default(),
            author: String::default(),
            tags: BTreeSet::from_iter(["1", "2"]).map(String::from),
            metadata: crate::models::book::BookMetadata::default(),
        }
    }
}

impl Dummy for Annotation {
    fn dummy() -> Self {
        Self {
            body: todo!(),
            style: todo!(),
            notes: todo!(),
            tags: todo!(),
            metadata: crate::models::annotation::AnnotationMetadata::default(),
        }
    }
}
