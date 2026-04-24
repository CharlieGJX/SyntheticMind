complete -c vibe-ai -s m -l model -x -a "(vibe-ai --list-models)" -d 'Select a LLM model' -r
complete -c vibe-ai -l prompt -d 'Use the system prompt'
complete -c vibe-ai -s r -l role -x -a "(vibe-ai --list-roles)" -d 'Select a role' -r
complete -c vibe-ai -s s -l session -x  -a "(vibe-ai --list-sessions)" -d 'Start or join a session' -r
complete -c vibe-ai -l empty-session -d 'Ensure the session is empty'
complete -c vibe-ai -l save-session -d 'Ensure the new conversation is saved to the session'
complete -c vibe-ai -s a -l agent -x  -a "(vibe-ai --list-agents)" -d 'Start a agent' -r
complete -c vibe-ai -l agent-variable -d 'Set agent variables'
complete -c vibe-ai -l rag -x  -a"(vibe-ai --list-rags)" -d 'Start a RAG' -r
complete -c vibe-ai -l rebuild-rag -d 'Rebuild the RAG to sync document changes'
complete -c vibe-ai -l macro -x  -a"(vibe-ai --list-macros)" -d 'Execute a macro' -r
complete -c vibe-ai -l serve -d 'Serve the LLM API and WebAPP'
complete -c vibe-ai -s e -l execute -d 'Execute commands in natural language'
complete -c vibe-ai -s c -l code -d 'Output code only'
complete -c vibe-ai -s f -l file -d 'Include files, directories, or URLs' -r -F
complete -c vibe-ai -s S -l no-stream -d 'Turn off stream mode'
complete -c vibe-ai -l dry-run -d 'Display the message without sending it'
complete -c vibe-ai -l info -d 'Display information'
complete -c vibe-ai -l sync-models -d 'Sync models updates'
complete -c vibe-ai -l list-models -d 'List all available chat models'
complete -c vibe-ai -l list-roles -d 'List all roles'
complete -c vibe-ai -l list-sessions -d 'List all sessions'
complete -c vibe-ai -l list-agents -d 'List all agents'
complete -c vibe-ai -l list-rags -d 'List all RAGs'
complete -c vibe-ai -l list-macros -d 'List all macros'
complete -c vibe-ai -s h -l help -d 'Print help'
complete -c vibe-ai -s V -l version -d 'Print version'
