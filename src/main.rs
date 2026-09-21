use adw::prelude::*;
use std::{ffi::OsString, os::unix::process::CommandExt, process::Command};

fn text(en: &'static str, pt: &'static str, es: &'static str) -> &'static str {
    let locale = ["LC_ALL", "LANGUAGE", "LC_MESSAGES", "LANG"]
        .iter()
        .find_map(|key| std::env::var(key).ok().filter(|v| !v.is_empty()))
        .unwrap_or_default();
    if locale.starts_with("pt") {
        pt
    } else if locale.starts_with("es") {
        es
    } else {
        en
    }
}

fn arguments(args: &[String]) -> Result<Option<Vec<OsString>>, String> {
    if args.is_empty() {
        return Ok(None);
    }
    if args.len() != 4 || args[0] != "--connect" || args[2] != "--uuid" {
        return Err("Usage: lyra-vms --connect qemu:///session|qemu:///system --uuid UUID".into());
    }
    if !matches!(args[1].as_str(), "qemu:///session" | "qemu:///system") {
        return Err(text(
            "Only local personal and system connections are supported.",
            "São aceitas apenas conexões pessoais ou do sistema neste computador.",
            "Solo se admiten conexiones personales o del sistema en este equipo.",
        )
        .into());
    }
    let uuid = uuid::Uuid::parse_str(&args[3]).map_err(|_| {
        text(
            "Invalid machine UUID",
            "UUID da máquina inválido",
            "UUID de máquina no válido",
        )
    })?;
    Ok(Some(vec![
        "--connect".into(),
        args[1].clone().into(),
        "--attach".into(),
        "--reconnect".into(),
        "--wait".into(),
        "--uuid".into(),
        "--".into(),
        uuid.to_string().into(),
    ]))
}

fn main() -> gtk::glib::ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["--help"] {
        println!(
            "Lyra VMs — graphical console (virt-viewer)\nUsage: lyra-vms --connect qemu:///session|qemu:///system --uuid UUID\nClose the console without shutting down the guest. Manage machines in Vega."
        );
        return gtk::glib::ExitCode::SUCCESS;
    }
    let message = match arguments(&args) {
        Ok(Some(args)) => {
            // Replace this process; no child supervisor can kill or shut down the VM.
            let error = Command::new("/usr/bin/virt-viewer").args(args).exec();
            format!(
                "{}\n{error}",
                text(
                    "Could not open the console. Install the virt-viewer package.",
                    "Não foi possível abrir a tela. Instale o pacote virt-viewer.",
                    "No se pudo abrir la pantalla. Instale el paquete virt-viewer."
                )
            )
        }
        Ok(None) => {
            let error = Command::new("/usr/bin/vega-gtk")
                .arg("--virtual-machines")
                .exec();
            format!(
                "{}\n{error}",
                text(
                    "Could not open Vega. Install Vega to select a virtual machine.",
                    "Não foi possível abrir o Vega. Instale o Vega para selecionar uma máquina virtual.",
                    "No se pudo abrir Vega. Instale Vega para seleccionar una máquina virtual."
                )
            )
        }
        Err(error) => error,
    };
    eprintln!("{message}");
    let app = adw::Application::builder()
        .application_id("org.lyraos.VMs")
        .flags(gtk::gio::ApplicationFlags::NON_UNIQUE)
        .build();
    app.connect_activate(move |app| {
        let page = adw::StatusPage::builder()
            .title("Lyra VMs")
            .icon_name("computer-symbolic")
            .description(&message)
            .build();
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Lyra VMs")
            .default_width(480)
            .default_height(320)
            .content(&page)
            .build();
        window.present();
    });
    app.run_with_args::<&str>(&[])
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse(args: &[&str]) -> Result<Option<Vec<OsString>>, String> {
        arguments(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }
    #[test]
    fn rejects_remote_connections_and_ambiguous_identifiers() {
        let uuid = "a84fdf47-f7f3-4306-9358-d2d3f38af37c";
        assert!(parse(&["--connect", "qemu+ssh://host/system", "--uuid", uuid]).is_err());
        assert!(parse(&["--connect", "qemu:///system", "--uuid", "--help"]).is_err());
        assert!(parse(&["--connect", "qemu:///system", "--uuid", "guest-name"]).is_err());
        let args = parse(&["--connect", "qemu:///session", "--uuid", uuid])
            .unwrap()
            .unwrap();
        assert_eq!(args[1], "qemu:///session");
        assert_eq!(args.last().unwrap(), uuid);
        assert!(!args.iter().any(|arg| arg == "--kiosk"));
    }
}
