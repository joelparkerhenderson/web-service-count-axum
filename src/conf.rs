//! Configuration module for the application.

#[allow(dead_code)]

/// Get the bind string from the environment variable BIND,
/// or return the default bind address string "0.0.0.0:8080".
pub async fn bind_string() -> String {
    if let Ok(x) = std::env::var("BIND") {
        return x
    }
    format!("{}:{}", &host_string().await, &port_string().await)
}

/// Get the host string from the environment variable HOST,
/// or return the default host IP address string "0.0.0.0".
pub async fn host_string() -> String {
    if let Ok(x) = std::env::var("HOST") {
        return x
    }
    String::from("0.0.0.0")
}

/// Get the port string from the environment variable PORT,
/// or return the default port number string "8080".
pub async fn port_string() -> String {
    if let Ok(x) = std::env::var("PORT") {
        return x
    }
    String::from("8080")
}

/// Shutdown signal to run axum with graceful shutdown when
/// a user presses Ctrl+C or Unix sends a terminate signal.
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    mod test_bind_string {
        use super::*;

        #[tokio::test]
        #[serial]
        async fn with_default() {
            unsafe {
                std::env::remove_var("BIND");
                std::env::remove_var("HOST");
                std::env::remove_var("PORT");
            }
            assert_eq!(bind_string().await, "0.0.0.0:8080");
        }

        #[tokio::test]
        #[serial]
        async fn with_env_var() {
            unsafe {
                std::env::set_var("BIND", "1.1.1.1:1111");
            }
            assert_eq!(bind_string().await, "1.1.1.1:1111");
        }

        #[tokio::test]
        #[serial]
        async fn with_env_vars_for_host_and_port() {
            unsafe {
                std::env::remove_var("BIND");
                std::env::set_var("HOST", "2.2.2.2");
                std::env::set_var("PORT", "2222");
            }
            assert_eq!(bind_string().await, "2.2.2.2:2222");
        }

    }

    mod test_host_string {
        use super::*;

        #[tokio::test]
        #[serial]
        async fn with_default() {
            unsafe {
                std::env::remove_var("HOST");
            }
            assert_eq!(host_string().await, "0.0.0.0");
        }

        #[tokio::test]
        #[serial]
        async fn with_env_var() {
            unsafe {
                std::env::set_var("HOST", "1.1.1.1");
            }
            assert_eq!(host_string().await, "1.1.1.1");
        }

    }

    mod test_port {
        use super::*;

        #[tokio::test]
        #[serial]
        async fn with_default() {
            unsafe {
                std::env::remove_var("PORT");
            }
            assert_eq!(port_string().await, "8080");
        }

        #[tokio::test]
        #[serial]
        async fn with_env_var() {
            unsafe {
                std::env::set_var("PORT", "1111");
            }
            assert_eq!(port_string().await, "1111");
        }
    }

    mod test_shutdown_signal {
        use super::*;

        #[tokio::test]
        async fn test_shutdown_signal() {
            // This test just calls shutdown_signal() to ensure it compiles and runs.
            // We cannot easily test the actual signal handling in an automated test.
            let _ = tokio::spawn(async {
                shutdown_signal().await;
            });
        }

    }

}
