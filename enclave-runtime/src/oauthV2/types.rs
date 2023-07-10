extern crate sgx_tstd as std;
use std::string::{String, ToString};
use std::str::FromStr;
use std::collections::HashMap;
use serde_json::Value;


/************************************\
 *              Enums               *
\************************************/

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl FromStr for HttpMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            "put" => Ok(HttpMethod::Put),
            "delete" => Ok(HttpMethod::Delete),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum HttpResponseType {
    Success,
    Redirection,
    ClientError,
    ServerError,
}

impl From<u64> for HttpResponseType {
    fn from(status_code: u64) -> Self {
        match status_code {
            200..=299 => HttpResponseType::Success,
            300..=399 => HttpResponseType::Redirection,
            400..=499 => HttpResponseType::ClientError,
            500..=599 => HttpResponseType::ServerError,
            _ => panic!("Invalid status code: {}", status_code),
        }
    }
}

/// The type of the grant that was provided by the user see:
/// https://datatracker.ietf.org/doc/html/rfc6749#section-4
#[derive(Debug)]
pub enum GrantType {
    /// The client gets an authorization code see:
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.1
    AuthorizationCode,
    /// The implicit grant type is used to obtain access tokens (it does not
    /// support the issuance of refresh tokens) and is optimized for public
    /// clients known to operate a particular redirection URI see:
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.2
    Implicit,
    /// The Resource owner password credentials grant is the one implemented
    /// in this example see:
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.3
    ResourceOwnerPasswordCredentials,
    /// Client is the one identifying see:
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.4
    ClientCredentials,
    /// The client uses an extension grant type by specifying the grant type
    /// using an absolute URI (defined by the authorization server) as the
    /// value of the "grant_type" parameter of the token endpoint, and by
    /// adding any additional parameters necessary see:
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.5
    Extension,
}

/// The type of the token that was provided by the authorization server see:
/// https://datatracker.ietf.org/doc/html/rfc6749#section-4
#[derive(Debug)]
pub enum TokenType {
    /// The bearer tokens use HTTPS security, and the request is not signed or encrypted.
    /// Possession of the bearer token is considered authentication see:
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-7.1
    /// https://www.soapui.org/docs/oauth2/oauth2-overview/
    Bearer,
    /// More secure than bearer tokens, MAC tokens are similar to signatures, 
    /// in that they provide a way to have (partial) cryptographic verification of the request.
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-7.1
    /// https://www.soapui.org/docs/oauth2/oauth2-overview/
    Mac,
}

impl FromStr for TokenType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bearer" => Ok(TokenType::Bearer),
            "mac" => Ok(TokenType::Mac),
            _ => Err(()),
        }
    }
}

/// Should the request be invalid specifies the type of failure see:
/// https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
#[derive(Debug)]
pub enum ErrorCode {
    /// The request is missing a required parameter
    InvalidRequest,
    /// Client authentication failed
    InvalidClient,
    /// The provided authorization grant is invalid
    InvalidGrant,
    /// The authenticated client is not authorized to use this
    /// authorization grant type
    UnauthorizedClient,
    /// The authorization grant type is not supported
    UnsupportedGrantType,
    /// The requested scope is invalid
    InvalidScope,
}

impl FromStr for ErrorCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "invalid_request" => Ok(ErrorCode::InvalidRequest),
            "invalid_client" => Ok(ErrorCode::InvalidClient),
            "invalid_grant" => Ok(ErrorCode::InvalidGrant),
            "unauthorized_client" => Ok(ErrorCode::UnauthorizedClient),
            "unsupported_grant_type" => Ok(ErrorCode::UnsupportedGrantType),
            "invalid_scope" => Ok(ErrorCode::InvalidScope),
            _ => Err(()),
        }
    }
}


/************************************\
 *             Request              *
\************************************/

/// A Request's request line
#[derive(Debug)]
pub struct RequestLine {
    pub method: HttpMethod,
    pub path: String,
    pub http_version: String,
}

/// Simple HTTP Request implementation
#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub headers: HashMap<String,String>,
    pub body: serde_json::Value,
}

/// An access token request and its necessary fields
#[derive(Debug)]
pub struct AccessTokenRequest {
    /// The HTTP request
    pub request: Request,
    pub grant_type: GrantType,
    /// The client credentials stored on the authorization server to identify a client and validate its secret see:
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-10.1
    pub client_id: String,
    pub client_secret: String,
    /// The user credentials to gain access to the resource server see:
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.3
    pub username: String,
    pub password: String
}


/************************************\
 *             Response             *
\************************************/
/// A Response's response line
#[derive(Debug)]
pub struct ResponseLine {
    pub http_version: String,
    pub status_code: u64,
    pub response_type: HttpResponseType,
}

/// Simple HTTP Request implementation
#[derive(Debug)]
pub struct Response {
    pub response_line: ResponseLine,
    pub headers: HashMap<String,String>,
    pub body: serde_json::Value,
}

/// An access token response and its necessary fields see:
/// https://datatracker.ietf.org/doc/html/rfc6749#section-5.1
#[derive(Debug)]
pub struct AccessTokenResponse {
    pub response: Response,
    /// The token
    pub access_token: String,
    pub token_type: TokenType,
    /// Time until the token expires (e.g. 3600 -> token expires in one hour)
    pub expires_in: u64,
}


/************************************\
 *              Error               *
\************************************/

/// An Error Response should the access token request fail see:
/// https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
#[derive(Debug)]
pub struct ErrorResponse {
    pub response: Response,
    pub error: ErrorCode,
    pub error_description: String,
    /// A URI identifying a human-readable web page with
    /// information about the error
    pub error_uri: String,
}




