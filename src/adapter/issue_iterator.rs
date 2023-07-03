use std::rc::Rc;

use async_std::task;
use octorust::types::IssueSimple;

use super::util::client;

pub struct IssueIterator {
    repo_owner: Rc<str>,
    repo_name: Rc<str>,
    current_page: Vec<IssueSimple>,
    item_ix_in_page: usize,
    current_page_ix: i64,
    max_page_ix: i64,
}

impl IssueIterator {
    pub fn new(repo_owner: Rc<str>, repo_name: Rc<str>) -> Self {
        let current_page = Self::get_page(repo_owner.clone(), repo_name.clone(), 1);
        Self {
            repo_owner,
            repo_name,
            item_ix_in_page: 0,
            current_page_ix: 1,
            max_page_ix: 9999,
            current_page,
        }
    }

    fn get_page(owner: Rc<str>, name: Rc<str>, page: i64) -> Vec<IssueSimple> {
        task::block_on(client().issues().list_for_repo(
            &owner,
            &name,
            "",
            octorust::types::IssuesListState::All,
            "",
            "",
            "",
            "",
            octorust::types::IssuesListSort::Updated,
            octorust::types::Order::Desc,
            None,
            100,
            page,
        ))
        .expect("to get page of issues")
    }

    fn incremenet_page_and_get_next_page(&mut self) {
        self.current_page_ix += 1;
        self.current_page = Self::get_page(
            self.repo_owner.clone(),
            self.repo_name.clone(),
            self.current_page_ix,
        );
    }
}

impl Iterator for IssueIterator {
    type Item = IssueSimple;

    fn next(&mut self) -> Option<Self::Item> {
        if self.item_ix_in_page < self.current_page.len() {
            self.item_ix_in_page += 1;
            Some(self.current_page[self.item_ix_in_page - 1].clone())
        } else if self.current_page_ix < self.max_page_ix
            && self.item_ix_in_page == self.current_page.len()
        {
            self.incremenet_page_and_get_next_page();
            self.item_ix_in_page = 0;
            self.next()
        } else {
            None
        }
    }
}
