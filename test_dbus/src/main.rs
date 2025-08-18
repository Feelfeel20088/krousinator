use zbus::proxy;
use zbus::{Connection, Result, zvariant::Value};

#[proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: std::collections::HashMap<String, zbus::zvariant::Value<'_>>,
        expire_timeout: i32,
    ) -> Result<u32>;
}

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;
    let proxy = NotificationsProxy::new(&connection).await?;

    let mut hints = std::collections::HashMap::new();
    hints.insert(
        "image-path".to_string(),
        Value::from("file:///home/felix/projects/krousinator/images/0C4AF4A4-289D-48B6-B742-CC391192DE9B_1_105_c.png"),
    );

    let notification_id = proxy
        .notify(
            "???",
            0,
            "",
            "???",
            "Kuvas is always watching 0,0",
            &[],
            hints,
            5000, // Timeout in ms
        )
        .await
        .unwrap();

    println!("Notification sent with ID: {}", notification_id);

    Ok(())
}
