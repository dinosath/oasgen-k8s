//
// This file is generated from openapi specification. Please do not modify it.
// It should be .gitignored
//
#![allow(warnings)]
#![allow(clippy::all)]

use axum::{
    body::{Body, BoxBody, HttpBody},
    extract::{FromRequest, RequestParts},
    handler::{future::IntoServiceFuture, Handler},
    http::Request,
    response::{IntoResponse, Response},
    routing::{self, on_service, MethodFilter},
    BoxError, Router,
};
use futures::{future::LocalBoxFuture, Future};
use std::{
    convert::Infallible,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use tower_service::Service;

macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!(T1);
        $name!(T1, T2);
        $name!(T1, T2, T3);
        $name!(T1, T2, T3, T4);
        $name!(T1, T2, T3, T4, T5);
    };
}

/// Pet router
pub struct PetRouter {
    pub(crate) router: Router,
}

impl PetRouter {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn update_pet<H, T>(mut self, handler: H) -> Self
    where
        H: UpdatePetHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/pet",
            on_service(MethodFilter::PUT, UpdatePetService::new(handler)),
        );

        self
    }

    pub fn add_pet<H, T>(mut self, handler: H) -> Self
    where
        H: AddPetHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/pet",
            on_service(MethodFilter::POST, AddPetService::new(handler)),
        );

        self
    }

    pub fn find_pets_by_status<H, T>(mut self, handler: H) -> Self
    where
        H: FindPetsByStatusHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/pet/findByStatus",
            on_service(MethodFilter::GET, FindPetsByStatusService::new(handler)),
        );

        self
    }

    pub fn find_pets_by_tags<H, T>(mut self, handler: H) -> Self
    where
        H: FindPetsByTagsHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/pet/findByTags",
            on_service(MethodFilter::GET, FindPetsByTagsService::new(handler)),
        );

        self
    }

    pub fn get_pet_by_id<H, T>(mut self, handler: H) -> Self
    where
        H: GetPetByIdHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/pet/:petId",
            on_service(MethodFilter::GET, GetPetByIdService::new(handler)),
        );

        self
    }

    pub fn update_pet_with_form<H, T>(mut self, handler: H) -> Self
    where
        H: UpdatePetWithFormHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/pet/:petId",
            on_service(MethodFilter::POST, UpdatePetWithFormService::new(handler)),
        );

        self
    }

    pub fn delete_pet<H, T>(mut self, handler: H) -> Self
    where
        H: DeletePetHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/pet/:petId",
            on_service(MethodFilter::DELETE, DeletePetService::new(handler)),
        );

        self
    }

    pub fn upload_file<H, T>(mut self, handler: H) -> Self
    where
        H: UploadFileHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/pet/:petId/uploadImage",
            on_service(MethodFilter::POST, UploadFileService::new(handler)),
        );

        self
    }
}

impl Default for PetRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PetRouter> for Router {
    fn from(r: PetRouter) -> Self {
        r.router
    }
}

/// PUT /pet handler
pub trait UpdatePetHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(update_pet_handler);

/// PUT /pet service
struct UpdatePetService<H, T, B>
where
    H: UpdatePetHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for UpdatePetService<H, T, B>
where
    H: UpdatePetHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> UpdatePetService<H, T, B>
where
    H: UpdatePetHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for UpdatePetService<H, T, B>
where
    H: UpdatePetHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(UpdatePetHandler::call(handler, req).await) })
    }
}

/// POST /pet handler
pub trait AddPetHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(add_pet_handler);

/// POST /pet service
struct AddPetService<H, T, B>
where
    H: AddPetHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for AddPetService<H, T, B>
where
    H: AddPetHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> AddPetService<H, T, B>
where
    H: AddPetHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for AddPetService<H, T, B>
where
    H: AddPetHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(AddPetHandler::call(handler, req).await) })
    }
}

/// GET /pet/findByStatus handler
pub trait FindPetsByStatusHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(find_pets_by_status_handler);

/// GET /pet/findByStatus service
struct FindPetsByStatusService<H, T, B>
where
    H: FindPetsByStatusHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for FindPetsByStatusService<H, T, B>
