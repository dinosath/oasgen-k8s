//
// This file is generated from openapi specification. Please do not modify it.
// It should be .gitignored
//
#![allow(warnings)]
#![allow(clippy::all)]

use axum::{
    body::{Body, BoxBody},
    http::{HeaderValue, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{
    de::{self, IntoDeserializer},
    Deserialize,
};
use std::fmt;
use validator::Validate;

pub enum UpdatePetResponse {
    Status200(super::model::UpdatePet200),
    Status400,
    Status404,
    Status405,
}

impl From<UpdatePetResponse> for Response<BoxBody> {
    fn from(response: UpdatePetResponse) -> Response<BoxBody> {
        match response {
            UpdatePetResponse::Status200(model) => {
                (StatusCode::from_u16(200).unwrap(), Json(model)).into_response()
            }

            UpdatePetResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),

            UpdatePetResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),

            UpdatePetResponse::Status405 => StatusCode::from_u16(405).unwrap().into_response(),
        }
    }
}

pub enum AddPetResponse {
    Status200(super::model::UpdatePetRequestBody),
    Status405,
}

impl From<AddPetResponse> for Response<BoxBody> {
    fn from(response: AddPetResponse) -> Response<BoxBody> {
        match response {
            AddPetResponse::Status200(model) => {
                (StatusCode::from_u16(200).unwrap(), Json(model)).into_response()
            }

            AddPetResponse::Status405 => StatusCode::from_u16(405).unwrap().into_response(),
        }
    }
}

/// GET /pet/findByStatus
/// query parameters
#[derive(Validate, Deserialize, Default)]
pub struct FindPetsByStatusQuery {
    #[serde(rename = "status")]
    pub status: Option<super::model::FindPetsByStatusStatusQueryVariant>,
}

pub enum FindPetsByStatusResponse {
    Status200(Vec<super::model::UpdatePetRequestBody>),
    Status400,
}

impl From<FindPetsByStatusResponse> for Response<BoxBody> {
    fn from(response: FindPetsByStatusResponse) -> Response<BoxBody> {
        match response {
            FindPetsByStatusResponse::Status200(model) => {
                (StatusCode::from_u16(200).unwrap(), Json(model)).into_response()
            }

            FindPetsByStatusResponse::Status400 => {
                StatusCode::from_u16(400).unwrap().into_response()
            }
        }
    }
}

/// GET /pet/findByTags
/// query parameters
#[derive(Validate, Deserialize, Default)]
pub struct FindPetsByTagsQuery {
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

pub enum FindPetsByTagsResponse {
    Status200(Vec<super::model::UpdatePetRequestBody>),
    Status400,
}

impl From<FindPetsByTagsResponse> for Response<BoxBody> {
    fn from(response: FindPetsByTagsResponse) -> Response<BoxBody> {
        match response {
            FindPetsByTagsResponse::Status200(model) => {
                (StatusCode::from_u16(200).unwrap(), Json(model)).into_response()
            }

            FindPetsByTagsResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
        }
    }
}

/// GET /pet/{petId}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct GetPetByIdPath {
    #[serde(rename = "petId")]
    pub pet_id: i64,
}

pub enum GetPetByIdResponse {
    Status200(super::model::UpdatePetRequestBody),
    Status400,
    Status404,
}

impl From<GetPetByIdResponse> for Response<BoxBody> {
    fn from(response: GetPetByIdResponse) -> Response<BoxBody> {
        match response {
            GetPetByIdResponse::Status200(model) => {
                (StatusCode::from_u16(200).unwrap(), Json(model)).into_response()
            }

            GetPetByIdResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),

            GetPetByIdResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
        }
    }
}

/// POST /pet/{petId}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct UpdatePetWithFormPath {
    #[serde(rename = "petId")]
    pub pet_id: i64,
}

/// POST /pet/{petId}
/// query parameters
#[derive(Validate, Deserialize, Default)]
pub struct UpdatePetWithFormQuery {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "status")]
    pub status: Option<String>,
}

pub enum UpdatePetWithFormResponse {
    Status405,
}

impl From<UpdatePetWithFormResponse> for Response<BoxBody> {
    fn from(response: UpdatePetWithFormResponse) -> Response<BoxBody> {
        match response {
            UpdatePetWithFormResponse::Status405 => {
                StatusCode::from_u16(405).unwrap().into_response()
            }
        }
    }
}

/// DELETE /pet/{petId}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct DeletePetPath {
    #[serde(rename = "petId")]
    pub pet_id: i64,
}

/// DELETE /pet/{petId}
/// request headers
#[derive(Validate, Deserialize)]
pub struct DeletePetRequestHeaders {
    #[serde(rename = "api_key")]
    pub api_key: Option<String>,
}

pub enum DeletePetResponse {
    Status400,
}

impl From<DeletePetResponse> for Response<BoxBody> {
    fn from(response: DeletePetResponse) -> Response<BoxBody> {
        match response {
            DeletePetResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
        }
    }
}

