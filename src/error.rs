#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("The '{1}' doesn't support an element of {0}.")]
    NotSupportElement(String, String),

    #[error("The '{0}' is unknown or can't be used as a child of the page.")]
    UnknownChild(String),

    #[error("The page component should has the defined width and height.")]
    UndefinedPageSize(),

    #[error("The {0} is a required attribute for {1}.")]
    RequiredAttribute(String, String),

    #[error("The image of {0} is not found in assets.")]
    ImageAssetNotFound(String),

    #[error("The font of {0} is not found in assets.")]
    FontAssetNotFound(String),

    #[error("The {0} component can't have children.")]
    InvalidChildren(String),
}
