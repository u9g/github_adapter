use std::rc::Rc;

use octocrab::{models::issues::Issue, Page};

use super::adapter::{octocrab, runtime};

pub struct IssueIterator {
    repo_owner: Rc<str>,
    repo_name: Rc<str>,
    current_page: Page<Issue>,
    item_ix_in_page: usize,
    current_page_ix: u32,
    max_page_ix: u32,
}

impl IssueIterator {
    pub fn new(repo_owner: Rc<str>, repo_name: Rc<str>) -> Self {
        let current_page = Self::get_page(repo_owner.clone(), repo_name.clone(), 1);
        Self {
            repo_owner,
            repo_name,
            item_ix_in_page: 0,
            current_page_ix: 0,
            max_page_ix: current_page
                .number_of_pages()
                .expect("to get number of issue pages"),
            current_page,
        }
    }

    fn get_page(repo_owner: Rc<str>, repo_name: Rc<str>, page: u32) -> Page<Issue> {
        let _enter = runtime().enter();

        runtime()
            .block_on(
                octocrab()
                    .issues(repo_owner.to_string(), repo_name.to_string())
                    .list()
                    .page(page)
                    .per_page(100)
                    .send(),
            )
            .expect("to be able to find issues")
    }

    fn get_next_page(&mut self) {
        println!("\tget next page");
        self.current_page = Self::get_page(
            self.repo_owner.clone(),
            self.repo_name.clone(),
            self.current_page_ix,
        );
    }
}

impl Iterator for IssueIterator {
    type Item = Issue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.item_ix_in_page < self.current_page.items.len() {
            // TODO: this clones, which is not ideal
            println!("{}", self.current_page.items[self.item_ix_in_page].title);
            self.item_ix_in_page += 1;
            Some(self.current_page.items[self.item_ix_in_page - 1].clone())
        } else if self.current_page_ix < self.max_page_ix
            && self.item_ix_in_page == self.current_page.items.len()
        {
            self.get_next_page();
            self.current_page_ix += 1;
            self.item_ix_in_page = 0;
            self.next()
        } else {
            None
        }
    }
}
