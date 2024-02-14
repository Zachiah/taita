# Taita

This is a project manager I am building for myself. I would like to make it useful for others as well in the long term, but for now the MVP is going to be hyperspecific to my usecase. [My vision for the project](./VISION.md)

## Usage (pre-alpha but still notes)

Taita keeps all its information in a directory `~/.taita`. You should make this a git repository and sync it between your machines if you want to sync your project information and notes.

### Adding projects

Run:
```
$ taita add \
    --name name_of_project \
    --dir name_of_directory \
    --tags "a, list, of, tags, that, describe, the, project"
```

for more info see:
```
$ taita help add
```

please note that all this command does is edit the file `~/.taita/projects.json`. Feel free to edit this file manually if you so desire

### With wofi
```
taita ls -p | wofi --show dmenu | xargs -I "{}" taita open -p "{}"
```
