use super::super::errors::SportMonksError;
use super::super::models::{Comment,Wrapper};
use super::super::gateway::{Gateway,Options};


pub struct CommentariesGateway {
    gateway: Gateway
}

impl CommentariesGateway {
    pub fn new(gateway: Gateway) -> CommentariesGateway {
        CommentariesGateway { gateway }
    }

    pub fn all_for(&self, fixture_id: u64) -> Result<Wrapper<Vec<Comment>>, SportMonksError> {
        self.gateway.get(&format!("/commentaries/fixture/{}", fixture_id), Options::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use std::fs;
    use std::path::Path;

    #[test]
    fn it_returns_all_the_commentariess() {
        let body = fs::read_to_string(Path::new("src/support/comments/all_for.json")).expect("Fixtures:");
        let m = mock("GET", "/commentaries/fixture/10420302?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = CommentariesGateway::new(Gateway::new("1234".into()));
        let response = instance.all_for(10420302);
        m.assert();

        assert!(response.is_ok());
        let result = response.unwrap();
        assert_eq!(result.data[0].fixture_id, 10420302);
        assert_eq!(result.data[0].important, false);
        assert_eq!(result.data[0].order, Some(130));
        assert_eq!(result.data[0].goal, false);
        assert_eq!(result.data[0].minute, Some(90));
        assert_eq!(result.data[0].extra_minute, Some(4));
        assert_eq!(result.data[0].comment, "Thats all. Game finished -  Roma 2, Bologna 1.");
    }
}