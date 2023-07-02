use std::rc::Rc;

use octocrab::models::IssueId;

use super::adapter::{octocrab, runtime};

#[derive(Debug, Clone, Copy)]
pub struct Reactions {
    pub plus_one: u32,
    pub minus_one: u32,
    pub laugh: u32,
    pub hooray: u32,
    pub confused: u32,
    pub heart: u32,
    pub rocket: u32,
    pub eyes: u32,

    pub total: u64,
}

impl Reactions {
    pub fn new(repo_owner: Rc<str>, repo_name: Rc<str>, issue_number: u64) -> Self {
        let instant = std::time::Instant::now();
        let _enter = runtime().enter();
        let instance = octocrab();
        let issue = instance.issues(repo_owner.to_string(), repo_name.to_string());

        let mut issue_reactions = vec![];

        let last_reactions = runtime()
            .block_on(issue.list_reactions(issue_number).per_page(100).send())
            .unwrap_or_else(|_| {
                panic!(
                    "expect to be able to get reactions for {}'s repo: {} for issue: {}",
                    &repo_owner.to_string().as_str(),
                    &repo_name.to_string().as_str(),
                    issue_number
                )
            });

        last_reactions
            .items
            .iter()
            .for_each(|reaction| issue_reactions.push(reaction.content.clone()));

        if let Some(number_of_pages) = last_reactions.number_of_pages() {
            // only exists if >1 pages
            for i in 2..number_of_pages {
                runtime()
                    .block_on(
                        issue
                            .list_reactions(issue_number)
                            .per_page(100)
                            .page(i)
                            .send(),
                    )
                    .expect("expect to be able to get reactions")
                    .items
                    .iter()
                    .for_each(|reaction| issue_reactions.push(reaction.content.clone()));
            }
        }

        let mut reactions = Self {
            plus_one: 0,
            minus_one: 0,
            laugh: 0,
            hooray: 0,
            confused: 0,
            heart: 0,
            rocket: 0,
            eyes: 0,
            total: 0,
        };

        for reaction in issue_reactions {
            match reaction {
                octocrab::models::reactions::ReactionContent::Heart => reactions.heart += 1,
                octocrab::models::reactions::ReactionContent::PlusOne => reactions.plus_one += 1,
                octocrab::models::reactions::ReactionContent::Laugh => reactions.laugh += 1,
                octocrab::models::reactions::ReactionContent::Confused => reactions.confused += 1,
                octocrab::models::reactions::ReactionContent::Hooray => reactions.hooray += 1,
                octocrab::models::reactions::ReactionContent::MinusOne => reactions.minus_one += 1,
                octocrab::models::reactions::ReactionContent::Rocket => reactions.rocket += 1,
                octocrab::models::reactions::ReactionContent::Eyes => reactions.eyes += 1,
            }
            reactions.total += 1;
        }
        println!("finish: {:?}", instant.elapsed());
        reactions
    }
}