/// POST /pet/{petId}/uploadImage
/// path parameters
#[derive(Validate, Deserialize)]
pub struct UploadFilePath {
    #[serde(rename = "petId")]
    pub pet_id: i64,
}

/// POST /pet/{petId}/uploadImage
/// query parameters
#[derive(Validate, Deserialize, Default)]
pub struct UploadFileQuery {
    #[serde(rename = "additionalMetadata")]
    pub additional_metadata: Option<String>,
}

pub enum UploadFileResponse {
    Status200(super::model::UploadFile200),
}

impl From<UploadFileResponse> for Response<BoxBody> {
    fn from(response: UploadFileResponse) -> Response<BoxBody> {
        match response {
            UploadFileResponse::Status200(model) => {
                (StatusCode::from_u16(200).unwrap(), Json(model)).into_response()
            }
        }
    }
}

impl From<super::model::UploadFile200> for UploadFileResponse {
    fn from(data: super::model::UploadFile200) -> Self {
        Self::Status200(data)
    }
}

pub enum GetInventoryResponse {
    Status200(BTreeMap<String, i64>),
}

impl From<GetInventoryResponse> for Response<BoxBody> {
    fn from(response: GetInventoryResponse) -> Response<BoxBody> {
        match response {
            GetInventoryResponse::Status200(model) => {
                (StatusCode::from_u16(200).unwrap(), Json(model)).into_response()
            }
        }
    }
}

impl From<BTreeMap<String, i64>> for GetInventoryResponse {
    fn from(data: BTreeMap<String, i64>) -> Self {
        Self::Status200(data)
    }
}

pub enum PlaceOrderResponse {
    Status200(super::model::PlaceOrder200),
    Status405,
}

impl From<PlaceOrderResponse> for Response<BoxBody> {
    fn from(response: PlaceOrderResponse) -> Response<BoxBody> {
        match response {
            PlaceOrderResponse::Status200(model) => {
                (StatusCode::from_u16(200).unwrap(), Json(model)).into_response()
            }

            PlaceOrderResponse::Status405 => StatusCode::from_u16(405).unwrap().into_response(),
        }
    }
}

impl From<super::model::PlaceOrder200> for PlaceOrderResponse {
    fn from(data: super::model::PlaceOrder200) -> Self {
        Self::Status200(data)
    }
}

/// GET /store/order/{orderId}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct GetOrderByIdPath {
    #[serde(rename = "orderId")]
    pub order_id: i64,
}

pub enum GetOrderByIdResponse {
    Status200(super::model::PlaceOrderRequestBody),
    Status400,
    Status404,
}

impl From<GetOrderByIdResponse> for Response<BoxBody> {
    fn from(response: GetOrderByIdResponse) -> Response<BoxBody> {
        match response {
            GetOrderByIdResponse::Status200(model) => {
                (StatusCode::from_u16(200).unwrap(), Json(model)).into_response()
            }

            GetOrderByIdResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),

            GetOrderByIdResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
        }
    }
}

/// DELETE /store/order/{orderId}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct DeleteOrderPath {
    #[serde(rename = "orderId")]
    pub order_id: i64,
}

pub enum DeleteOrderResponse {
    Status400,
    Status404,
}

impl From<DeleteOrderResponse> for Response<BoxBody> {
    fn from(response: DeleteOrderResponse) -> Response<BoxBody> {
        match response {
            DeleteOrderResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),

            DeleteOrderResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
        }
    }
}

pub enum CreateUserResponse {
    Status0(super::model::CreateUser0),
}

impl From<CreateUserResponse> for Response<BoxBody> {
    fn from(response: CreateUserResponse) -> Response<BoxBody> {
        match response {
            CreateUserResponse::Status0(model) => {
                (StatusCode::from_u16(0).unwrap(), Json(model)).into_response()
            }
        }
    }
}

pub enum CreateUsersWithListInputResponse {
    Status200(super::model::CreateUserRequestBody),
    Status0,
}

impl From<CreateUsersWithListInputResponse> for Response<BoxBody> {
    fn from(response: CreateUsersWithListInputResponse) -> Response<BoxBody> {
        match response {
            CreateUsersWithListInputResponse::Status200(model) => {
                (StatusCode::from_u16(200).unwrap(), Json(model)).into_response()
            }

            CreateUsersWithListInputResponse::Status0 => {
                StatusCode::from_u16(0).unwrap().into_response()
            }
        }
    }
}

/// GET /user/login
/// query parameters
#[derive(Validate, Deserialize, Default)]
pub struct LoginUserQuery {
    #[serde(rename = "username")]
    pub username: Option<String>,

    #[serde(rename = "password")]
    pub password: Option<String>,
}

/// GET /user/login
/// response headers
#[derive(Deserialize)]
pub struct LoginUserResponse200Headers {
    #[serde(rename = "X-Rate-Limit")]
    pub x_rate_limit: Option<i64>,

    #[serde(rename = "X-Expires-After")]
    pub x_expires_after: Option<DateTime<Utc>>,
}

