use std::rc::Rc;

use octocrab::models::IssueId;

use tokio::runtime::Runtime;

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

    pub total: u32,
}

impl Reactions {
    pub fn new(
        repo_owner: Rc<str>,
        repo_name: Rc<str>,
        issue_id: IssueId,
        runtime: &Runtime,
    ) -> Self {
        let instance = octocrab::instance();
        let issue = instance.issues(repo_owner.to_string(), repo_name.to_string());

        let mut issue_reactions = vec![];

        let last_reactions = runtime
            .block_on(issue.list_reactions(issue_id.0).per_page(100).send())
            .expect("expect to be able to get reactions");

        last_reactions
            .items
            .iter()
            .for_each(|reaction| issue_reactions.push(reaction.content.clone()));

        for i in 2..last_reactions.number_of_pages().expect("to get # of pages") {
            runtime
                .block_on(
                    issue
                        .list_reactions(issue_id.0)
                        .per_page(100)
                        .page(i)
                        .send(),
                )
                .expect("expect to be able to get reactions")
                .items
                .iter()
                .for_each(|reaction| issue_reactions.push(reaction.content.clone()));
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

        reactions
    }
}
