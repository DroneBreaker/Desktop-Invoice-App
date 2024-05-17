use serde::{Deserialize, Serialize};
use tauri::http;
use http::method::Method;

#[derive(Deserialize)]
pub struct LoginRequest {
    business_tin: String,
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    success: bool,
    token: Option<String>,
    message: String,
}

// #[tauri::command]
// remember to call `.manage(MyState::default())`
// pub async fn handle_login(
//     business_tin: String,
//     username: String,
//     password: String,
// ) -> Result<LoginResponse, Box<dyn std::error::Error>> {
//     let request_payload = LoginRequest {
//         business_tin,
//         username,
//         password,
//     };

//     let url = "http://localhost:8080/login";

//     // let response = (Method::POST, url)
//     //     .header("Content-Type")
//     //     .body(serde_json::to_string(&request_payload.try_into())?)
//     //     .send()
//     //     .await;

//     let response_body: Result<String, _> = response.text().await.into_string();
//     // let login_response = serde_json::from_str(&response_body.try_into())?;

//     // Ok(login_response)

//     // if request.business_tin == "business_tin" && request.username == "drone" && request.password == "admin" {
//     //     LoginResponse {
//     //       success: true,
//     //       token: Some("dummy_token".to_string()),
//     //       message:"Login successful".to_string()
//     //     }
//     // } else {
//     //     LoginResponse {
//     //       success: false,
//     //       token: None,
//     //       message:"Login failed".to_string()
//     //     }
//     // }
// }
