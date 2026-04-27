# 🚀 Guía de Instalación de vibe-ai en Termux

Esta guía te permitirá instalar **vibe-ai** (un fork potente de SyntheticMind) en tu entorno de Android mediante Termux.

## 1. Requisitos Previos

Asegúrate de tener actualizado Termux y los paquetes base:

```bash
pkg update && pkg upgrade
pkg install git rust make clang binutils
```

## 2. Clonar el Repositorio

Clona el fork oficial desde GitHub:

```bash
git clone https://github.com/CharlieGJX/SyntheticMind.git
cd SyntheticMind
```

## 3. Compilación e Instalación

Debido a las limitaciones de memoria en algunos dispositivos Android, compilaremos con opciones optimizadas para Termux:

```bash
# Compilar el binario (esto puede tardar unos minutos)
cargo build --release

# Instalar el binario en el sistema
cp target/release/vibe-ai $PREFIX/bin/
chmod +x $PREFIX/bin/vibe-ai
```

## 4. Configurar Autocompletado (Opcional)

Para que el comando `vibe-ai` tenga sugerencias al presionar la tecla Tab:

### Para Zsh:
```bash
mkdir -p $PREFIX/share/zsh/site-functions/
cp scripts/completions/vibe-ai.zsh $PREFIX/share/zsh/site-functions/_vibe-ai
```

### Para Fish:
```bash
mkdir -p $PREFIX/share/fish/vendor_completions.d/
cp scripts/completions/vibe-ai.fish $PREFIX/share/fish/vendor_completions.d/vibe-ai.fish
```

### Para Bash:
```bash
mkdir -p $PREFIX/share/bash-completion/completions/
cp scripts/completions/vibe-ai.bash $PREFIX/share/bash-completion/completions/vibe-ai
```

## 5. Primeros Pasos

Verifica que la instalación fue exitosa:

```bash
vibe-ai --info
```

### Configuración de IA
La primera vez que lo ejecutes, **vibe-ai** te guiará para configurar tu primera API Key (OpenAI, Anthropic, Gemini, etc.).

*   **Directorio de configuración:** `~/.config/vibe-ai/`
*   **Variables de entorno:** Todas comienzan con `VIBE_AI_` (ej. `VIBE_AI_CONFIG_DIR`).

---
¡Disfruta de tu asistente de IA personalizado en la palma de tu mano! 📱✨
