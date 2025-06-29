#!/usr/bin/bash

echo PS1="\"\[\033[1;35m\]\u\[\033[0m\]@\[\033[1;36m\]\h\[\033[0m\] \[\033[34m\]\w\[\033[0m\]\n└> \[\033[1;32m\]$\[\033[0m\] \"" >> ~/.bashrc
echo alias ls="\"eza --icons -lah --sort=Name --group-directories-first --git\"" >> ~/.bashrc
echo export DISPLAY="\"127.0.0.1:10.0\"" >> ~/.bashrc
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
