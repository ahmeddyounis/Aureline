// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Shared fail-closed subprocess posture for Git review and apply lanes.

use std::ffi::OsStr;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const MAX_CAPTURED_STREAM_BYTES: usize = 16 * 1024 * 1024;
const MAX_STDIN_BYTES: usize = 16 * 1024 * 1024;
const GIT_COMMAND_TIMEOUT: Duration = Duration::from_secs(60);
const SUPERVISOR_POLL_INTERVAL: Duration = Duration::from_millis(5);

#[derive(Debug)]
pub(crate) struct HardenedGitOutput {
    pub(crate) status: ExitStatus,
    pub(crate) stdout: Vec<u8>,
    pub(crate) stderr: Vec<u8>,
}

/// Single transport family admitted by a reviewed publish preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishTransportPosture {
    /// Absolute/relative local paths and `file://` URLs.
    File,
    /// HTTPS without ambient credential helpers.
    Https,
    /// SSH through the exact reviewed agent socket.
    Ssh,
}

/// Builds a local-only Git command without ambient config, authentication,
/// network transport, prompt, pager, hook, monitor, credential-helper,
/// submodule, or external-diff execution authority.
pub(crate) fn command(git_binary: &Path, root: &Path, args: &[String]) -> Command {
    command_with_transport_posture(git_binary, root, args, None)
}

fn command_with_transport_posture(
    git_binary: &Path,
    root: &Path,
    args: &[String],
    publish_transport: Option<PublishTransportPosture>,
) -> Command {
    let mut command = Command::new(git_binary);
    command.env_clear();
    if let Some(path) = std::env::var_os("PATH") {
        command.env("PATH", path);
    }
    #[cfg(windows)]
    {
        for key in ["SystemRoot", "WINDIR", "PATHEXT", "TEMP", "TMP"] {
            if let Some(value) = std::env::var_os(key) {
                command.env(key, value);
            }
        }
    }
    command
        .env("LC_ALL", "C")
        .env("LANG", "C")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", null_device_path())
        .env("GIT_ATTR_NOSYSTEM", "1")
        .env("GIT_LITERAL_PATHSPECS", "1")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env("GIT_PAGER", "")
        .env("GIT_EDITOR", "false")
        .env("GIT_SEQUENCE_EDITOR", "false")
        .env("GIT_MERGE_AUTOEDIT", "no")
        .env("GIT_ASKPASS", "")
        .env("SSH_ASKPASS", "")
        .env("SSH_ASKPASS_REQUIRE", "never")
        .arg("-c")
        .arg("core.fsmonitor=false")
        .arg("-c")
        .arg("core.untrackedCache=false")
        .arg("-c")
        .arg("submodule.recurse=false")
        .arg("-c")
        .arg(format!("core.attributesFile={}", null_device_path()))
        .arg("-c")
        .arg(format!("core.hooksPath={}", null_device_path()))
        .arg("-c")
        .arg("diff.external=")
        .arg("-c")
        .arg("diff.trustExitCode=false")
        .arg("-c")
        .arg("credential.helper=")
        .arg("-c")
        .arg("credential.interactive=never")
        .arg("-c")
        .arg("commit.gpgSign=false")
        .arg("-c")
        .arg("tag.gpgSign=false")
        .arg("-c")
        .arg("protocol.allow=never")
        .arg("-c")
        .arg(
            if publish_transport == Some(PublishTransportPosture::File) {
                "protocol.file.allow=always"
            } else {
                "protocol.file.allow=never"
            },
        )
        .arg("-c")
        .arg(
            if publish_transport == Some(PublishTransportPosture::Https) {
                "protocol.https.allow=always"
            } else {
                "protocol.https.allow=never"
            },
        )
        .arg("-c")
        .arg(if publish_transport == Some(PublishTransportPosture::Ssh) {
            "protocol.ssh.allow=always"
        } else {
            "protocol.ssh.allow=never"
        })
        .arg("-c")
        .arg("protocol.ext.allow=never")
        .arg("-C")
        .arg(root)
        .args(args);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    command
}

