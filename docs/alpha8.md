# Alpha 8 acceptance

## Approved architecture

Vega owns the management UI and backend client. Lyra VMs is only the graphical
console entry point; the initial version reuses virt-viewer. Libvirt owns machine
execution independently of either GUI. Closing either GUI must not stop a guest.
The items below cover the combined feature, not a second management application.

## Qualification checklist

- Native responsive GTK interface: search, empty/loading/error states, clear
  running/stopped/paused states, no misleading success before backend completion.
- Reuse QEMU/libvirt, discover existing domains by UUID, authenticate via Polkit.
- Create/import without overwriting source media; validate CPU/RAM/disk limits;
  undo only newly created resources on failure.
- Start, graceful shutdown, confirmed force stop, pause/resume, console and
  offline CPU/RAM configuration, with state checks at execution time.
- Remove only the selected UUID after confirmation; preserve existing/external
  disks, do not recurse through filesystem trees or delete shared volumes.
- English fallback with pt-BR and es-ES UI translations and keyboard navigation.
- Meaningful failure/cancellation tests, disposable VM integration and native UI
  tests; package/vendor/CI/staging/release gates before recipe integration.
- Audit before candidate build, then qualify exact ISO checksum on VM/hardware.

Storage follows XDG: ${XDG_DATA_HOME:-$HOME/.local/share}/lyra-vms for
personal machines using qemu:///session. The maintainer proposed a hidden home
folder and then .local/shared/vms; corrected/recommended share/lyra-vms.
Default to personal VMs. Offer the system connection separately, preserving
existing system domain paths; newly created system disks use a libvirt-managed
/var/lib/libvirt/images/lyra-vms pool. Never chmod the user's home or move
existing storage automatically.
