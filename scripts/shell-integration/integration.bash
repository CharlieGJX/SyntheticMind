_vibe-ai_bash() {
    if [[ -n "$READLINE_LINE" ]]; then
        READLINE_LINE=$(vibe-ai -e "$READLINE_LINE")
        READLINE_POINT=${#READLINE_LINE}
    fi
}
bind -x '"\ee": _vibe-ai_bash'