/// Builds the reviewed publish posture. This is the only runner that admits
/// file, HTTPS, or SSH transports and the only one allowed to inherit a
/// reviewed SSH agent socket. Callers pass the exact socket retained by an
/// admitted SSH preview; this function never reads ambient auth itself.
pub(crate) fn command_for_publish(
    git_binary: &Path,
    root: &Path,
    args: &[String],
    transport: PublishTransportPosture,
    ssh_auth_sock: Option<&OsStr>,
) -> Command {
    let mut command = command_with_transport_posture(git_binary, root, args, Some(transport));
    if let Some(ssh_auth_sock) = ssh_auth_sock {
        command.env("SSH_AUTH_SOCK", ssh_auth_sock);
    }
    command
        .env("GIT_SSH_VARIANT", "ssh")
        .env("GIT_SSH_COMMAND", hardened_ssh_command());
    command
}

/// Runs a configured command while draining both output streams and retaining
/// at most the published local evidence bound. Oversized output fails closed.
pub(crate) fn run(mut command: Command) -> io::Result<HardenedGitOutput> {
    run_with_limits(&mut command, MAX_CAPTURED_STREAM_BYTES, GIT_COMMAND_TIMEOUT)
}

/// Runs a configured command with a bounded, supervised stdin body.
///
/// The writer is supervised alongside the process and both output readers, so
/// a child that stops reading stdin cannot pin the caller indefinitely. Input,
/// stdout, and stderr each fail closed at the published evidence bound.
pub(crate) fn run_with_stdin(mut command: Command, stdin: &[u8]) -> io::Result<HardenedGitOutput> {
    run_with_input_limits(
        &mut command,
        stdin,
        MAX_STDIN_BYTES,
        MAX_CAPTURED_STREAM_BYTES,
        GIT_COMMAND_TIMEOUT,
    )
}

fn run_with_limits(
    command: &mut Command,
    max_stream_bytes: usize,
    timeout: Duration,
) -> io::Result<HardenedGitOutput> {
    supervise(command, None, max_stream_bytes, timeout)
}

fn run_with_input_limits(
    command: &mut Command,
    stdin: &[u8],
    max_stdin_bytes: usize,
    max_stream_bytes: usize,
    timeout: Duration,
) -> io::Result<HardenedGitOutput> {
    if stdin.len() > max_stdin_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Git input exceeded the safe transfer limit",
        ));
    }
    supervise(command, Some(stdin.to_vec()), max_stream_bytes, timeout)
}

