//
// This file is generated from openapi specification. Please do not modify it.
// It should be .gitignored
//
#![allow(warnings)]
#![allow(clippy::all)]

use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeMap;
use validator::Validate;

use chrono::{DateTime, Utc};

lazy_static::lazy_static! {
   static ref UUID: Regex = Regex::new(r"^\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b$").unwrap();
   static ref FLOAT: Regex = Regex::new(r"^[0-9\.]+$").unwrap();

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum OrderStatusVariant {
    #[serde(rename = "placed")]
    Placed,

    #[serde(rename = "approved")]
    Approved,

    #[serde(rename = "delivered")]
    Delivered,
}

impl std::string::ToString for OrderStatusVariant {
    fn to_string(&self) -> String {
        match self {
            Self::Placed => "placed",
            Self::Approved => "approved",
            Self::Delivered => "delivered",
        }
        .to_string()
    }
}

impl std::str::FromStr for OrderStatusVariant {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "placed" => Ok(Self::Placed),
            "approved" => Ok(Self::Approved),
            "delivered" => Ok(Self::Delivered),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct Order {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "petId", skip_serializing_if = "Option::is_none")]
    pub pet_id: Option<i64>,

    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,

    #[serde(rename = "shipDate", skip_serializing_if = "Option::is_none")]
    pub ship_date: Option<DateTime<Utc>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatusVariant>,
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

impl Order {
    pub fn new(
        id: Option<i64>,
        pet_id: Option<i64>,
        quantity: Option<i64>,
        ship_date: Option<DateTime<Utc>>,
        status: Option<OrderStatusVariant>,
        complete: Option<bool>,
    ) -> Self {
        Self {
            id,
            pet_id,
            quantity,
            ship_date,
            status,
            complete,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct CustomerAddress {
    #[serde(rename = "street", skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "zip", skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl CustomerAddress {
    pub fn new(
        street: Option<String>,
        city: Option<String>,
        state: Option<String>,
        zip: Option<String>,
    ) -> Self {
        Self {
            street,
            city,
            state,
            zip,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct Customer {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[validate]
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<CustomerAddress>>,
}

impl Customer {
    pub fn new(
        id: Option<i64>,
        username: Option<String>,
        address: Option<Vec<CustomerAddress>>,
    ) -> Self {
        Self {
            id,
            username,
            address,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct Address {
    #[serde(rename = "street", skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "zip", skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl Address {
    pub fn new(
        street: Option<String>,
        city: Option<String>,
        state: Option<String>,
        zip: Option<String>,
    ) -> Self {
        Self {
            street,
            city,
            state,
            zip,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct Category {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Category {
    pub fn new(id: Option<i64>, name: Option<String>) -> Self {
        Self { id, name }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct User {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(rename = "userStatus", skip_serializing_if = "Option::is_none")]
    pub user_status: Option<i64>,
}

impl User {
    pub fn new(
        id: Option<i64>,
        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        password: Option<String>,
        phone: Option<String>,
        user_status: Option<i64>,
    ) -> Self {
        Self {
            id,
            username,
            first_name,
            last_name,
            email,
            password,
            phone,
            user_status,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct Tag {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Tag {
    pub fn new(id: Option<i64>, name: Option<String>) -> Self {
        Self { id, name }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct PetCategory {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PetCategory {
    pub fn new(id: Option<i64>, name: Option<String>) -> Self {
        Self { id, name }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct PetTags {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PetTags {
    pub fn new(id: Option<i64>, name: Option<String>) -> Self {
        Self { id, name }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PetStatusVariant {
    #[serde(rename = "available")]
    Available,

    #[serde(rename = "pending")]
    Pending,

    #[serde(rename = "sold")]
    Sold,
}

impl std::string::ToString for PetStatusVariant {
    fn to_string(&self) -> String {
        match self {
            Self::Available => "available",
            Self::Pending => "pending",
            Self::Sold => "sold",
        }
        .to_string()
    }
}

impl std::str::FromStr for PetStatusVariant {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "available" => Ok(Self::Available),
            "pending" => Ok(Self::Pending),
            "sold" => Ok(Self::Sold),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub struct Pet {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name")]
    pub name: String,
    #[validate]
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<PetCategory>,
    #[serde(rename = "photoUrls")]
    pub photo_urls: Vec<String>,
    #[validate]
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<PetTags>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<PetStatusVariant>,
}

impl Pet {
    pub fn new(
        id: Option<i64>,
        name: String,
        category: Option<PetCategory>,
        photo_urls: Vec<String>,
        tags: Option<Vec<PetTags>>,
        status: Option<PetStatusVariant>,
    ) -> Self {
        Self {
            id,
            name,
            category,
            photo_urls,
            tags,
            status,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct ApiResponse {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ApiResponse {
    pub fn new(code: Option<i64>, type_: Option<String>, message: Option<String>) -> Self {
        Self {
            code,
            type_,
            message,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct PetRequestCategory {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PetRequestCategory {
    pub fn new(id: Option<i64>, name: Option<String>) -> Self {
        Self { id, name }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct PetRequestTags {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PetRequestTags {
    pub fn new(id: Option<i64>, name: Option<String>) -> Self {
        Self { id, name }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PetRequestStatusVariant {
    #[serde(rename = "available")]
    Available,

    #[serde(rename = "pending")]
    Pending,

    #[serde(rename = "sold")]
    Sold,
}

impl std::string::ToString for PetRequestStatusVariant {
    fn to_string(&self) -> String {
        match self {
            Self::Available => "available",
            Self::Pending => "pending",
            Self::Sold => "sold",
        }
        .to_string()
    }
}

impl std::str::FromStr for PetRequestStatusVariant {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "available" => Ok(Self::Available),
            "pending" => Ok(Self::Pending),
            "sold" => Ok(Self::Sold),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub struct PetRequest {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name")]
    pub name: String,
    #[validate]
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<PetRequestCategory>,
    #[serde(rename = "photoUrls")]
    pub photo_urls: Vec<String>,
    #[validate]
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<PetRequestTags>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<PetRequestStatusVariant>,
}

impl PetRequest {
    pub fn new(
        id: Option<i64>,
        name: String,
        category: Option<PetRequestCategory>,
        photo_urls: Vec<String>,
        tags: Option<Vec<PetRequestTags>>,
        status: Option<PetRequestStatusVariant>,
    ) -> Self {
        Self {
            id,
            name,
            category,
            photo_urls,
            tags,
            status,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct UserArrayRequest {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(rename = "userStatus", skip_serializing_if = "Option::is_none")]
    pub user_status: Option<i64>,
}

impl UserArrayRequest {
    pub fn new(
        id: Option<i64>,
        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        password: Option<String>,
        phone: Option<String>,
        user_status: Option<i64>,
    ) -> Self {
        Self {
            id,
            username,
            first_name,
            last_name,
            email,
            password,
            phone,
            user_status,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub struct UpdatePet200 {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name")]
    pub name: String,
    #[validate]
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<PetRequestCategory>,
    #[serde(rename = "photoUrls")]
    pub photo_urls: Vec<String>,
    #[validate]
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<PetRequestTags>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<PetRequestStatusVariant>,
}

impl UpdatePet200 {
    pub fn new(
        id: Option<i64>,
        name: String,
        category: Option<PetRequestCategory>,
        photo_urls: Vec<String>,
        tags: Option<Vec<PetRequestTags>>,
        status: Option<PetRequestStatusVariant>,
    ) -> Self {
        Self {
            id,
            name,
            category,
            photo_urls,
            tags,
            status,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub struct UpdatePetRequestBody {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name")]
    pub name: String,
    #[validate]
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<PetRequestCategory>,
    #[serde(rename = "photoUrls")]
    pub photo_urls: Vec<String>,
    #[validate]
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<PetRequestTags>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<PetRequestStatusVariant>,
}

impl UpdatePetRequestBody {
    pub fn new(
        id: Option<i64>,
        name: String,
        category: Option<PetRequestCategory>,
        photo_urls: Vec<String>,
        tags: Option<Vec<PetRequestTags>>,
        status: Option<PetRequestStatusVariant>,
    ) -> Self {
        Self {
            id,
            name,
            category,
            photo_urls,
            tags,
            status,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FindPetsByStatusStatusQueryVariant {
    #[serde(rename = "available")]
    Available,

    #[serde(rename = "pending")]
    Pending,

    #[serde(rename = "sold")]
    Sold,
}

impl std::string::ToString for FindPetsByStatusStatusQueryVariant {
    fn to_string(&self) -> String {
        match self {
            Self::Available => "available",
            Self::Pending => "pending",
            Self::Sold => "sold",
        }
        .to_string()
    }
}

impl std::str::FromStr for FindPetsByStatusStatusQueryVariant {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "available" => Ok(Self::Available),
            "pending" => Ok(Self::Pending),
            "sold" => Ok(Self::Sold),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct UploadFile200 {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl UploadFile200 {
    pub fn new(code: Option<i64>, type_: Option<String>, message: Option<String>) -> Self {
        Self {
            code,
            type_,
            message,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PlaceOrder200StatusVariant {
    #[serde(rename = "placed")]
    Placed,

    #[serde(rename = "approved")]
    Approved,

    #[serde(rename = "delivered")]
    Delivered,
}

impl std::string::ToString for PlaceOrder200StatusVariant {
    fn to_string(&self) -> String {
        match self {
            Self::Placed => "placed",
            Self::Approved => "approved",
            Self::Delivered => "delivered",
        }
        .to_string()
    }
}

impl std::str::FromStr for PlaceOrder200StatusVariant {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "placed" => Ok(Self::Placed),
            "approved" => Ok(Self::Approved),
            "delivered" => Ok(Self::Delivered),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct PlaceOrder200 {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "petId", skip_serializing_if = "Option::is_none")]
    pub pet_id: Option<i64>,

    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,

    #[serde(rename = "shipDate", skip_serializing_if = "Option::is_none")]
    pub ship_date: Option<DateTime<Utc>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<PlaceOrder200StatusVariant>,
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

impl PlaceOrder200 {
    pub fn new(
        id: Option<i64>,
        pet_id: Option<i64>,
        quantity: Option<i64>,
        ship_date: Option<DateTime<Utc>>,
        status: Option<PlaceOrder200StatusVariant>,
        complete: Option<bool>,
    ) -> Self {
        Self {
            id,
            pet_id,
            quantity,
            ship_date,
            status,
            complete,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct PlaceOrderRequestBody {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "petId", skip_serializing_if = "Option::is_none")]
    pub pet_id: Option<i64>,

    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,

    #[serde(rename = "shipDate", skip_serializing_if = "Option::is_none")]
    pub ship_date: Option<DateTime<Utc>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<PlaceOrder200StatusVariant>,
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

impl PlaceOrderRequestBody {
    pub fn new(
        id: Option<i64>,
        pet_id: Option<i64>,
        quantity: Option<i64>,
        ship_date: Option<DateTime<Utc>>,
        status: Option<PlaceOrder200StatusVariant>,
        complete: Option<bool>,
    ) -> Self {
        Self {
            id,
            pet_id,
            quantity,
            ship_date,
            status,
            complete,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct CreateUser0 {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(rename = "userStatus", skip_serializing_if = "Option::is_none")]
    pub user_status: Option<i64>,
}

impl CreateUser0 {
    pub fn new(
        id: Option<i64>,
        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        password: Option<String>,
        phone: Option<String>,
        user_status: Option<i64>,
    ) -> Self {
        Self {
            id,
            username,
            first_name,
            last_name,
            email,
            password,
            phone,
            user_status,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
pub struct CreateUserRequestBody {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(rename = "userStatus", skip_serializing_if = "Option::is_none")]
    pub user_status: Option<i64>,
}

impl CreateUserRequestBody {
    pub fn new(
        id: Option<i64>,
        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        password: Option<String>,
        phone: Option<String>,
        user_status: Option<i64>,
    ) -> Self {
        Self {
            id,
            username,
            first_name,
            last_name,
            email,
            password,
            phone,
            user_status,
        }
    }
}

pub fn optional_nullable<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}
