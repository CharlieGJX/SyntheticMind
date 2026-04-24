#!/usr/bin/env bash
# Script de instalación inteligente para vibe-ai en Termux

set -e

# Configuración
REPO="CharlieGJX/SyntheticMind"
BINARY_NAME="vibe-ai"
PREFIX="/data/data/com.termux/files/usr"

echo "🔍 Detectando arquitectura..."
ARCH=$(uname -m)
case "$ARCH" in
    aarch64) ARCH_SUFFIX="aarch64" ;;
    armv7l|armv8l) ARCH_SUFFIX="arm" ;;
    x86_64) ARCH_SUFFIX="x86_64" ;;
    *) echo "❌ Arquitectura no soportada: $ARCH"; exit 1 ;;
esac

echo "🚀 Buscando la última versión de $BINARY_NAME en GitHub..."
LATEST_RELEASE_URL=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "browser_download_url" | grep "$ARCH_SUFFIX" | cut -d '"' -f 4)

if [ -z "$LATEST_RELEASE_URL" ]; then
    echo "❌ No se encontró un binario precompilado para tu arquitectura ($ARCH_SUFFIX)."
    exit 1
fi

echo "📥 Descargando $BINARY_NAME..."
curl -L "$LATEST_RELEASE_URL" -o "vibe-ai-package.tar.gz"

echo "📦 Extrayendo e instalando binario..."
mkdir -p tmp_install
tar -xzf "vibe-ai-package.tar.gz" -C tmp_install
mv tmp_install/vibe-ai "$PREFIX/bin/"
chmod +x "$PREFIX/bin/vibe-ai"

echo "⚙️ Configurando autocompletados..."

# Instalación para Zsh
if [ -d "$PREFIX/share/zsh/site-functions" ]; then
    cp tmp_install/completions/vibe-ai.zsh "$PREFIX/share/zsh/site-functions/_vibe-ai"
    echo "   ✅ Autocompletado Zsh instalado."
fi

# Instalación para Fish
if [ -d "$PREFIX/share/fish/vendor_completions.d" ]; then
    cp tmp_install/completions/vibe-ai.fish "$PREFIX/share/fish/vendor_completions.d/vibe-ai.fish"
    echo "   ✅ Autocompletado Fish instalado."
fi

# Instalación para Bash
if [ -d "$PREFIX/share/bash-completion/completions" ]; then
    cp tmp_install/completions/vibe-ai.bash "$PREFIX/share/bash-completion/completions/vibe-ai"
    echo "   ✅ Autocompletado Bash instalado."
fi

# Limpieza
rm -rf "vibe-ai-package.tar.gz" tmp_install

echo "🎉 ¡Instalación completada! Reinicia tu shell para activar los cambios."
echo "Escribe 'vibe-ai --info' para verificar."
