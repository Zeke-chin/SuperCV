use reqwest::Client;
use serde::Deserialize;

use crate::client::common::{ClientError, ClientUserTrait};
use crate::client::models::device::{CreateDevice, DeviceResp, UpdateDevice};
use crate::client::models::user::{UserLogin, UserRegister, UserResetPassword, UserResp};

#[derive(Deserialize)]
struct ApiResponse<T> {
	code: i32,
	data: Option<T>,
	error_msg: Option<String>,
}

pub struct HttpClient {
	client: Client,
	base_url: String,
}

impl HttpClient {
	pub fn new(base_url: String) -> Self {
		HttpClient {
			client: Client::new(),
			base_url,
		}
	}

	async fn handle_response<T: for<'de> Deserialize<'de>>(
		&self,
		response: reqwest::Response,
	) -> Result<T, ClientError> {
		let status = response.status();
		let body = response
			.text()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;

		if status.is_success() {
			let api_response: ApiResponse<T> =
				serde_json::from_str(&body).map_err(|e| ClientError::SerializationError(e.to_string()))?;

			if api_response.code == 200 {
				api_response.data.ok_or_else(|| ClientError::ApiError {
					code: api_response.code,
					message: "No data in response".to_string(),
				})
			} else {
				Err(ClientError::ApiError {
					code: api_response.code,
					message: api_response.error_msg.unwrap_or_else(|| "Unknown error".to_string()),
				})
			}
		} else {
			Err(ClientError::ApiError {
				code: status.as_u16() as i32,
				message: body,
			})
		}
	}
}

#[async_trait::async_trait]
impl ClientUserTrait for HttpClient {
	async fn register_user(&self, create_user: UserRegister) -> Result<UserResp, ClientError> {
		let url = format!("{}/user/register", self.base_url);
		let response = self
			.client
			.post(&url)
			.json(&create_user)
			.send()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}

	async fn login_user(&self, entity: UserLogin) -> Result<UserResp, ClientError> {
		let url = format!("{}/user/login", self.base_url);
		let response = self
			.client
			.post(&url)
			.json(&entity)
			.send()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}

	async fn reset_user(&self, entity: UserResetPassword) -> Result<UserResp, ClientError> {
		let url = format!("{}/user/reset", self.base_url);
		let response = self
			.client
			.post(&url)
			.json(&entity)
			.send()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}

	async fn create_device(&self, create_device: CreateDevice) -> Result<DeviceResp, ClientError> {
		todo!()
	}

	async fn update_device(&self, update_device: UpdateDevice) -> Result<DeviceResp, ClientError> {
		todo!()
	}

	async fn get_devices_by_user_id(&self, user_id: i32) -> Result<Vec<DeviceResp>, ClientError> {
		todo!()
	}

	async fn delete_device(&self, device_id: i32) -> Result<bool, ClientError> {
		todo!()
	}
}