where
    H: FindPetsByStatusHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> FindPetsByStatusService<H, T, B>
where
    H: FindPetsByStatusHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for FindPetsByStatusService<H, T, B>
where
    H: FindPetsByStatusHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(FindPetsByStatusHandler::call(handler, req).await) })
    }
}

/// GET /pet/findByTags handler
pub trait FindPetsByTagsHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(find_pets_by_tags_handler);

/// GET /pet/findByTags service
struct FindPetsByTagsService<H, T, B>
where
    H: FindPetsByTagsHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for FindPetsByTagsService<H, T, B>
where
    H: FindPetsByTagsHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> FindPetsByTagsService<H, T, B>
where
    H: FindPetsByTagsHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for FindPetsByTagsService<H, T, B>
where
    H: FindPetsByTagsHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(FindPetsByTagsHandler::call(handler, req).await) })
    }
}

/// GET /pet/{petId} handler
pub trait GetPetByIdHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(get_pet_by_id_handler);

/// GET /pet/{petId} service
struct GetPetByIdService<H, T, B>
where
    H: GetPetByIdHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for GetPetByIdService<H, T, B>
where
    H: GetPetByIdHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> GetPetByIdService<H, T, B>
where
    H: GetPetByIdHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for GetPetByIdService<H, T, B>
where
    H: GetPetByIdHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(GetPetByIdHandler::call(handler, req).await) })
    }
}

/// POST /pet/{petId} handler
pub trait UpdatePetWithFormHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(update_pet_with_form_handler);

/// POST /pet/{petId} service
struct UpdatePetWithFormService<H, T, B>
where
    H: UpdatePetWithFormHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for UpdatePetWithFormService<H, T, B>
where
    H: UpdatePetWithFormHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> UpdatePetWithFormService<H, T, B>
where
    H: UpdatePetWithFormHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for UpdatePetWithFormService<H, T, B>
where
    H: UpdatePetWithFormHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(UpdatePetWithFormHandler::call(handler, req).await) })
    }
}

/// DELETE /pet/{petId} handler
pub trait DeletePetHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(delete_pet_handler);

/// DELETE /pet/{petId} service
struct DeletePetService<H, T, B>
where
    H: DeletePetHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for DeletePetService<H, T, B>
where
    H: DeletePetHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> DeletePetService<H, T, B>
where
    H: DeletePetHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for DeletePetService<H, T, B>
where
    H: DeletePetHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(DeletePetHandler::call(handler, req).await) })
    }
}

/// POST /pet/{petId}/uploadImage handler
pub trait UploadFileHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(upload_file_handler);

/// POST /pet/{petId}/uploadImage service
struct UploadFileService<H, T, B>
where
    H: UploadFileHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for UploadFileService<H, T, B>
where
    H: UploadFileHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> UploadFileService<H, T, B>
where
    H: UploadFileHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for UploadFileService<H, T, B>
where
    H: UploadFileHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(UploadFileHandler::call(handler, req).await) })
    }
}

/// Store router
pub struct StoreRouter {
    pub(crate) router: Router,
}

impl StoreRouter {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn get_inventory<H, T>(mut self, handler: H) -> Self
    where
        H: GetInventoryHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/store/inventory",
            on_service(MethodFilter::GET, GetInventoryService::new(handler)),
        );

        self
    }

    pub fn place_order<H, T>(mut self, handler: H) -> Self
    where
        H: PlaceOrderHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/store/order",
            on_service(MethodFilter::POST, PlaceOrderService::new(handler)),
        );

        self
    }

    pub fn get_order_by_id<H, T>(mut self, handler: H) -> Self
    where
        H: GetOrderByIdHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/store/order/:orderId",
            on_service(MethodFilter::GET, GetOrderByIdService::new(handler)),
        );

        self
    }

    pub fn delete_order<H, T>(mut self, handler: H) -> Self
    where
        H: DeleteOrderHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/store/order/:orderId",
            on_service(MethodFilter::DELETE, DeleteOrderService::new(handler)),
        );

        self
    }
}

