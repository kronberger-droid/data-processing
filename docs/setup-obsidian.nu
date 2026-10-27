#!/usr/bin/env nu

# Fill docs/.obsidian with the plugin code and theme from the main vault.
#
# Plugin settings (data.json) and the vault config are tracked in git. The
# plugin code is not: typst-mate alone ships a 35 MB wasm, and Obsidian never
# downloads community plugins on its own, so they get copied over from here.
def main [
    --vault: string = "~/Documents/notes/general-vault"  # vault to copy from
] {
    let src = ($vault | path expand | path join ".obsidian")
    let dst = ($env.FILE_PWD | path join ".obsidian")

    for plugin in (open ($dst | path join "community-plugins.json")) {
        let to = ($dst | path join "plugins" $plugin)
        mkdir $to
        ls ($src | path join "plugins" $plugin)
        | where {|f| ($f.name | path basename) != "data.json" }
        | each {|f| cp -r $f.name $to }
        | ignore
    }

    mkdir ($dst | path join "themes")
    cp -r ($src | path join "themes" "Minimal") ($dst | path join "themes")
}
