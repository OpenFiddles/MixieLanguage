#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

have() { command -v "$1" >/dev/null 2>&1; }
ask() {
  if have dialog; then
    dialog --title "$1" --yesno "$2" 10 68
  else
    printf '%s [y/N] ' "$2"
    read -r answer
    [[ "$answer" == "y" || "$answer" == "Y" ]]
  fi
}
info() {
  if have dialog; then dialog --title "$1" --msgbox "$2" 10 68 || true
  else printf '\n== %s ==\n%s\n' "$1" "$2"; fi
}
menu() {
  if have dialog; then
    dialog --clear --title "MixieLand First-run Setup" --menu "Choose an action:" 16 70 6 \
      install "Install prerequisites" \
      build "Build MixieLang (release)" \
      test "Run test suite" \
      launch "Open MixieLang setup menu" \
      quit "Exit" 3>&1 1>&2 2>&3
  else
    printf '\nMixieLand setup\n1) Install prerequisites\n2) Build MixieLang\n3) Run tests\n4) Open setup menu\n0) Exit\nChoose: ' >&2
    read -r n
    case "$n" in 1) echo install;; 2) echo build;; 3) echo test;; 4) echo launch;; *) echo quit;; esac
  fi
}
install_deps() {
  local manager=""
  if [[ -n "${TERMUX_VERSION:-}" || "${PREFIX:-}" == *com.termux* ]]; then manager=pkg
  elif have apt-get; then manager=apt
  elif have dnf; then manager=dnf
  elif have pacman; then manager=pacman
  elif have apk; then manager=apk
  elif have zypper; then manager=zypper
  elif have brew; then manager=brew
  else
    info "Manual installation" "No supported package manager found. Install Rust/Cargo, Git, Make, and dialog with your platform's official package manager."
    return
  fi

  local packages=()
  case "$manager" in
    pkg) packages=(rust git make clang dialog);;
    apt) packages=(rustc cargo git make dialog);;
    dnf) packages=(rust cargo git make dialog);;
    pacman) packages=(rust git base-devel dialog);;
    apk) packages=(rust cargo git make dialog);;
    zypper) packages=(rust cargo git make dialog);;
    brew) packages=(rust git make dialog);;
  esac
  local command=()
  case "$manager" in
    pkg) command=(pkg install -y "${packages[@]}");;
    apt) command=(sudo apt-get update);;
    dnf) command=(sudo dnf install -y "${packages[@]}");;
    pacman) command=(sudo pacman -S --needed --noconfirm "${packages[@]}");;
    apk) command=(sudo apk add "${packages[@]}");;
    zypper) command=(sudo zypper --non-interactive install "${packages[@]}");;
    brew) command=(brew install "${packages[@]}");;
  esac
  if [[ "$manager" == apt ]]; then
    if [[ "$EUID" -eq 0 ]]; then apt-get update && apt-get install -y "${packages[@]}"
    else sudo apt-get update && sudo apt-get install -y "${packages[@]}"; fi
  else
    if [[ "$manager" != pkg && "$manager" != brew && "$EUID" -ne 0 ]]; then
      command=(sudo "${command[@]:1}")
    fi
    "${command[@]}"
  fi
}
while true; do
  choice="$(menu)" || exit 0
  case "$choice" in
    install)
      if ask "Install prerequisites" "Install Rust/Cargo, Git, Make, and dialog using the detected package manager?"; then
        install_deps && info "Done" "Prerequisite installation finished. Choose Build next."
      fi
      ;;
    build)
      cargo build --release && info "Build complete" "Binary: target/release/mixie"
      ;;
    test)
      cargo test --all-targets && info "Tests passed" "All test targets passed."
      ;;
    launch)
      if [[ -x "$ROOT/target/release/mixie" ]]; then "$ROOT/target/release/mixie" setup
      else info "Build required" "Build MixieLang first."; fi
      ;;
    quit|*) exit 0;;
  esac
done
