use hdk::prelude::*;
use example_integrity::*;

#[hdk_extern]
pub fn get_all_posts() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_posts");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllPosts)?
            .build(),
    )
}