fn supervise(
    command: &mut Command,
    stdin: Option<Vec<u8>>,
    max_stream_bytes: usize,
    timeout: Duration,
) -> io::Result<HardenedGitOutput> {
    let mut child = command
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let Some(stdout) = child.stdout.take() else {
        terminate_child_tree(&mut child);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Git stdout unavailable",
        ));
    };
    let Some(stderr) = child.stderr.take() else {
        terminate_child_tree(&mut child);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Git stderr unavailable",
        ));
    };
    let (sender, receiver) = mpsc::channel::<SupervisorEvent>();
    let stdout_sender = sender.clone();
    let _stdout_reader = thread::spawn(move || {
        let _ = stdout_sender.send(SupervisorEvent::Stream(
            StreamKind::Stdout,
            drain_bounded(stdout, max_stream_bytes),
        ));
    });
    let stderr_sender = sender.clone();
    let _stderr_reader = thread::spawn(move || {
        let _ = stderr_sender.send(SupervisorEvent::Stream(
            StreamKind::Stderr,
            drain_bounded(stderr, max_stream_bytes),
        ));
    });
    let mut stdin_complete = stdin.is_none();
    if let Some(stdin) = stdin {
        let Some(mut child_stdin) = child.stdin.take() else {
            terminate_child_tree(&mut child);
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Git stdin unavailable",
            ));
        };
        let stdin_sender = sender.clone();
        let _stdin_writer = thread::spawn(move || {
            let result = child_stdin
                .write_all(&stdin)
                .and_then(|()| child_stdin.flush());
            drop(child_stdin);
            let _ = stdin_sender.send(SupervisorEvent::Stdin(result));
        });
    }
    drop(sender);

    let started_at = Instant::now();
    let mut status = None;
    let mut stdout = None;
    let mut stderr = None;
    loop {
        while let Ok(event) = receiver.try_recv() {
            match event {
                SupervisorEvent::Stream(stream, result) => {
                    let bytes = match result {
                        Ok(bytes) => bytes,
                        Err(error) => {
                            terminate_child_tree(&mut child);
                            return Err(error);
                        }
                    };
                    match stream {
                        StreamKind::Stdout => stdout = Some(bytes),
                        StreamKind::Stderr => stderr = Some(bytes),
                    }
                }
                SupervisorEvent::Stdin(result) => match result {
                    Ok(()) => stdin_complete = true,
                    Err(error) => {
                        terminate_child_tree(&mut child);
                        return Err(error);
                    }
                },
            }
        }
        if status.is_none() {
            status = match child.try_wait() {
                Ok(status) => status,
                Err(error) => {
                    terminate_child_tree(&mut child);
                    return Err(error);
                }
            };
        }
        match (status.take(), stdout.take(), stderr.take(), stdin_complete) {
            (Some(status), Some(stdout), Some(stderr), true) => {
                return Ok(HardenedGitOutput {
                    status,
                    stdout,
                    stderr,
                });
            }
            (pending_status, pending_stdout, pending_stderr, pending_stdin) => {
                status = pending_status;
                stdout = pending_stdout;
                stderr = pending_stderr;
                stdin_complete = pending_stdin;
            }
        }
        if started_at.elapsed() >= timeout {
            terminate_child_tree(&mut child);
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "Git command exceeded the safe execution deadline",
            ));
        }
        thread::sleep(SUPERVISOR_POLL_INTERVAL);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StreamKind {
    Stdout,
    Stderr,
}

#[derive(Debug)]
enum SupervisorEvent {
    Stream(StreamKind, io::Result<Vec<u8>>),
    Stdin(io::Result<()>),
}

fn drain_bounded(mut stream: impl Read, max_bytes: usize) -> io::Result<Vec<u8>> {
    let mut retained = Vec::new();
    let mut buffer = [0_u8; 8192];
    loop {
        let read = stream.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        let Some(length) = retained.len().checked_add(read) else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Git output exceeded the safe capture limit",
            ));
        };
        if length > max_bytes {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Git output exceeded the safe capture limit",
            ));
        }
        retained.extend_from_slice(&buffer[..read]);
    }
    Ok(retained)
}

