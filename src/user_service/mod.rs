use bson::ordered::OrderedDocument;
use bson::{doc, Bson, Document};
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{error::Error, results::InsertOneResult, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
// this is like a mongodb model
pub struct User {
  pub first_name: String,
  pub last_name: String,
  pub user_name: String,
  pub password: String,
  pub email: String,
}

#[derive(Clone)]
pub struct UserService {
  collection: Collection,
}

// function takes the parameters in  the struct and returns a document with the same fields;
fn build_user(
  first_name: String,
  last_name: String,
  email: String,
  user_name: String,
  password: String,
) -> User {
  User {
    first_name,
    last_name,
    user_name,
    password,
    email,
  }
}

fn user_from_document(document: Document) -> User {
  let mut _first_name = "".to_string();
  let mut _last_name = "".to_string();
  let mut _email = "".to_string();
  let mut _user_name = "".to_string();
  let mut _password = "".to_string();

  if let Some(&Bson::String(ref first_name)) = document.get("firstName") {
    _first_name = first_name.to_string();
  }
  if let Some(&Bson::String(ref last_name)) = document.get("lastName") {
    _last_name = last_name.to_string();
  }
  if let Some(&Bson::String(ref user_name)) = document.get("username") {
    _user_name = user_name.to_string();
  }
  if let Some(&Bson::String(ref email)) = document.get("email") {
    _email = email.to_string();
  }
  if let Some(&Bson::String(ref password)) = document.get("password") {
    _password = password.to_string();
  }
  build_user(_first_name, _last_name, _user_name, _email, _password)
}

fn user_to_document(user: &User) -> Document {
  let User {
    first_name,
    last_name,
    user_name,
    password,
    email,
  } = user;
  doc! {
    "firstName": first_name,
    "lastName": last_name,
    "username": user_name,
    "password": password,
    "email": email
  }
}

impl UserService {
  // creates a collection
  pub fn new(collection: Collection) -> UserService {
    UserService { collection }
  }
  pub fn create(&self, user: &User) -> Result<InsertOneResult, Error> {
    self.collection.insert_one(user_to_document(user), None)
  }

  pub fn get(&self) -> Result<Vec<User>, Error> {
    // find with no options hence we pass none twice;
    let cursor = self.collection.find(None, None).unwrap();
    // The vector will not allocate until elements are pushed onto it.
    let mut data: Vec<User> = Vec::new();
    for result in cursor {
      if let Ok(item) = result {
        data.push(user_from_document(item))
      }
    }
    Ok(data)
  }
}