pub enum LoginUserResponse {
    Status200(StringWithHeaders),
    Status400,
}

impl From<LoginUserResponse> for Response<BoxBody> {
    fn from(response: LoginUserResponse) -> Response<BoxBody> {
        match response {
            LoginUserResponse::Status200(model) => {
                let mut response = model.into_response();
                let headers = response.headers_mut();

                if let Some(header) = model.headers.x_rate_limit {
                    headers.insert("x-rate-limit", HeaderValue::from_str(&header).unwrap());
                }

                if let Some(header) = model.headers.x_expires_after {
                    headers.insert("x-expires-after", HeaderValue::from_str(&header).unwrap());
                }

                response
            }
            LoginUserResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),
        }
    }
}

pub enum LogoutUserResponse {
    Status0,
}

impl From<LogoutUserResponse> for Response<BoxBody> {
    fn from(response: LogoutUserResponse) -> Response<BoxBody> {
        match response {
            LogoutUserResponse::Status0 => StatusCode::from_u16(0).unwrap().into_response(),
        }
    }
}

/// GET /user/{username}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct GetUserByNamePath {
    #[serde(rename = "username")]
    pub username: String,
}

pub enum GetUserByNameResponse {
    Status200(super::model::CreateUserRequestBody),
    Status400,
    Status404,
}

impl From<GetUserByNameResponse> for Response<BoxBody> {
    fn from(response: GetUserByNameResponse) -> Response<BoxBody> {
        match response {
            GetUserByNameResponse::Status200(model) => {
                (StatusCode::from_u16(200).unwrap(), Json(model)).into_response()
            }

            GetUserByNameResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),

            GetUserByNameResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
        }
    }
}

/// PUT /user/{username}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct UpdateUserPath {
    #[serde(rename = "username")]
    pub username: String,
}

pub enum UpdateUserResponse {
    Status0,
}

impl From<UpdateUserResponse> for Response<BoxBody> {
    fn from(response: UpdateUserResponse) -> Response<BoxBody> {
        match response {
            UpdateUserResponse::Status0 => StatusCode::from_u16(0).unwrap().into_response(),
        }
    }
}

/// DELETE /user/{username}
/// path parameters
#[derive(Validate, Deserialize)]
pub struct DeleteUserPath {
    #[serde(rename = "username")]
    pub username: String,
}

pub enum DeleteUserResponse {
    Status400,
    Status404,
}

impl From<DeleteUserResponse> for Response<BoxBody> {
    fn from(response: DeleteUserResponse) -> Response<BoxBody> {
        match response {
            DeleteUserResponse::Status400 => StatusCode::from_u16(400).unwrap().into_response(),

            DeleteUserResponse::Status404 => StatusCode::from_u16(404).unwrap().into_response(),
        }
    }
}

/// all convertable Result are acceptable
macro_rules! impl_from_result {
    ($t:ident) => {
        impl<O, E> From<Result<O, E>> for $t
        where
            O: Into<$t>,
            E: Into<$t>,
        {
            fn from(res: Result<O, E>) -> Self {
                match res {
                    Ok(r) => r.into(),
                    Err(e) => e.into(),
                }
            }
        }
    };
}

impl_from_result!(UpdatePetResponse);
impl_from_result!(AddPetResponse);
impl_from_result!(FindPetsByStatusResponse);
impl_from_result!(FindPetsByTagsResponse);
impl_from_result!(GetPetByIdResponse);
impl_from_result!(UpdatePetWithFormResponse);
impl_from_result!(DeletePetResponse);
impl_from_result!(UploadFileResponse);
impl_from_result!(GetInventoryResponse);
impl_from_result!(PlaceOrderResponse);
impl_from_result!(GetOrderByIdResponse);
impl_from_result!(DeleteOrderResponse);
impl_from_result!(CreateUserResponse);
impl_from_result!(CreateUsersWithListInputResponse);
impl_from_result!(LoginUserResponse);
impl_from_result!(LogoutUserResponse);
impl_from_result!(GetUserByNameResponse);
impl_from_result!(UpdateUserResponse);
impl_from_result!(DeleteUserResponse);

/// custom deserializer for query parameters /users?id=3,4,5
/// style = form, explode = false according to https://swagger.io/docs/specification/serialization/
pub fn deserialized_id_list<'de, D, I>(deserializer: D) -> std::result::Result<Vec<I>, D::Error>
where
    D: de::Deserializer<'de>,
    I: de::DeserializeOwned,
{
    struct StringVecVisitor<I>(std::marker::PhantomData<I>);

    impl<'de, I> de::Visitor<'de> for StringVecVisitor<I>
    where
        I: de::DeserializeOwned,
    {
        type Value = Vec<I>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a string containing a list")
        }

        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            let mut ids = Vec::new();
            for id in v.split(',') {
                let id = I::deserialize(id.into_deserializer())?;
                ids.push(id);
            }
            Ok(ids)
        }
    }

    deserializer.deserialize_any(StringVecVisitor(std::marker::PhantomData::<I>))
}
