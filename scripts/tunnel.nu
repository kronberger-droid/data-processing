# Forward the dice server of exercise 5 from the university server, so that
# `localhost:56701` here is the `wuerfel_server` over there.
#
# This is a module, not a script: a job spawned by `nu scripts/tunnel.nu`
# would die with that process. Load it into the interactive shell instead:
#   use scripts/tunnel.nu
#   tunnel <user>@<host>
# Stop a background tunnel with `job kill <id>`.

# Open the tunnel, in the foreground or as a background job.
export def main [
    host: string # ssh destination, as in `ssh <host>`
    --port (-p): int = 56701 # port of the dice server, used on both ends
]: nothing -> any {
    let forward = $"($port):localhost:($port)"
    # Without this ssh stays up even when the local port is already taken.
    let args = [-N -o ExitOnForwardFailure=yes -L $forward $host]

    let answer = input "run the tunnel in the background with job spawn? [y/N] "
    if not ($answer | str trim | str lowercase | str starts-with y) {
        print $"forwarding localhost:($port), ctrl-c to stop"
        ^ssh ...$args
        return
    }

    # A background ssh cannot ask for a password without fighting the prompt
    # for the terminal, so it has to get in by key or agent, or fail.
    # The job's output never reaches the terminal, so when ssh exits the job
    # mails what it said to the shell, tagged with its own id.
    let id = job spawn --description $"tunnel ($forward)" {
        let ssh = ^ssh -o BatchMode=yes ...$args | complete
        $ssh.stderr | str trim | job send 0 --tag (job id)
    }
    # Mail within the grace period means ssh is already gone again.
    let died = try { job recv --tag $id --timeout 2sec } catch { null }
    if $died != null {
        error make {msg: $"the tunnel to ($host) exited right away: ($died)"}
    }
    print $"forwarding localhost:($port) as job ($id), stop it with `job kill ($id)`"
    $id
}
