# Instalación / Installation

Guía completa de instalación y desinstalación por sistema operativo.

Complete installation and uninstallation guide by operating system.

---

## Prerrequisitos / Prerequisites

- **Go** >= 1.22 ([https://go.dev/dl/](https://go.dev/dl/))
- **Git** (para clonar el repositorio / to clone the repository)

---

## macOS

### Instalar / Install

**Opción 1: make install (recomendado / recommended)**

```bash
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
make install
```

El binario se instala en `~/go/bin/cli-devforge`.

The binary is installed to `~/go/bin/cli-devforge`.

**Opción 2: go install directo / Direct go install**

```bash
go install github.com/jcvalenzo/cli-devforge@latest
```

**Verificar / Verify:**

```bash
~/go/bin/cli-devforge version
# → devforge dev
```

**Agregar al PATH (opcional / optional):**

Agrega esto a tu `~/.zshrc` o `~/.bash_profile`:

Add this to your `~/.zshrc` or `~/.bash_profile`:

```bash
export PATH="$HOME/go/bin:$PATH"
```

Luego ejecuta / Then run:

```bash
source ~/.zshrc  # o source ~/.bash_profile
cli-devforge version
```

### Desinstalar / Uninstall

```bash
# 1. Eliminar el binario / Remove the binary
rm -f ~/go/bin/cli-devforge

# 2. Eliminar archivos compilados del repo (si existen) / Remove build artifacts (if any)
cd cli-devforge && make clean

# 3. (Opcional) Eliminar la línea de PATH de tu shell config
# (Optional) Remove the PATH line from your shell config
# Busca y elimina: export PATH="$HOME/go/bin:$PATH"
```

**Verificar desinstalación / Verify uninstall:**

```bash
which cli-devforge 2>/dev/null && echo "Aún instalado / Still installed" || echo "Desinstalado / Uninstalled"
```

---

## Linux (Debian/Ubuntu)

### Instalar / Install

**Opción 1: make install (recomendado / recommended)**

```bash
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
make install
```

El binario se instala en `~/go/bin/cli-devforge`.

The binary is installed to `~/go/bin/cli-devforge`.

**Opción 2: go install directo / Direct go install**

```bash
go install github.com/jcvalenzo/cli-devforge@latest
```

**Verificar / Verify:**

```bash
~/go/bin/cli-devforge version
# → devforge dev
```

**Agregar al PATH (opcional / optional):**

Agrega esto a tu `~/.bashrc`:

Add this to your `~/.bashrc`:

```bash
export PATH="$HOME/go/bin:$PATH"
```

Luego ejecuta / Then run:

```bash
source ~/.bashrc
cli-devforge version
```

### Desinstalar / Uninstall

```bash
# 1. Eliminar el binario / Remove the binary
rm -f ~/go/bin/cli-devforge

# 2. Eliminar archivos compilados del repo / Remove build artifacts
cd cli-devforge && make clean

# 3. (Opcional) Eliminar la línea de PATH de tu ~/.bashrc
# (Optional) Remove the PATH line from your ~/.bashrc
```

**Verificar desinstalación / Verify uninstall:**

```bash
which cli-devforge 2>/dev/null && echo "Aún instalado / Still installed" || echo "Desinstalado / Uninstalled"
```

---

## Linux (RHEL/CentOS/Fedora)

### Instalar / Install

**Opción 1: make install (recomendado / recommended)**

```bash
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
make install
```

El binario se instala en `~/go/bin/cli-devforge`.

The binary is installed to `~/go/bin/cli-devforge`.

**Opción 2: go install directo / Direct go install**

```bash
go install github.com/jcvalenzo/cli-devforge@latest
```

**Verificar / Verify:**

```bash
~/go/bin/cli-devforge version
# → devforge dev
```

**Agregar al PATH (opcional / optional):**

Agrega esto a tu `~/.bashrc` o `~/.bash_profile`:

Add this to your `~/.bashrc` or `~/.bash_profile`:

```bash
export PATH="$HOME/go/bin:$PATH"
```

Luego ejecuta / Then run:

```bash
source ~/.bashrc  # o source ~/.bash_profile
cli-devforge version
```

### Desinstalar / Uninstall

```bash
# 1. Eliminar el binario / Remove the binary
rm -f ~/go/bin/cli-devforge

# 2. Eliminar archivos compilados del repo / Remove build artifacts
cd cli-devforge && make clean

# 3. (Opcional) Eliminar la línea de PATH de tu shell config
# (Optional) Remove the PATH line from your shell config
```

**Verificar desinstalación / Verify uninstall:**

```bash
which cli-devforge 2>/dev/null && echo "Aún instalado / Still installed" || echo "Desinstalado / Uninstalled"
```

---

## Linux (Universal - build from source)

Si no tienes Go instalado, compila desde el código fuente:

If you don't have Go installed, build from source:

### Instalar / Install

```bash
# 1. Instalar Go (si no está) / Install Go (if not present)
# Visita / Visit: https://go.dev/dl/

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
# 1. Eliminar el binario del sistema / Remove system binary
sudo rm -f /usr/local/bin/devforge

# 2. Eliminar archivos compilados del repo / Remove build artifacts
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

El binario se instala en `%GOPATH%\bin\cli-devforge.exe`.

The binary is installed to `%GOPATH%\bin\cli-devforge.exe`.

**Opción 2: PowerShell**

```powershell
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
go build -o bin\devforge.exe .
go install .
```

**Verificar / Verify:**

```powershell
cli-devforge version
# → devforge dev
```

**Agregar al PATH (si no está / if not present):**

1. Busca `%GOPATH%\bin` (usualmente `C:\Users\<tu-usuario>\go\bin`)
2. Agrégalo a las Variables de Sistema > Path

Look for `%GOPATH%\bin` (usually `C:\Users\<your-user>\go\bin`)
Add it to System Variables > Path

### Desinstalar / Uninstall

```powershell
# 1. Eliminar el binario / Remove the binary
del %GOPATH%\bin\cli-devforge.exe

# 2. Eliminar archivos compilados del repo / Remove build artifacts
cd cli-devforge
del /s /q bin\
rmdir bin
```

**Verificar desinstalación / Verify uninstall:**

```powershell
where cli-devforge 2>nul && echo "Aún instalado / Still installed" || echo "Desinstalado / Uninstalled"
```

---

## Solución de Problemas / Troubleshooting

### "command not found" / "cli-devforge is not recognized"

El binario no está en tu PATH.

The binary is not in your PATH.

**Solución / Fix:**

```bash
# Verificar si el binario existe / Check if binary exists
ls ~/go/bin/cli-devforge

# Si existe, agregar al PATH / If it exists, add to PATH
export PATH="$HOME/go/bin:$PATH"

# Para hacerlo permanente, agrega la línea a tu shell config
# To make it permanent, add the line to your shell config
```

### "permission denied" / "denied"

**Solución / Fix:**

```bash
chmod +x ~/go/bin/cli-devforge
```

### "go: command not found"

Go no está instalado o no está en tu PATH.

Go is not installed or not in your PATH.

**Solución / Fix:**

```bash
# Verificar / Check
go version

# Si no se encuentra, instalar Go desde / If not found, install Go from
# https://go.dev/dl/
```

### "no templates found for <prefix>/<lang>"

El lenguaje no es soportado para ese prefijo.

The language is not supported for that prefix.

**Solución / Fix:**

```bash
# Listar prefijos y lenguajes soportados
# List prefixes and supported languages
devforge list
```
