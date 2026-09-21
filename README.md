# Lyra VMs

Graphical console entry point for local QEMU/KVM virtual machines. **Vega manages
machines; Lyra VMs opens their display; libvirt/QEMU execute them.** This division
was approved by the maintainer on 21 September 2026.

Approved for the Lyra OS Alpha 8 scope by the maintainer on 21 September 2026,
including possible release delay. This repository is under development; it is
not yet a qualified replacement for virt-manager.

The initial console delegates to the existing native GTK `virt-viewer` program.
It is not yet a custom embedded GTK4 console. The Rust launcher accepts only a
local connection and a validated UUID, and requests attachment through libvirt.
Closing the viewer never sends a shutdown request. Restarting a guest reconnects
the display. CPU/RAM, storage and lifecycle controls belong to Vega, not here.

```
lyra-vms --connect qemu:///session --uuid MACHINE-UUID
lyra-vms --connect qemu:///system --uuid MACHINE-UUID
```

Personal machines use `$XDG_DATA_HOME/lyra-vms`, defaulting to
`~/.local/share/lyra-vms`. Existing system disks keep their original paths.

The application runs as the desktop user. Administrative access uses libvirt
and the system's Polkit agent, never a root GTK process or a global allow rule.
Tests that mutate VMs must target a marked
disposable VM or libvirt test driver, never the developer workstation's domains.

Keep virt-manager available until this application's package and exact candidate
ISO pass qualification. Offline SDK work remains outside this task.
