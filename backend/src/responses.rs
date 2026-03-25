use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder, Response};
use rocket::Request;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

/// Wrapper for successful API responses with metadata
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ResponseMeta>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
            meta: None,
        }
    }

    pub fn with_meta(data: T, meta: ResponseMeta) -> Self {
        Self {
            success: true,
            data,
            meta: Some(meta),
        }
    }
}

/// Metadata for list responses
#[derive(Debug, Serialize)]
pub struct ResponseMeta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}

impl ResponseMeta {
    pub fn new(total: i64, limit: i64, offset: i64) -> Self {
        let has_more = offset + limit < total;
        Self {
            total,
            limit,
            offset,
            has_more,
        }
    }
}

/// Error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}

/// Custom error type for API operations
#[derive(Debug)]
pub struct ApiError {
    pub status: Status,
    pub code: String,
    pub message: String,
}

impl ApiError {
    pub fn not_found(message: String) -> Self {
        Self {
            status: Status::NotFound,
            code: "NOT_FOUND".to_string(),
            message,
        }
    }

    pub fn bad_request(message: String) -> Self {
        Self {
            status: Status::BadRequest,
            code: "BAD_REQUEST".to_string(),
            message,
        }
    }

    pub fn internal_error(message: String) -> Self {
        Self {
            status: Status::InternalServerError,
            code: "INTERNAL_ERROR".to_string(),
            message,
        }
    }

    pub fn validation_error(message: String) -> Self {
        Self {
            status: Status::UnprocessableEntity,
            code: "VALIDATION_ERROR".to_string(),
            message,
        }
    }

    pub fn conflict(message: String) -> Self {
        Self {
            status: Status::Conflict,
            code: "CONFLICT".to_string(),
            message,
        }
    }
}

/// Implement Responder for ApiError
impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let error_response = ErrorResponse {
            success: false,
            error: ErrorDetail {
                code: self.code,
                message: self.message,
            },
        };

        let json = serde_json::to_string(&error_response).unwrap();
        Response::build()
            .status(self.status)
            .header(ContentType::JSON)
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
}

/// Pagination query parameters
#[derive(Debug, Deserialize, rocket::form::FromForm)]
pub struct PaginationParams {
    #[field(default = 50)]
    pub limit: i64,
    #[field(default = 0)]
    pub offset: i64,
}

impl PaginationParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.limit < 1 || self.limit > 100 {
            return Err(ApiError::bad_request(
                "Limit must be between 1 and 100".to_string(),
            ));
        }
        if self.offset < 0 {
            return Err(ApiError::bad_request(
                "Offset must be non-negative".to_string(),
            ));
        }
        Ok(())
    }
}

/// Search query parameters for snipes
#[derive(Debug, Deserialize, rocket::form::FromForm)]
pub struct SnipeSearchParams {
    #[field(default = 50)]
    pub limit: i64,
    #[field(default = 0)]
    pub offset: i64,
    pub sniper_id: Option<i64>,
    pub snipee_id: Option<i64>,
    pub channel_id: Option<i64>,
    pub guild_id: Option<i64>,
    pub has_text: Option<bool>,
}

impl SnipeSearchParams {
    pub fn pagination(&self) -> PaginationParams {
        PaginationParams {
            limit: self.limit,
            offset: self.offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_success() {
        let response = ApiResponse::success("test data");
        assert!(response.success);
        assert_eq!(response.data, "test data");
        assert!(response.meta.is_none());
    }

    #[test]
    fn test_api_response_with_meta() {
        let meta = ResponseMeta::new(100, 50, 0);
        let response = ApiResponse::with_meta(vec![1, 2, 3], meta);
        assert!(response.success);
        assert_eq!(response.data, vec![1, 2, 3]);
        assert!(response.meta.is_some());
    }

    #[test]
    fn test_response_meta_has_more() {
        let meta = ResponseMeta::new(100, 50, 0);
        assert_eq!(meta.total, 100);
        assert_eq!(meta.limit, 50);
        assert_eq!(meta.offset, 0);
        assert!(meta.has_more);

        let meta = ResponseMeta::new(30, 50, 0);
        assert!(!meta.has_more);

        let meta = ResponseMeta::new(100, 50, 50);
        assert!(!meta.has_more);
    }

    #[test]
    fn test_api_error_not_found() {
        let error = ApiError::not_found("Resource not found".to_string());
        assert_eq!(error.status, rocket::http::Status::NotFound);
        assert_eq!(error.code, "NOT_FOUND");
        assert_eq!(error.message, "Resource not found");
    }

    #[test]
    fn test_api_error_bad_request() {
        let error = ApiError::bad_request("Invalid input".to_string());
        assert_eq!(error.status, rocket::http::Status::BadRequest);
        assert_eq!(error.code, "BAD_REQUEST");
    }

    #[test]
    fn test_api_error_validation() {
        let error = ApiError::validation_error("Validation failed".to_string());
        assert_eq!(error.status, rocket::http::Status::UnprocessableEntity);
        assert_eq!(error.code, "VALIDATION_ERROR");
    }

    #[test]
    fn test_api_error_conflict() {
        let error = ApiError::conflict("Already exists".to_string());
        assert_eq!(error.status, rocket::http::Status::Conflict);
        assert_eq!(error.code, "CONFLICT");
    }

    #[test]
    fn test_api_error_internal() {
        let error = ApiError::internal_error("Server error".to_string());
        assert_eq!(error.status, rocket::http::Status::InternalServerError);
        assert_eq!(error.code, "INTERNAL_ERROR");
    }

    #[test]
    fn test_pagination_params_validation() {
        let params = PaginationParams {
            limit: 50,
            offset: 0,
        };
        assert!(params.validate().is_ok());

        let params = PaginationParams {
            limit: 100,
            offset: 0,
        };
        assert!(params.validate().is_ok());

        let params = PaginationParams {
            limit: 101,
            offset: 0,
        };
        assert!(params.validate().is_err());

        let params = PaginationParams {
            limit: 0,
            offset: 0,
        };
        assert!(params.validate().is_err());

        let params = PaginationParams {
            limit: 50,
            offset: -1,
        };
        assert!(params.validate().is_err());
    }

    #[test]
    fn test_snipe_search_params_pagination() {
        let search_params = SnipeSearchParams {
            limit: 20,
            offset: 10,
            sniper_id: Some(123),
            snipee_id: None,
            channel_id: None,
            guild_id: None,
            has_text: None,
        };

        let pagination = search_params.pagination();
        assert_eq!(pagination.limit, 20);
        assert_eq!(pagination.offset, 10);
    }
}
