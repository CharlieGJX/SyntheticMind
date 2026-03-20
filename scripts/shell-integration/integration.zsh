_vibe-ai_zsh() {
    if [[ -n "$BUFFER" ]]; then
        local _old=$BUFFER
        BUFFER+="⌛"
        zle -I && zle redisplay
        BUFFER=$(vibe-ai -e "$_old")
        zle end-of-line
    fi
}
zle -N _vibe-ai_zsh
bindkey '\ee' _vibe-ai_zsh