use std::vec::IntoIter;

pub struct GenericIterator<T, F>
where
    F: Fn(i64) -> Vec<T>,
{
    current_page_iter: IntoIter<T>,
    item_ix_in_page: usize,
    current_page_ix: i64,
    getter: Box<F>,
    can_continue: bool,
}

impl<T, F> GenericIterator<T, F>
where
    F: Fn(i64) -> Vec<T>,
{
    pub fn new(getter: Box<F>) -> Self {
        // keep in sync with increment_page_and_get_next_page code
        let current_page = getter(1);
        Self {
            getter,
            item_ix_in_page: 0,
            current_page_ix: 1,
            can_continue: !current_page.is_empty(),
            current_page_iter: current_page.into_iter(),
        }
    }

    fn increment_page_and_get_next_page(&mut self) {
        self.current_page_ix += 1;
        let new_page = (self.getter)(self.current_page_ix);
        self.can_continue = !new_page.is_empty();
        self.current_page_iter = new_page.into_iter()
    }
}

impl<T, F> Iterator for GenericIterator<T, F>
where
    F: Fn(i64) -> Vec<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.can_continue {
            if let Some(next) = self.current_page_iter.next() {
                Some(next)
            } else {
                self.increment_page_and_get_next_page();
                self.item_ix_in_page = 0;
                self.next()
            }
        } else {
            None
        }
    }
}
