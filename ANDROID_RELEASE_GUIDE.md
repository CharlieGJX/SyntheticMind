# 📱 Guía de Compilación y Distribución para Android (Termux)

Esta guía explica cómo generar, empaquetar y distribuir binarios de `vibe-ai` optimizados para dispositivos Android utilizando el entorno **Termux**.

---

## 1. Arquitecturas Soportadas

Para cubrir la totalidad del ecosistema Android, es necesario compilar para las siguientes arquitecturas:

| Arquitectura | Target de Rust | Dispositivos |
| :--- | :--- | :--- |
| **aarch64** | `aarch64-unknown-linux-musl` | Casi todos los smartphones modernos (64-bit). |
| **armv7** | `armv7-unknown-linux-musleabihf` | Dispositivos antiguos o de gama baja (32-bit). |
| **x86_64** | `x86_64-unknown-linux-musl` | Emuladores de Android y algunas tablets (Intel/AMD). |

### ¿Por qué `musl` en lugar de `android-ndk`?
Usamos el target `musl` para generar **binarios estáticos**. Esto garantiza que `vibe-ai` funcione en cualquier versión de Android (desde la 7.0 hasta la 15+) sin preocuparse por la versión de la `libc` de Android instalada, lo cual es un problema común en Termux.

---

## 2. Automatización con GitHub Actions

El archivo `.github/workflows/release.yaml` ya está configurado para manejar estas arquitecturas. Aquí te explicamos cómo se hace de forma "profesional":

### Configuración del Matrix
En el flujo de trabajo de GitHub, utilizamos una matriz para compilar en paralelo:

```yaml
strategy:
  matrix:
    include:
      - target: aarch64-unknown-linux-musl
        os: ubuntu-latest
        use-cross: true  # Usa 'cross-rs' para compilación cruzada
      - target: armv7-unknown-linux-musleabihf
        os: ubuntu-latest
        use-cross: true
```

### El uso de `cross`
Para compilar desde servidores de GitHub (Ubuntu) hacia arquitecturas ARM, utilizamos [cross-rs](https://github.com/cross-rs/cross). Este utiliza contenedores Docker con el toolchain preconfigurado, evitando errores de enlazado (`linker errors`).

---

## 3. Empaquetado para Termux

Para que los usuarios puedan instalar `vibe-ai` fácilmente, el binario debe estar empaquetado correctamente. El flujo actual genera archivos `.tar.gz` con la siguiente estructura interna:

```text
vibe-ai-v0.x.x-aarch64.tar.gz
├── vibe-ai                # El binario ejecutable
└── completions/           # Scripts de autocompletado (bash, zsh, fish)
    ├── vibe-ai.bash
    └── ...
```

### Proceso de liberación (Releases)
1. **Crear un Tag:** Al subir un tag (ej. `git tag v1.0.0 && git push --tags`), el flujo se dispara.
2. **Compilación Paralela:** GitHub genera los binarios para todas las arquitecturas.
3. **Subida de Assets:** Los archivos resultantes se adjuntan automáticamente a la sección de "Releases" de GitHub.

---

## 4. Guía de Instalación para el Usuario Final

Puedes incluir estas instrucciones en tu `README.md` o en la descripción del Release para ayudar a tus usuarios:

### Instalación Manual en Termux
```bash
# 1. Identificar la arquitectura
ARCH=$(uname -m)

# 2. Descargar el binario (ejemplo para aarch64)
curl -LO https://github.com/TU_USUARIO/vibe-ai/releases/download/v1.0.0/vibe-ai-v1.0.0-${ARCH}.tar.gz

# 3. Extraer y mover al PATH
tar -xzf vibe-ai-*.tar.gz
mv vibe-ai $PREFIX/bin/
chmod +x $PREFIX/bin/vibe-ai

# 4. (Opcional) Instalar autocompletado para Bash
mkdir -p ~/.bash_completion.d
cp completions/vibe-ai.bash ~/.bash_completion.d/
echo "source ~/.bash_completion.d/vibe-ai.bash" >> ~/.bashrc
```

---

## 5. Optimización del Binario

Para reducir el tamaño del binario (importante en móviles con poco espacio), el perfil de `release` en `Cargo.toml` debería verse así:

```toml
[profile.release]
opt-level = 'z'     # Optimiza por tamaño
lto = true          # Link Time Optimization para reducir dead code
codegen-units = 1   # Mayor optimización a costa de tiempo de compilación
strip = true        # Elimina símbolos de depuración (reduce ~40% el tamaño)
```

---

## 6. Colaboración y Feedback

Si otros desarrolladores quieren contribuir con binarios para arquitecturas más exóticas (como RISC-V), solo deben agregar una línea al `matrix` en `.github/workflows/release.yaml` siguiendo el patrón de `musl`.

Esta arquitectura garantiza que `vibe-ai` sea una herramienta de IA **realmente portátil**, capaz de correr desde un servidor en la nube hasta el bolsillo del usuario.
