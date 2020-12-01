//  WebAuth



pub struct WebAuthOptions {
  // plugins
  _send_telemetry:bool,
  _times_to_retry_failed_requests:isize,
  tenant:String,
  token_issuer:String,
  // this is an unkown field at the moment
  jwks_uri:String
}

pub struct WebAuth {
  transaction_manager:_,
  client:_,
  popup:_,
  cross_origin_authentication:_,
  web_message_handler:_
  sso_data_storage:_
  base_options:WebAuthOptions
}

pub enum ResponseMode {}

pub struct WebAuthConstructorOptions {
  /// How the Auth response is encoded and redirected back to the client.
  /// Supported values are query, fragment and form_post.
  /// The query value is only supported when responseType is code.
  /// https://openid.net/specs/oauth-v2-multiple-response-types-1_0.html#ResponseModes
  response_mode: Option<ResponseMode>,
  /// where to redirect on authentication success
  redirect_uri: Option<String>,
  /// a list of scopes the application is requesting
  scope: Option<String>,
  /// identifier of the resource server who will consume the access token issued after Auth
  audience: Option<String>,
  /// Unsure what this is.
  popup_origin: Option<String>,
  // TODO: Doc leeyway
  leeway: Option<String>,
  // plugins Option<Vec>,
  ///* maximum elapsed time in seconds since the last time the user was actively authenticated by the authorization server
  max_age: isize,
}



pub struct ParseHashResponse {
  access_token: Option<String>,
  id_token: Option<String>,
  id_token_payload: Option<String>,
  app_state: Option<String>,
  refresh_token: Option<String>,
  state:Option<String>,
  expires_in:Option<isize>,
  token_type: Option<String>,
  scope:Option<String>
}
impl Default for ParseHashResponse {
  fn default() -> Self {
    Self {access_token:None,id_token:None,id_token_payload:None,app_state:None,refresh_token:None,state:None,expires_in:Some(10),token_type:None,scope:None}
  }
}



impl WebAuth {
  pub fn new(
    // the auth0 tenant domain
    domain: String,
    // the auth0 client id
    client_id: String,
    // all the optional arguments
    opts: Option<WebAuthConstructorOptions>,
  ) {
  }
  pub fn parse_hash(){
    todo!();
  }
  pub fn validate_authentication_response(&self){
    todo!()
  }
  fn build_parse_hash_response(qs_params:String,app_state:String,token:String)->ParseHashResponse{
    todo!();
    ParseHashResponse::default()
  }
  pub fn renew_auth(){
    todo!()
  }
  pub fn check_session(){
    todo!()
  }
  pub fn change_password(){
    todo!()
  }
  pub fn passwordless_start(){
    todo!()
  }
  pub fn signup(){
    todo!()
  }
  pub fn authorize(){
    todo!()
  }
  pub fn signup_and_authorize(){
    todo!()
  }
  pub fn logout(){
    todo!()
  }
  pub fn login(){
    todo!()
  }
  pub fn passwordless_login(){
    todo!()
  }

  pub fn crossOriginAuthenticationCallback(){
    unimplemented!() // potentially unimplementable
  }
  pub fn crossOriginVerification(){
    unimplemented!() // potentially unimplementable
  }
  pub fn passwordless_verify(){
    todo!()
  }
  pub fn render_captcha(){
    unimplemented!()
  }
}
