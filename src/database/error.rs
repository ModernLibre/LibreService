use crate::error::ServiceError;
use actix_web::error::BlockingError;

impl From<r2d2::Error> for ServiceError {
    fn from(_: r2d2::Error) -> ServiceError {
        ServiceError::InternalServerError
    }
}

impl From<BlockingError> for ServiceError {
    fn from(_: BlockingError) -> ServiceError {
        ServiceError::InternalServerError
    }
}

impl From<diesel::result::Error> for ServiceError {
    fn from(error: diesel::result::Error) -> ServiceError {
        use diesel::result::{DatabaseErrorKind, Error};
        match error {
            Error::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_owned();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}
