use utoipa::{openapi::OpenApi, Modify};

pub(crate) struct ModifyInfo;

impl Modify for ModifyInfo {
    fn modify(&self, openapi: &mut OpenApi) {
        openapi.info.title = "Growing Cultures API".into();
        if let Some(description) = &mut openapi.info.description {
            *description += "\n\n";
            *description += include_str!("../CHANGES.md");
        }
    }
}
