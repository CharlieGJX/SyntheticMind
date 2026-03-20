function _vibe-ai_fish
    set -l _old (commandline)
    if test -n $_old
        echo -n "⌛"
        commandline -f repaint
        commandline (vibe-ai -e $_old)
    end
end
bind \ee _vibe-ai_fish