## VISION for Taita project manager
- [ ] Allows notes per project/branch/commit stored in your notes repo.
- [ ] Maybe allow notes in local repos on like `.tita` folder if repos want to track the notes.
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

## Rough sketch of opening a project flow in my mind's eye right now
```
# Get a terminal
if "in terminal" and not options['-n']:
    if "in tmux session":
        "detach tmux session"

    terminal = "this one"
if "in wofi" or options['-n']:
    terminal = "open alacritty"

# Join the session
session_id = "the unique identifier for the project/branch... we generate"
if tmux_session_exists(session_id):
    attatch_to_session(session_id)
else:
    create_tmux_session_with_id(session_id)
    set_home_dir() # from project details
    set_branch() # from project details

# Links part (maybe implement)
open_firefox()
open_all_links() # from project details
```
