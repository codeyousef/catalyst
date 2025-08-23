Name:           catalyst-git
Version:        0.4.2.{{{ git_dir_version }}}
Release:        1
Summary:        Lightning-fast and Powerful Code Editor written in Rust
License:        Apache-2.0
URL:            https://catalyst-ide.dev

VCS:            {{{ git_dir_vcs }}}
Source:        	{{{ git_dir_pack }}}

BuildRequires:  cargo libxkbcommon-x11-devel libxcb-devel vulkan-loader-devel wayland-devel openssl-devel pkgconf libxkbcommon-x11-devel

%description
Catalyst is written in pure Rust, with a UI in Floem (also written in Rust).
It is designed with Rope Science from the Xi-Editor, enabling lightning-fast computation, and leverages wgpu for rendering.

%prep
{{{ git_dir_setup_macro }}}
cargo fetch --locked

%build
cargo build --profile release-lto --package catalyst-app --frozen

%install
install -Dm755 target/release-lto/catalyst %{buildroot}%{_bindir}/catalyst
install -Dm644 extra/linux/dev.catalyst.catalyst.desktop %{buildroot}/usr/share/applications/dev.catalyst.catalyst.desktop
install -Dm644 extra/linux/dev.catalyst.catalyst.metainfo.xml %{buildroot}/usr/share/metainfo/dev.catalyst.catalyst.metainfo.xml
install -Dm644 extra/images/logo.png %{buildroot}/usr/share/pixmaps/dev.catalyst.catalyst.png

%files
%license LICENSE*
%doc *.md
%{_bindir}/catalyst
/usr/share/applications/dev.catalyst.catalyst.desktop
/usr/share/metainfo/dev.catalyst.catalyst.metainfo.xml
/usr/share/pixmaps/dev.catalyst.catalyst.png

%changelog
* Mon Jan 01 2024 Jakub Panek
- See full changelog on GitHub