impl Default for StoreRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl From<StoreRouter> for Router {
    fn from(r: StoreRouter) -> Self {
        r.router
    }
}

/// GET /store/inventory handler
pub trait GetInventoryHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(get_inventory_handler);

/// GET /store/inventory service
struct GetInventoryService<H, T, B>
where
    H: GetInventoryHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for GetInventoryService<H, T, B>
where
    H: GetInventoryHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> GetInventoryService<H, T, B>
where
    H: GetInventoryHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for GetInventoryService<H, T, B>
where
    H: GetInventoryHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(GetInventoryHandler::call(handler, req).await) })
    }
}

/// POST /store/order handler
pub trait PlaceOrderHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(place_order_handler);

/// POST /store/order service
struct PlaceOrderService<H, T, B>
where
    H: PlaceOrderHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for PlaceOrderService<H, T, B>
where
    H: PlaceOrderHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> PlaceOrderService<H, T, B>
where
    H: PlaceOrderHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for PlaceOrderService<H, T, B>
where
    H: PlaceOrderHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(PlaceOrderHandler::call(handler, req).await) })
    }
}

/// GET /store/order/{orderId} handler
pub trait GetOrderByIdHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(get_order_by_id_handler);

/// GET /store/order/{orderId} service
struct GetOrderByIdService<H, T, B>
where
    H: GetOrderByIdHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for GetOrderByIdService<H, T, B>
where
    H: GetOrderByIdHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> GetOrderByIdService<H, T, B>
where
    H: GetOrderByIdHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for GetOrderByIdService<H, T, B>
where
    H: GetOrderByIdHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(GetOrderByIdHandler::call(handler, req).await) })
    }
}

/// DELETE /store/order/{orderId} handler
pub trait DeleteOrderHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(delete_order_handler);

/// DELETE /store/order/{orderId} service
struct DeleteOrderService<H, T, B>
where
    H: DeleteOrderHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for DeleteOrderService<H, T, B>
where
    H: DeleteOrderHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> DeleteOrderService<H, T, B>
where
    H: DeleteOrderHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for DeleteOrderService<H, T, B>
where
    H: DeleteOrderHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(DeleteOrderHandler::call(handler, req).await) })
    }
}

/// User router
pub struct UserRouter {
    pub(crate) router: Router,
}

impl UserRouter {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn create_user<H, T>(mut self, handler: H) -> Self
    where
        H: CreateUserHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/user",
            on_service(MethodFilter::POST, CreateUserService::new(handler)),
        );

        self
    }

    pub fn create_users_with_list_input<H, T>(mut self, handler: H) -> Self
    where
        H: CreateUsersWithListInputHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/user/createWithList",
            on_service(
                MethodFilter::POST,
                CreateUsersWithListInputService::new(handler),
            ),
        );

        self
    }

    pub fn login_user<H, T>(mut self, handler: H) -> Self
    where
        H: LoginUserHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/user/login",
            on_service(MethodFilter::GET, LoginUserService::new(handler)),
        );

        self
    }

    pub fn logout_user<H, T>(mut self, handler: H) -> Self
    where
        H: LogoutUserHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/user/logout",
            on_service(MethodFilter::GET, LogoutUserService::new(handler)),
        );

        self
    }

    pub fn get_user_by_name<H, T>(mut self, handler: H) -> Self
    where
        H: GetUserByNameHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/user/:username",
            on_service(MethodFilter::GET, GetUserByNameService::new(handler)),
        );

        self
    }

    pub fn update_user<H, T>(mut self, handler: H) -> Self
    where
        H: UpdateUserHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/user/:username",
            on_service(MethodFilter::PUT, UpdateUserService::new(handler)),
        );

        self
    }

    pub fn delete_user<H, T>(mut self, handler: H) -> Self
    where
        H: DeleteUserHandler<T>,
        T: 'static,
    {
        self.router = self.router.route(
            "/user/:username",
            on_service(MethodFilter::DELETE, DeleteUserService::new(handler)),
        );

        self
    }
}

