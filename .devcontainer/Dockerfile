FROM quay.io/fedora/fedora-minimal:42

RUN echo install_weak_deps=False >> /etc/dnf/dnf.conf
RUN dnf install -y git eza cc clang lld libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel libxkbcommon sshd xauth 
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y