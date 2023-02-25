use reqwest::Method;
use serde::{Serialize, Deserialize};

pub trait ApiRequest: Serialize {
    type Output: for<'d> Deserialize<'d>;

    const METHOD: Method;
    
    const ENDPOINT: &'static str;

    // XXX Not implemented because not needed
    // fn method(&self) -> Method;
    // fn endpoint<U: IntoUrl>(&self) -> U;
}
