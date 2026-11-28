#!/usr/bin/env nu
# Forward the dice server of exercise 5 from the university server, so that
# `localhost:56701` here is the `wuerfel_server` over there.
#
# As a module:
#   use scripts/tunnel.nu
#   tunnel start <user>@<host>
#   tunnel status
#   tunnel stop
# or as a script, `nu scripts/tunnel.nu start <user>@<host>`. The host can
# also come from `$env.DICE_HOST`.
#
# ssh puts itself in the background once it is logged in, so the tunnel
# outlives the shell that started it. It is reached again through a control
# socket, which is how `stop` ends this ssh and no other.

const default_port = 56701

# Control socket of the tunnel on `port`.
def socket [port: int]: nothing -> path {
    $env.XDG_RUNTIME_DIR? | default $nu.temp-dir | path join $"dice-tunnel-($port).sock"
}

# Whether the ssh behind the control socket still answers. The destination is
# required by ssh but unused when it only talks to the socket.
def is-up [port: int]: nothing -> bool {
    (^ssh -S (socket $port) -O check tunnel | complete).exit_code == 0
}

# Start forwarding the port in the background.
export def start [
    host?: string # ssh destination, as in `ssh <host>`, else `$env.DICE_HOST`
    --port (-p): int = $default_port # port of the dice server, used on both ends
]: nothing -> nothing {
    if (is-up $port) {
        print $"already forwarding localhost:($port)"
        return
    }
    let host = $host | default $env.DICE_HOST?
    if $host == null {
        error make {msg: "no host given, pass one or set $env.DICE_HOST"}
    }
    # A socket left behind by a tunnel that died makes ssh skip `-M`.
    rm -f (socket $port)
    # Without ExitOnForwardFailure ssh stays up when the port is already taken.
    ^ssh -f -N -M -S (socket $port) -o ExitOnForwardFailure=yes -L $"($port):localhost:($port)" $host
    print $"forwarding localhost:($port) to ($host), end it with `tunnel stop`"
}

# Stop forwarding the port.
export def stop [
    --port (-p): int = $default_port # port the tunnel was started on
]: nothing -> nothing {
    if not (is-up $port) {
        print $"no tunnel on localhost:($port)"
        return
    }
    ^ssh -S (socket $port) -O exit tunnel | complete | ignore
    print $"stopped forwarding localhost:($port)"
}

# Say whether the port is being forwarded.
export def status [
    --port (-p): int = $default_port # port the tunnel was started on
]: nothing -> nothing {
    print $"localhost:($port) is (if (is-up $port) { 'forwarded' } else { 'not forwarded' })"
}

# The same three commands for `nu scripts/tunnel.nu <command>`.
def "main start" [host?: string, --port (-p): int = $default_port] { start $host --port $port }
def "main stop" [--port (-p): int = $default_port] { stop --port $port }
def "main status" [--port (-p): int = $default_port] { status --port $port }
def main [] { print "usage: nu scripts/tunnel.nu <start [host] | stop | status>" }