impl Default for UserRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl From<UserRouter> for Router {
    fn from(r: UserRouter) -> Self {
        r.router
    }
}

/// POST /user handler
pub trait CreateUserHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(create_user_handler);

/// POST /user service
struct CreateUserService<H, T, B>
where
    H: CreateUserHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for CreateUserService<H, T, B>
where
    H: CreateUserHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> CreateUserService<H, T, B>
where
    H: CreateUserHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for CreateUserService<H, T, B>
where
    H: CreateUserHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(CreateUserHandler::call(handler, req).await) })
    }
}

/// POST /user/createWithList handler
pub trait CreateUsersWithListInputHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(create_users_with_list_input_handler);

/// POST /user/createWithList service
struct CreateUsersWithListInputService<H, T, B>
where
    H: CreateUsersWithListInputHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for CreateUsersWithListInputService<H, T, B>
where
    H: CreateUsersWithListInputHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> CreateUsersWithListInputService<H, T, B>
where
    H: CreateUsersWithListInputHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for CreateUsersWithListInputService<H, T, B>
where
    H: CreateUsersWithListInputHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(CreateUsersWithListInputHandler::call(handler, req).await) })
    }
}

/// GET /user/login handler
pub trait LoginUserHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(login_user_handler);

/// GET /user/login service
struct LoginUserService<H, T, B>
where
    H: LoginUserHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for LoginUserService<H, T, B>
where
    H: LoginUserHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> LoginUserService<H, T, B>
where
    H: LoginUserHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for LoginUserService<H, T, B>
where
    H: LoginUserHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(LoginUserHandler::call(handler, req).await) })
    }
}

/// GET /user/logout handler
pub trait LogoutUserHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(logout_user_handler);

/// GET /user/logout service
struct LogoutUserService<H, T, B>
where
    H: LogoutUserHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for LogoutUserService<H, T, B>
where
    H: LogoutUserHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> LogoutUserService<H, T, B>
where
    H: LogoutUserHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for LogoutUserService<H, T, B>
where
    H: LogoutUserHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(LogoutUserHandler::call(handler, req).await) })
    }
}

/// GET /user/{username} handler
pub trait GetUserByNameHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(get_user_by_name_handler);

/// GET /user/{username} service
struct GetUserByNameService<H, T, B>
where
    H: GetUserByNameHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for GetUserByNameService<H, T, B>
where
    H: GetUserByNameHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> GetUserByNameService<H, T, B>
where
    H: GetUserByNameHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for GetUserByNameService<H, T, B>
where
    H: GetUserByNameHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(GetUserByNameHandler::call(handler, req).await) })
    }
}

/// PUT /user/{username} handler
pub trait UpdateUserHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(update_user_handler);

/// PUT /user/{username} service
struct UpdateUserService<H, T, B>
where
    H: UpdateUserHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for UpdateUserService<H, T, B>
where
    H: UpdateUserHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> UpdateUserService<H, T, B>
where
    H: UpdateUserHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for UpdateUserService<H, T, B>
where
    H: UpdateUserHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(UpdateUserHandler::call(handler, req).await) })
    }
}

/// DELETE /user/{username} handler
pub trait DeleteUserHandler<T, B = Body>: Clone + Send + Sized + 'static {
    fn call(self, req: Request<B>) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
}

all_the_tuples!(delete_user_handler);

/// DELETE /user/{username} service
struct DeleteUserService<H, T, B>
where
    H: DeleteUserHandler<T, B>,
{
    handler: H,
    _marker: PhantomData<fn() -> (T, B)>,
}

impl<H, T, B> Clone for DeleteUserService<H, T, B>
where
    H: DeleteUserHandler<T, B>,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> DeleteUserService<H, T, B>
where
    H: DeleteUserHandler<T, B>,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: PhantomData,
        }
    }
}

impl<H, T, B> Service<Request<B>> for DeleteUserService<H, T, B>
where
    H: DeleteUserHandler<T, B>,
    B: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move { Ok(DeleteUserHandler::call(handler, req).await) })
    }
}
