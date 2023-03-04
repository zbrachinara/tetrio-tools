use serde::{Serialize, Deserialize};

use crate::User;


#[derive(Serialize, Deserialize)]
pub struct Ttr<'a> {
    #[serde(borrow)]
    pub user: User<'a>
}