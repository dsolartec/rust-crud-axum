use std::sync::Arc;

use axum::{Router, Server, routing::{post, get, IntoMakeService, put}};
use hyper::server::conn::AddrIncoming;
use tokio::sync::RwLock;

use crate::{internal::{platform::{encryption::JwtEncryption, logger::Logger}, apperror::AppError}, modules::{users::{UsersService, UsersController}, permissions::{PermissionsService, PermissionsController}, auth::AuthController}};

use super::{handlers, state::ControllersState};

type ApplicationServer = Server<AddrIncoming, IntoMakeService<Router>>;

pub struct App {
    logger: Logger,

    // Encryption
    jwt_encryption: JwtEncryption,

    // Services
    users_service: Arc<RwLock<UsersService>>,
    permissions_service: Arc<RwLock<PermissionsService>>,
}

impl App {
    fn setup_auth_routes(&self) -> Router<ControllersState> {
        let otp = Router::new()
            .route("/qrCode", get(handlers::auth_handlers::get_otp_qr_code))
            .route("/enable/:otp_code", put(handlers::auth_handlers::enable_otp))
            .route("/verify/:otp_code", post(handlers::auth_handlers::verify_otp_code));

        Router::new()
            .route("/logIn", post(handlers::auth_handlers::log_in))
            .route("/signUp", post(handlers::auth_handlers::sign_up))
            .route("/me", get(handlers::auth_handlers::me))
            .nest("/otp", otp)
    }

    fn setup_permissions_routes(&self) -> Router<ControllersState> {
        Router::new()
            .route(
                "/",
                get(handlers::permissions_handlers::get_permissions)
                .post(handlers::permissions_handlers::create_permission),
            )
            .route("/id/:id", get(handlers::permissions_handlers::get_permission_by_id))
            .route(
                "/name/:name",
                get(handlers::permissions_handlers::get_permission_by_name)
                .delete(handlers::permissions_handlers::delete_permission),
            )
    }

    fn setup_users_routes(&self) -> Router<ControllersState> {
        let user_by_username = Router::new()
            .route(
                "/",
                get(handlers::users_handlers::get_user_by_username)
                .delete(handlers::users_handlers::delete_user),
            )
            .route("/permissions", get(handlers::users_handlers::get_permissions_for_user))
            .route(
                "/permission/:permission_name",
                post(handlers::users_handlers::grant_permission)
                .delete(handlers::users_handlers::revoke_permission),
            );

        Router::new()
            .route("/", get(handlers::users_handlers::get_users))
            .route("/id/:id", get(handlers::users_handlers::get_user_by_id))
            .nest("/username/:username", user_by_username)
    }

    fn setup_web_routes(&self) -> Router<ControllersState> {
        Router::new()
            .route("/", get(handlers::web_handlers::login_page))
            .route("/signup", get(handlers::web_handlers::signup_page))
            .route("/home", get(handlers::web_handlers::home_page))
            .route("/otp", get(handlers::web_handlers::otp_page))
    }

    fn setup_routes(&self) -> Router {
        let auth_controller = AuthController::new(&self.logger, &self.jwt_encryption, &self.users_service, &self.permissions_service);
        let users_controller = UsersController::new(&self.users_service);
        let permissions_controller = PermissionsController::new(&self.users_service, &self.permissions_service);

        let state = ControllersState::new(auth_controller, users_controller, permissions_controller);

        Router::new()
            .nest("/auth", self.setup_auth_routes())
            .nest("/permissions", self.setup_permissions_routes())
            .nest("/users", self.setup_users_routes())
            .nest("/", self.setup_web_routes())
            .with_state(state)
    }

    pub fn run(&self) -> ApplicationServer {
        let server = axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
            .serve(self.setup_routes().into_make_service());

        self.logger.info("[APP] Application listening on :8080");

        server
    }

    pub async fn new(
        logger: Option<Logger>,

        // Encryption
        jwt_encryption: Option<JwtEncryption>,

        // Services
        users_service: Option<UsersService>,
        permissions_service: Option<PermissionsService>,
    ) -> Result<App, AppError> {
        let logger = logger.unwrap_or_else(|| Logger::new());

        // Encryption
        let jwt_encryption = jwt_encryption.unwrap_or_else(|| JwtEncryption::new());

        // Services
        let users_service = users_service.unwrap_or_else(|| UsersService::new(&logger));
        let permissions_service = permissions_service.unwrap_or_else(|| PermissionsService::new(&logger));
        
        Ok(App {
            logger,

            // Encryption
            jwt_encryption,

            // Services
            users_service: Arc::new(RwLock::new(users_service)),
            permissions_service: Arc::new(RwLock::new(permissions_service)),
        })
    }
}
