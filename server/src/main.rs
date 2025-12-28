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
    // path.unwrap_or("/path/to/shell_foldr"), // 默认路径，应当修改为实际项目路径
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

  println!("stdout -->>> {}", String::from_utf8_lossy(&output.stdout).to_string());
  println!("stderr -->>> {}", String::from_utf8_lossy(&output.stderr).to_string());
  let stderr =  String::from_utf8_lossy(&output.stderr).to_string();

  // TODO: if need check stderr as condition is `Ok` or `Err`, can check the `stderr` data, if has data, then fail
  if stderr != "" {
    Err(String::from_utf8_lossy(&output.stderr).to_string())
  } else {
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
  }


  // if output.status.success() {
  //   Ok(String::from_utf8_lossy(&output.stdout).to_string())
  // } else {
  //   Err(String::from_utf8_lossy(&output.stderr).to_string())
  // }
}

async fn presync(Json(req): Json<Request>) -> (StatusCode, Json<Response>) {
  println!("-->>> impl presync");
  match run_command("./presync.sh", req.path.as_deref()).await {
    Ok(output) => {
      // println!("-->>> impl presync done!");
      (
        StatusCode::OK,
        Json(Response {
          success: true,
          // message: format!("Presync successful: {}", output),
          message: format!("{}", "-->>> Presync successful"),
        }),
      )
    }
    Err(e) => {
      println!("-->>> impl presync error");
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(Response {
          success: false,
          // message: format!("Presync failed: {}", e),
          message: format!("{}", "-->>> Presync failed"),
        }),
      )
    }
  }
}

async fn build(Json(req): Json<Request>) -> (StatusCode, Json<Response>) {
  match run_command("./build.sh", req.path.as_deref()).await {
    // 假设有build脚本
    Ok(output) => (
      StatusCode::OK,
      Json(Response {
        success: true,
        // message: format!("Build successful: {}", output),
        message: format!("{}", "-->>> Build successful"),
      }),
    ),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(Response {
        success: false,
        // message: format!("Build failed: {}", e),
        message: format!("{}", "-->>> Build failed"),
      }),
    ),
  }
}

async fn unitest(Json(req): Json<Request>) -> (StatusCode, Json<Response>) {
  match run_command("./unitest.sh", req.path.as_deref()).await {
    // 假设是Rust项目
    Ok(output) => (
      StatusCode::OK,
      Json(Response {
        success: true,
        // message: format!("Unitest successful: {}", output),
        message: format!("{}", "-->>> Unitest successful"),
      }),
    ),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(Response {
        success: false,
        // message: format!("Unitest failed: {}", e),
        message: format!("{}", "-->>> Unitest failed"),
      }),
    ),
  }
}

async fn deploy(Json(req): Json<Request>) -> (StatusCode, Json<Response>) {
  match run_command("./deploy.sh", req.path.as_deref()).await {
    // 假设有deploy脚本
    Ok(output) => (
      StatusCode::OK,
      Json(Response {
        success: true,
        // message: format!("Deploy successful: {}", output),
        message: format!("{}", "-->>> Deploy successful"),
      }),
    ),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(Response {
        success: false,
        // message: format!("Deploy failed: {}", e),
        message: format!("{}", "-->>> Deploy failed"),
      }),
    ),
  }
}

async fn verify(Json(req): Json<Request>) -> (StatusCode, Json<Response>) {
  match run_command("./verify.sh", req.path.as_deref()).await {
    // 假设有verify脚本
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
