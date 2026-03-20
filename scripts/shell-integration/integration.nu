def _vibe-ai_nushell [] {
    let _prev = (commandline)
    if ($_prev != "") {
        print '⌛'
        commandline edit -r (vibe-ai -e $_prev)
    }
}

$env.config.keybindings = ($env.config.keybindings | append {
        name: vibe-ai_integration
        modifier: alt
        keycode: char_e
        mode: [emacs, vi_insert]
        event:[
            {
                send: executehostcommand,
                cmd: "_vibe-ai_nushell"
            }
        ]
    }
)