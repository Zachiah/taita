## VISION for Taita project manager
- [ ] Allows notes per project/branch/commit stored in your notes repo.
- [ ] Maybe allow notes in local repos on like `.taita` folder if repos want to track the notes.
- [ ] if so should seemlessly merge the 2 as like a checkbox so to speak.
- [ ] should have project pickers for wofi
- [ ] should have project pickers for nvim-telescope
- [ ] and other integrations in future
- [ ] configuration options for directory names and stuff
- [ ] projects should have
    - [ ] tags
    - [ ] name
    - [ ] description
    - [ ] etc
- [ ] finders should fuzzyfind over all relevant data
- [ ] should be able to save vim session files per commit/branch/project for later use
- [ ] maybe dedicated support for links and like opening them all at once

## Issues with currently implemented features
- [ ] Errors from alacritty will go into previous terminal
- [ ] Opening logic doesn't reuse tmux sessions and always opens a new terminal (see below)

## Rough sketch of the flow for opening a project in my mind's eye right now

### Get a terminal
- `if` in terminal `and` `not` `options['-n']`:
    - `if` in tmux session:
        - detach tmux session
    - `terminal` `=` the one we are in
- `if` in wofi `or` `options['-n']`:
    - `terminal` `=` open alacritty

### Join the session
- `session_id` `=` the unique identifier for the project/branch... we generate
- `if` tmux session exists for `session_id`:
    - attatch to session `session_id`
- `else`:
    - create tmux session with id `session_id`
    - set home dir from project details
    - set branch from project details
    - open nvim the notes file from the project dir

### Links part (maybe implement)
- open firefox
- open all links from project details
