use std::fmt::Show;

use conduit::Request;
use conduit_middleware::Middleware;

use cargo_registry::user::User;
use cargo_registry::package::Package;
use cargo_registry::db::RequestTransaction;

pub struct MockUser(pub User);

impl Middleware for MockUser {
    fn before(&self, req: &mut Request) -> Result<(), Box<Show + 'static>> {
        let MockUser(ref u) = *self;
        req.mut_extensions().insert(u.clone());
        User::find_or_insert(req.tx().unwrap(), u.email.as_slice(),
                             u.gh_access_token.as_slice(),
                             u.api_token.as_slice()).unwrap();
        Ok(())
    }
}

pub struct MockPackage(pub Package);

impl Middleware for MockPackage {
    fn before(&self, req: &mut Request) -> Result<(), Box<Show + 'static>> {
        let MockPackage(ref p) = *self;
        Package::find_or_insert(req.tx().unwrap(), p.name.as_slice()).unwrap();
        Ok(())
    }
}
