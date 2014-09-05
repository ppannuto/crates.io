use std::fmt::Show;

use conduit::{mod, Handler, Request, Response};
use conduit_middleware::Middleware;
use conduit_test::MockRequest;

use cargo_registry::db::RequestTransaction;

#[deriving(Decodable)]
struct PackageList { packages: Vec<Package>, meta: PackageMeta }
#[deriving(Decodable)]
struct PackageMeta { total: int, page: int }
#[deriving(Decodable)]
struct Package { name: String, id: String }
#[deriving(Decodable)]
struct PackageResponse { package: Package }
#[deriving(Decodable)]
struct BadPackage { ok: bool, error: String }

#[test]
fn index() {
    let mut middle = ::middleware();
    let mut req = MockRequest::new(conduit::Get, "/packages");
    let mut response = ok_resp!(middle.call(&mut req));
    let json: PackageList = ::json(&mut response);
    assert_eq!(json.packages.len(), 0);
    assert_eq!(json.meta.total, 0);
    assert_eq!(json.meta.page, 0);

    let pkg = ::package();
    middle.add(::middleware::MockPackage(pkg.clone()));
    let mut response = ok_resp!(middle.call(&mut req));
    let json: PackageList = ::json(&mut response);
    assert_eq!(json.packages.len(), 1);
    assert_eq!(json.meta.total, 1);
    assert_eq!(json.meta.page, 0);
    assert_eq!(json.packages[0].name, pkg.name);
    assert_eq!(json.packages[0].id, pkg.name);
}

#[test]
fn show() {
    let mut middle = ::middleware();
    let pkg = ::package();
    middle.add(::middleware::MockPackage(pkg.clone()));
    let mut req = MockRequest::new(conduit::Get,
                                   format!("/packages/{}", pkg.name).as_slice());
    let mut response = ok_resp!(middle.call(&mut req));
    let json: PackageResponse = ::json(&mut response);
    assert_eq!(json.package.name, pkg.name);
    assert_eq!(json.package.id, pkg.name);
}

fn new_req(api_token: &str, pkg: &str, version: &str, deps: &[&str])
           -> MockRequest {
    let mut req = MockRequest::new(conduit::Post, "/packages/new");
    req.header("X-Cargo-Auth", api_token)
       .header("X-Cargo-Pkg-Name", pkg)
       .header("X-Cargo-Pkg-Version", version)
       .with_body("")
       .header("Content-Type", "application/x-tar")
       .header("Content-Encoding", "x-gzip");
    drop(deps);
    return req;
}

#[test]
fn new_wrong_token() {
    let mut middle = ::middleware();
    middle.add(::middleware::MockUser(::user()));
    let mut req = new_req("wrong-token", "foo", "1.0.0", []);
    let response = t_resp!(middle.call(&mut req));
    assert_eq!(response.status.val0(), 404);
}

#[test]
fn new_bad_names() {
    let mut middle = ::middleware();
    middle.add(::middleware::MockUser(::user()));
    let mut req = new_req(::user().api_token.as_slice(), "bad name", "1.0.0", []);
    let mut response = ok_resp!(middle.call(&mut req));
    let json: BadPackage = ::json(&mut response);
    assert!(!json.ok);
}
