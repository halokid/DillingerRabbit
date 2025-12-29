use axum::http::StatusCode;
use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;

#[derive(Serialize)]
struct Response {
  success: bool,
  message: String,
}

#[derive(Deserialize)]
struct Request {
  path: Option<String>,
}

async fn run_command(cmd: &str, path: Option<&str>) -> Result<String, String> {
  let command = format!(
    "cd {} && {}",
    path.unwrap_or("./"), // 默认路径，应当修改为实际项目路径
    cmd
  );

  let output = Command::new("sh")
    .arg("-c")
    .arg(&command)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .output()
    .await
    .map_err(|e| format!("Failed to execute command: {}", e))?;

  let stderr = String::from_utf8_lossy(&output.stderr).to_string();

  if stderr.is_empty() {
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
  } else {
    Err(stderr)
  }
}

#[cfg(target_os = "windows")]
async fn run_command_windows(cmd: &str, path: Option<&str>) -> Result<String, String> {
  let command = format!(
    "cd {} && {}",
    path.unwrap_or("."), // Windows默认路径
    cmd
  );

  let output = Command::new("cmd")
    .args(&["/C", &command])
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .output()
    .await
    .map_err(|e| format!("Failed to execute command: {}", e))?;

  let stderr = String::from_utf8_lossy(&output.stderr).to_string();

  if stderr.is_empty() {
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
  } else {
    Err(stderr)
  }
}

async fn presync(Json(req): Json<Request>) -> (StatusCode, Json<Response>) {
  println!("-->>> impl presync");
  #[cfg(target_os = "windows")]
  let result = run_command_windows("./presync.bat", req.path.as_deref()).await;
  #[cfg(not(target_os = "windows"))]
  let result = run_command("./presync.sh", req.path.as_deref()).await;

  match result {
    Ok(output) => (
      StatusCode::OK,
      Json(Response {
        success: true,
        message: format!("{}", "-->>> Presync successful"),
      }),
    ),
    Err(e) => {
      println!("-->>> impl presync error");
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(Response {
          success: false,
          message: format!("{}", "-->>> Presync failed"),
        }),
      )
    }
  }
}

async fn build(Json(req): Json<Request>) -> (StatusCode, Json<Response>) {
  #[cfg(target_os = "windows")]
  let result = run_command_windows("./build.bat", req.path.as_deref()).await;
  #[cfg(not(target_os = "windows"))]
  let result = run_command("./build.sh", req.path.as_deref()).await;

  match result {
    Ok(output) => (
      StatusCode::OK,
      Json(Response {
        success: true,
        message: format!("{}", "-->>> Build successful"),
      }),
    ),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(Response {
        success: false,
        message: format!("{}", "-->>> Build failed"),
      }),
    ),
  }
}

async fn unitest(Json(req): Json<Request>) -> (StatusCode, Json<Response>) {
  #[cfg(target_os = "windows")]
  let result = run_command_windows("./unitest.bat", req.path.as_deref()).await;
  #[cfg(not(target_os = "windows"))]
  let result = run_command("./unitest.sh", req.path.as_deref()).await;

  match result {
    Ok(output) => (
      StatusCode::OK,
      Json(Response {
        success: true,
        message: format!("{}", "-->>> Unitest successful"),
      }),
    ),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(Response {
        success: false,
        message: format!("{}", "-->>> Unitest failed"),
      }),
    ),
  }
}

async fn deploy(Json(req): Json<Request>) -> (StatusCode, Json<Response>) {
  #[cfg(target_os = "windows")]
  let result = run_command_windows("./deploy.bat", req.path.as_deref()).await;
  #[cfg(not(target_os = "windows"))]
  let result = run_command("./deploy.sh", req.path.as_deref()).await;

  match result {
    Ok(output) => (
      StatusCode::OK,
      Json(Response {
        success: true,
        message: format!("{}", "-->>> Deploy successful"),
      }),
    ),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(Response {
        success: false,
        message: format!("{}", "-->>> Deploy failed"),
      }),
    ),
  }
}

async fn verify(Json(req): Json<Request>) -> (StatusCode, Json<Response>) {
  #[cfg(target_os = "windows")]
  let result = run_command_windows("./verify.bat", req.path.as_deref()).await;
  #[cfg(not(target_os = "windows"))]
  let result = run_command("./verify.sh", req.path.as_deref()).await;

  match result {
    Ok(output) => (
      StatusCode::OK,
      Json(Response {
        success: true,
        message: format!("Verify successful: {}", output),
      }),
    ),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(Response {
        success: false,
        message: format!("Verify failed: {}", e),
      }),
    ),
  }
}

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/presync", post(presync))
    .route("/build", post(build))
    .route("/unitest", post(unitest))
    .route("/deploy", post(deploy))
    .route("/verify", post(verify));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  println!("Server listening on http://0.0.0.0:3000");
  axum::serve(listener, app).await.unwrap();
}
