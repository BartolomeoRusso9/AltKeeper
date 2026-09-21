# Security

## Reporting a vulnerability

Please **do not open a public issue** for a security problem. Use GitHub's private reporting:
the **Security** tab of this repository, then **Report a vulnerability**
(https://github.com/BartolomeoRusso9/altkeeper/security/advisories/new).

Say what you found, how to reproduce it and what an attacker gains. Never include your Apple ID
password, `account.json`, the pairing file (`rp-pairing.plist`) or the ntfy topic in a report.
I will answer as soon as I can; this is a personal project, so there is no fixed deadline.

## What is in scope

- The web page (`serve`): authentication, CSRF, injection, reading files it should not.
- The AltServer listener: a malformed request crashing or hanging the process, or reading or
  writing outside the working directory.
- Secrets ending up in a log, in the Docker image or in a file with loose permissions.

## Known and documented, not vulnerabilities

- **The AltServer port (49500) has no login**, exactly like the real AltServer. Anyone on your
  network can talk to it. Keep it inside your home network and never forward it on the router.
- `account.json` can hold the Apple ID password **in clear text** if you choose to save it
  (mode `600`). Protect the machine that stores it.
- Without TLS the web page's PIN travels in clear text on your network. Put it behind a reverse
  proxy with HTTPS if that matters to you, or leave it on `127.0.0.1`.
- The project relies on private Apple APIs and on AltStore's protocol; Apple can change them.

## Supported versions

Only the latest release gets fixes.
