# Instalación / Installation

Guía completa de instalación y desinstalación por sistema operativo.

Complete installation and uninstallation guide by operating system.

---

## Prerrequisitos / Prerequisites

- **Rust** stable ([https://rustup.rs/](https://rustup.rs/))
- **Git** (para clonar el repositorio / to clone the repository)

Instala Rust con `rustup` / Install Rust with `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

---

## macOS

### Instalar / Install

**Opción 1: make install (recomendado / recommended)**

```bash
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
make install
```

El binario se instala en `~/.cargo/bin/devforge`.

The binary is installed to `~/.cargo/bin/devforge`.

**Opción 2: cargo install directo / Direct cargo install**

```bash
cargo install --git https://github.com/jcvalenzo/cli-devforge --locked
```

**Opción 3: compilar desde el repositorio / Build from the repository**

```bash
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
make build
./bin/devforge version
```

**Verificar / Verify:**

```bash
~/.cargo/bin/devforge version
# → devforge v0.5.0 o devforge dev
```

**Agregar al PATH (opcional / optional):**

Agrega esto a tu `~/.zshrc` o `~/.bash_profile`:

Add this to your `~/.zshrc` or `~/.bash_profile`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Luego ejecuta / Then run:

```bash
source ~/.zshrc  # o source ~/.bash_profile
devforge version
```

### Desinstalar / Uninstall

```bash
# 1. Eliminar el binario / Remove the binary
cargo uninstall devforge

# 2. Eliminar archivos compilados del repo (si existen) / Remove build artifacts (if any)
cd cli-devforge && make clean

# 3. Eliminar el binario legado de Go (si existe) / Remove legacy Go binary (if any)
rm -f ~/go/bin/cli-devforge
```

**Verificar desinstalación / Verify uninstall:**

```bash
which devforge 2>/dev/null && echo "Aún instalado / Still installed" || echo "Desinstalado / Uninstalled"
```

---

## Linux (Debian/Ubuntu)

### Instalar / Install

```bash
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
make install
```

El binario se instala en `~/.cargo/bin/devforge`.

The binary is installed to `~/.cargo/bin/devforge`.

Alternativa / Alternative:

```bash
cargo install --git https://github.com/jcvalenzo/cli-devforge --locked
```

**Verificar / Verify:**

```bash
~/.cargo/bin/devforge version
```

**Agregar al PATH (opcional / optional):**

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Desinstalar / Uninstall

```bash
cargo uninstall devforge
cd cli-devforge && make clean
rm -f ~/go/bin/cli-devforge
```

---

## Linux (RHEL/CentOS/Fedora)

### Instalar / Install

```bash
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
make install
```

El binario se instala en `~/.cargo/bin/devforge`.

The binary is installed to `~/.cargo/bin/devforge`.

Alternativa / Alternative:

```bash
cargo install --git https://github.com/jcvalenzo/cli-devforge --locked
```

**Agregar al PATH (opcional / optional):**

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Desinstalar / Uninstall

```bash
cargo uninstall devforge
cd cli-devforge && make clean
rm -f ~/go/bin/cli-devforge
```

---

## Linux (Universal - build from source)

Si no quieres instalar en `~/.cargo/bin`, compila y copia el binario:

If you don't want to install into `~/.cargo/bin`, build and copy the binary:

```bash
# 1. Instalar Rust (si no está) / Install Rust (if not present)
# Visita / Visit: https://rustup.rs/

# 2. Clonar y compilar / Clone and build
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
make build

# 3. Copiar el binario a un directorio en tu PATH
# Copy the binary to a directory in your PATH
sudo cp bin/devforge /usr/local/bin/devforge

# 4. Verificar / Verify
devforge version
```

### Desinstalar / Uninstall

```bash
sudo rm -f /usr/local/bin/devforge
cd cli-devforge && make clean
```

---

## Windows

### Instalar / Install

**Opción 1: Git Bash o WSL (recomendado / recommended)**

```bash
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
make install
```

El binario se instala en `%USERPROFILE%\.cargo\bin\devforge.exe`.

The binary is installed to `%USERPROFILE%\.cargo\bin\devforge.exe`.

**Opción 2: PowerShell**

```powershell
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
cargo install --path . --locked
```

**Opción 3: compilar sin instalar / Build without installing**

```powershell
cargo build --release --locked
.\target\release\devforge.exe version
```

**Verificar / Verify:**

```powershell
devforge version
# → devforge v0.5.0 o devforge dev
```

**Agregar al PATH (si no está / if not present):**

1. Busca `%USERPROFILE%\.cargo\bin` (usualmente `C:\Users\<tu-usuario>\.cargo\bin`)
2. Agrégalo a las Variables de Sistema > Path

Look for `%USERPROFILE%\.cargo\bin` (usually `C:\Users\<your-user>\.cargo\bin`)
Add it to System Variables > Path

### Desinstalar / Uninstall

```powershell
# 1. Eliminar el binario / Remove the binary
cargo uninstall devforge

# 2. Eliminar archivos compilados del repo / Remove build artifacts
cd cli-devforge
make clean
```

**Verificar desinstalación / Verify uninstall:**

```powershell
where devforge 2>nul && echo "Aún instalado / Still installed" || echo "Desinstalado / Uninstalled"
```

---

## Descargar binarios de GitHub Releases / Download from GitHub Releases

Descarga el asset correspondiente desde [GitHub Releases](https://github.com/jcvalenzo/cli-devforge/releases/latest):

| Sistema Operativo / OS | Arquitectura / Arch | Archivo / File |
|-------------------------|---------------------|----------------|
| macOS | Intel (x86_64) | `devforge-darwin-amd64` |
| macOS | Apple Silicon (arm64) | `devforge-darwin-arm64` |
| Linux | x86_64 | `devforge-linux-amd64` |
| Linux | ARM64 | `devforge-linux-arm64` |
| Windows | x86_64 | `devforge-windows-amd64.exe` |

```bash
chmod +x devforge-<os>-<arch>
mv devforge-<os>-<arch> ~/.local/bin/devforge
sha256sum -c checksums.txt
```

En Windows, renombra el `.exe` a `devforge.exe` y colócalo en un directorio de tu `PATH`.

On Windows, rename the `.exe` to `devforge.exe` and place it in a directory in your `PATH`.

---

## Migración desde la versión Go / Migrating from the Go version

La versión anterior se instalaba como `cli-devforge` en `~/go/bin` (o `%GOPATH%\bin`). La versión Rust usa `devforge` en `~/.cargo/bin`.

The previous version installed `cli-devforge` into `~/go/bin` (or `%GOPATH%\bin`). The Rust version uses `devforge` in `~/.cargo/bin`.

```bash
rm -f ~/go/bin/cli-devforge
which -a devforge cli-devforge
```

Si tienes ambos binarios, asegúrate de que `~/.cargo/bin` aparezca antes que `~/go/bin` en tu `PATH`.

If both binaries exist, make sure `~/.cargo/bin` appears before `~/go/bin` in your `PATH`.

---

## Solución de Problemas / Troubleshooting

### "command not found" / "devforge is not recognized"

El binario no está en tu PATH.

The binary is not in your PATH.

```bash
ls ~/.cargo/bin/devforge
export PATH="$HOME/.cargo/bin:$PATH"
```

### "permission denied" / "denied"

```bash
chmod +x ~/.cargo/bin/devforge
```

### "cargo: command not found"

Rust no está instalado o no está en tu PATH.

Rust is not installed or not in your PATH.

```bash
rustc --version
cargo --version
# Si no se encuentra / If not found: https://rustup.rs/
```

### "no templates found for <prefix>/<lang>"

El lenguaje no es soportado (o no tiene plantillas todavía) para ese prefijo.

The language is not supported (or has no templates yet) for that prefix.

```bash
devforge list
```

> **Nota / Note:** `worker-` anuncia `python`, pero todavía no tiene plantillas. Usa `--lang go`.
> `worker-` advertises `python`, but it has no templates yet. Use `--lang go`.
