extern crate sgx_tstd as std;
use std::string::{String, ToString};
use std::str::FromStr;
use std::collections::HashMap;


/************************************\
 *              Enums               *
\************************************/

#[derive(Debug, Clone)]
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

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            HttpMethod::Get => "GET".to_string(),
            HttpMethod::Post => "POST".to_string(),
            HttpMethod::Put => "PUT".to_string(),
            HttpMethod::Delete => "DELETE".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
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

impl ToString for HttpResponseType {
    fn to_string(&self) -> String {
        match self {
            HttpResponseType::Success => "Success".to_string(),
            HttpResponseType::Redirection => "Redirection".to_string(),
            HttpResponseType::ClientError => "Client Error".to_string(),
            HttpResponseType::ServerError => "Server Error".to_string(),
        }
    }
}

/// The type of the grant that was provided by the user see:
/// https://datatracker.ietf.org/doc/html/rfc6749#section-4
#[derive(Debug, Clone)]
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

impl ToString for GrantType {
    fn to_string(&self) -> String {
        match self {
            GrantType::AuthorizationCode => "authorization_code".to_string(),
            GrantType::Implicit => "implicit".to_string(),
            GrantType::ResourceOwnerPasswordCredentials => "password".to_string(),
            GrantType::ClientCredentials => "client_credentials".to_string(),
            GrantType::Extension => "extension".to_string(),
        }
    }
}

impl FromStr for GrantType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "authorization_code" => Ok(GrantType::AuthorizationCode),
            "implicit" => Ok(GrantType::Implicit),
            "password" => Ok(GrantType::ResourceOwnerPasswordCredentials),
            "client_credentials" => Ok(GrantType::ClientCredentials),
            "extension" => Ok(GrantType::Extension),
            _ => Err(()),
        }
    }
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

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::Bearer => "Bearer".to_string(),
            TokenType::Mac => "MAC".to_string(),
        }
    }
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

impl ToString for ErrorCode {
    fn to_string(&self) -> String {
        match self {
            ErrorCode::InvalidRequest => "invalid_request".to_string(),
            ErrorCode::InvalidClient => "invalid_client".to_string(),
            ErrorCode::InvalidGrant => "invalid_grant".to_string(),
            ErrorCode::UnauthorizedClient => "unauthorized_client".to_string(),
            ErrorCode::UnsupportedGrantType => "unsupported_grant_type".to_string(),
            ErrorCode::InvalidScope => "invalid_scope".to_string(),
        }
    }
}


/************************************\
 *             Request              *
\************************************/

/// A Request's request line
#[derive(Debug, Clone)]
pub struct RequestLine {
    pub method: HttpMethod,
    pub path: String,
    pub http_version: String,
}

/// Simple HTTP Request implementation
#[derive(Debug, Clone)]
pub struct Request {
    pub request_line: RequestLine,
    pub headers: HashMap<String,String>,
    pub body: serde_json::Value,
}

impl ToString for Request {
    fn to_string(&self) -> String {
        let mut request_str = format!(
            "{} {} {}\r\n",
            self.request_line.method.to_string(),
            self.request_line.path,
            self.request_line.http_version
        );

        for (header, value) in &self.headers {
            request_str.push_str(&format!("{}: {}\r\n", header, value));
        }

        request_str.push_str("\r\n");
        request_str.push_str(&self.body.to_string());

        request_str
    }
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

impl ToString for AccessTokenRequest {
    fn to_string(&self) -> String {
        let request_body = serde_json::json!({
            "grant_type": self.grant_type.to_string(),
            "client_id": self.client_id,
            "client_secret": self.client_secret,
            "username": self.username,
            "password": self.password
        });

        let mut request = self.request.clone();
        request.body = request_body;
        request.to_string()
    }
}


/************************************\
 *             Response             *
\************************************/
/// A Response's response line
#[derive(Debug, Clone)]
pub struct ResponseLine {
    pub http_version: String,
    pub status_code: u64,
    pub response_type: HttpResponseType,
}

/// Simple HTTP Request implementation
#[derive(Debug, Clone)]
pub struct Response {
    pub response_line: ResponseLine,
    pub headers: HashMap<String,String>,
    pub body: serde_json::Value,
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut response_str = format!(
            "{} {} {}\r\n",
            self.response_line.http_version,
            self.response_line.status_code,
            self.response_line.response_type.to_string()
        );

        for (header, value) in &self.headers {
            response_str.push_str(&format!("{}: {}\r\n", header, value));
        }

        response_str.push_str("\r\n");

        if let Some(html_content) = self.body.get("html_content") {
            if let Some(html_string) = html_content.as_str() {
                response_str.push_str(html_string);
            } else {
                response_str.push_str(&self.body.to_string());
            }
        } else {
            response_str.push_str(&self.body.to_string());
        }

        response_str
    }
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

impl ToString for AccessTokenResponse {
    fn to_string(&self) -> String {
        let response_body = serde_json::json!({
            "access_token": self.access_token,
            "token_type": self.token_type.to_string(),
            "expires_in": self.expires_in.to_string(),
        });
        
        let mut response = self.response.clone();
        response.body = response_body;
        response.to_string()
    }
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

impl ToString for ErrorResponse {
    fn to_string(&self) -> String {
        let response_body = serde_json::json!({
            "error": self.error.to_string(),
            "error_description": self.error_description,
            "error_uri": self.error_uri,
        });

        let mut response = self.response.clone();
        response.body = response_body;
        response.to_string()
    }
}