fn terminate_child_tree(child: &mut std::process::Child) {
    #[cfg(unix)]
    {
        let process_group = format!("-{}", child.id());
        let _ = Command::new("/bin/kill")
            .env_clear()
            .args(["-KILL", "--", process_group.as_str()])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    #[cfg(windows)]
    {
        let process_id = child.id().to_string();
        let _ = Command::new("taskkill")
            .env_clear()
            .args(["/F", "/T", "/PID", process_id.as_str()])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let _ = child.kill();
    let _ = child.wait();
}

pub(crate) const fn null_device_path() -> &'static str {
    if cfg!(windows) {
        "NUL"
    } else {
        "/dev/null"
    }
}

fn hardened_ssh_command() -> String {
    format!(
        "ssh -F {} -oBatchMode=yes -oClearAllForwardings=yes -oPermitLocalCommand=no \
         -oRequestTTY=no -oIdentityFile=none -oIdentitiesOnly=no \
         -oPasswordAuthentication=no -oKbdInteractiveAuthentication=no \
         -oGSSAPIAuthentication=no -oHostbasedAuthentication=no \
         -oStrictHostKeyChecking=yes -oUpdateHostKeys=no -oNumberOfPasswordPrompts=0",
        null_device_path()
    )
}

#[cfg(all(test, unix))]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    use super::*;

    #[test]
    fn subprocess_environment_and_git_execution_extensions_are_closed() {
        const CHILD_MARKER: &str = "AURELINE_HARDENED_GIT_ENV_TEST_CHILD";
        const TEST_SOCKET: &str = "/tmp/aureline-test-agent.sock";

        if std::env::var_os(CHILD_MARKER).is_none() {
            let output = Command::new(std::env::current_exe().expect("current test executable"))
                .args([
                    "--exact",
                    "hardened_git::tests::subprocess_environment_and_git_execution_extensions_are_closed",
                    "--nocapture",
                ])
                .env(CHILD_MARKER, "1")
                .env("SSH_AUTH_SOCK", TEST_SOCKET)
                .env("GIT_SSH_COMMAND", "hostile-ssh-override")
                .env("AURELINE_SECRET_SENTINEL", "must-not-leak")
                .output()
                .expect("launch isolated environment test");
            assert!(
                output.status.success(),
                "isolated environment test failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }

        let temp = tempfile::tempdir().expect("tempdir");
        let probe = temp.path().join("git-probe");
        fs::write(
            &probe,
            "#!/bin/sh\nprintf '%s\\n' \"${HOME-unset}\" \"${GIT_CONFIG_GLOBAL-unset}\" \"${SSH_AUTH_SOCK-unset}\" \"${GIT_TERMINAL_PROMPT-unset}\" \"${GIT_ASKPASS-unset}\" \"${GIT_SSH_COMMAND-unset}\" \"${AURELINE_SECRET_SENTINEL-unset}\" \"$@\"\n",
        )
        .expect("write probe");
        let mut permissions = fs::metadata(&probe).expect("probe metadata").permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&probe, permissions).expect("make probe executable");

        let local_output =
            run(command(&probe, temp.path(), &["status".to_string()])).expect("local probe runs");
        assert!(local_output.status.success());
        let local_text = String::from_utf8(local_output.stdout).expect("utf8 local probe output");
        let local_environment = local_text.lines().take(7).collect::<Vec<_>>();
        assert_eq!(
            local_environment,
            vec![
                "unset",
                null_device_path(),
                "unset",
                "0",
                "",
                "unset",
                "unset"
            ]
        );
        for denied_protocol in [
            "protocol.file.allow=never",
            "protocol.https.allow=never",
            "protocol.ssh.allow=never",
            "protocol.ext.allow=never",
        ] {
            assert!(
                local_text.contains(denied_protocol),
                "local Git command admitted transport: {denied_protocol}"
            );
        }

        let publish_output = run(command_for_publish(
            &probe,
            temp.path(),
            &["push".to_string()],
            PublishTransportPosture::Ssh,
            Some(OsStr::new(TEST_SOCKET)),
        ))
        .expect("publish probe runs");
        assert!(publish_output.status.success());
        let publish_text =
            String::from_utf8(publish_output.stdout).expect("utf8 publish probe output");
        let publish_environment = publish_text.lines().take(7).collect::<Vec<_>>();
        assert_eq!(publish_environment[0], "unset");
        assert_eq!(publish_environment[1], null_device_path());
        assert_eq!(publish_environment[2], TEST_SOCKET);
        assert_eq!(publish_environment[3], "0");
        assert_eq!(publish_environment[4], "");
        assert_eq!(publish_environment[5], hardened_ssh_command());
        assert_eq!(publish_environment[6], "unset");
        assert!(publish_text.contains(null_device_path()));
        for required in [
            "core.fsmonitor=false",
            "submodule.recurse=false",
            "core.hooksPath=",
            "diff.external=",
            "credential.helper=",
            "credential.interactive=never",
            "protocol.allow=never",
            "protocol.file.allow=never",
            "protocol.https.allow=never",
            "protocol.ssh.allow=always",
            "protocol.ext.allow=never",
        ] {
            assert!(
                publish_text.contains(required),
                "missing hardening arg: {required}"
            );
        }
        for required in [
            "-oBatchMode=yes",
            "-oIdentityFile=none",
            "-oPasswordAuthentication=no",
            "-oStrictHostKeyChecking=yes",
        ] {
            assert!(publish_environment[5].contains(required));
        }

        for (transport, admitted, denied) in [
            (
                PublishTransportPosture::Https,
                "protocol.https.allow=always",
                ["protocol.file.allow=never", "protocol.ssh.allow=never"],
            ),
            (
                PublishTransportPosture::File,
                "protocol.file.allow=always",
                ["protocol.https.allow=never", "protocol.ssh.allow=never"],
            ),
        ] {
            let output = run(command_for_publish(
                &probe,
                temp.path(),
                &["push".to_string()],
                transport,
                None,
            ))
            .expect("non-SSH publish probe runs");
            let text = String::from_utf8(output.stdout).expect("utf8 publish probe output");
            assert!(text.contains(admitted));
            assert!(denied.iter().all(|setting| text.contains(setting)));
            assert_eq!(text.lines().nth(2), Some("unset"));
        }
    }

    #[test]
    fn subprocess_stdin_output_and_time_are_bounded() {
        let temp = tempfile::tempdir().expect("tempdir");
        let output_probe = temp.path().join("output-probe");
        fs::write(
            &output_probe,
            "#!/bin/sh\nwhile :; do printf '0123456789abcdef'; done\n",
        )
        .expect("write output probe");
        let timeout_probe = temp.path().join("timeout-probe");
        fs::write(&timeout_probe, "#!/bin/sh\nsleep 30\n").expect("write timeout probe");
        let stdin_probe = temp.path().join("stdin-probe");
        fs::write(
            &stdin_probe,
            "#!/bin/sh\nif IFS= read -r value; then printf 'stdin-open'; else printf 'stdin-closed'; fi\n",
        )
        .expect("write stdin probe");
        let input_probe = temp.path().join("input-probe");
        fs::write(&input_probe, "#!/bin/sh\nwc -c | tr -d ' '\n").expect("write input probe");
        for probe in [&output_probe, &timeout_probe, &stdin_probe, &input_probe] {
            let mut permissions = fs::metadata(probe).expect("probe metadata").permissions();
            permissions.set_mode(0o700);
            fs::set_permissions(probe, permissions).expect("make probe executable");
        }

        let started_at = Instant::now();
        let output_error = run_with_limits(
            &mut command(&output_probe, temp.path(), &[]),
            1024,
            Duration::from_secs(5),
        )
        .expect_err("oversized output fails closed");
        assert_eq!(output_error.kind(), io::ErrorKind::InvalidData);
        assert!(started_at.elapsed() < Duration::from_secs(2));

        let started_at = Instant::now();
        let timeout_error = run_with_limits(
            &mut command(&timeout_probe, temp.path(), &[]),
            1024,
            Duration::from_millis(100),
        )
        .expect_err("deadline fails closed");
        assert_eq!(timeout_error.kind(), io::ErrorKind::TimedOut);
        assert!(started_at.elapsed() < Duration::from_secs(2));

        let stdin_output = run_with_limits(
            &mut command(&stdin_probe, temp.path(), &[]),
            1024,
            Duration::from_secs(2),
        )
        .expect("stdin probe runs");
        assert_eq!(stdin_output.stdout, b"stdin-closed");

        let input_output = run_with_input_limits(
            &mut command(&input_probe, temp.path(), &[]),
            b"reviewed patch bytes",
            1024,
            1024,
            Duration::from_secs(2),
        )
        .expect("bounded stdin is delivered");
        assert_eq!(input_output.stdout, b"20\n");

        let oversized_input = run_with_input_limits(
            &mut command(&input_probe, temp.path(), &[]),
            &[0_u8; 1025],
            1024,
            1024,
            Duration::from_secs(2),
        )
        .expect_err("oversized stdin fails before launch");
        assert_eq!(oversized_input.kind(), io::ErrorKind::InvalidInput);

        let started_at = Instant::now();
        let blocked_writer = run_with_input_limits(
            &mut command(&timeout_probe, temp.path(), &[]),
            &[0_u8; 1024 * 1024],
            1024 * 1024,
            1024,
            Duration::from_millis(100),
        )
        .expect_err("a child that does not read stdin is terminated");
        assert_eq!(blocked_writer.kind(), io::ErrorKind::TimedOut);
        assert!(started_at.elapsed() < Duration::from_secs(2));
    }
}
