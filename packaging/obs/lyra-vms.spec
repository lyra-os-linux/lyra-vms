Name:           lyra-vms
Version:        0.1.0
Release:        0
Summary:        Graphical console launcher for Lyra OS virtual machines
License:        GPL-3.0-or-later
URL:            https://github.com/lyra-os-linux/lyra-vms
Source0:        %{name}-%{version}.tar.xz
Source1:        vendor.tar.xz
BuildRequires:  cargo
BuildRequires:  rust >= 1.92
BuildRequires:  pkgconfig(gtk4) >= 4.10
BuildRequires:  pkgconfig(libadwaita-1) >= 1.5
BuildRequires:  desktop-file-utils
Requires:       virt-viewer
Requires:       vega-gtk >= 5.1.41

%description
Open a local virtual machine graphical console using virt-viewer.
Virtual machines are managed in Vega and executed by libvirt/QEMU.
Closing the console does not shut down the guest.

%prep
%autosetup
tar -xJf %{SOURCE1}

%build
cargo build --release --frozen

%install
install -Dm755 target/release/lyra-vms %{buildroot}%{_bindir}/lyra-vms
install -Dm644 data/org.lyraos.VMs.desktop %{buildroot}%{_datadir}/applications/org.lyraos.VMs.desktop

%check
desktop-file-validate data/org.lyraos.VMs.desktop
cargo test --frozen

%files
%license LICENSE
%doc README.md
%{_bindir}/lyra-vms
%{_datadir}/applications/org.lyraos.VMs.desktop

%changelog
