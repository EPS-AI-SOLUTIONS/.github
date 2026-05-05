# EPS-AI-SOLUTIONS

Organization for the Jaskier ecosystem — atomic polyrepo migration of the JaskierWorkspace monorepo (B1236).

## Repository Structure

- **Apps** (35): Hydras, Tissaia, FitPlanner67, Grantify, Megaskop, JaskierLauncher, ...
- **Crates** (42): `jaskier-*` published to private Kellnr registry
- **Packages** (18): `@eps-ai-solutions/*` published to GitHub Packages
- **Services** (17): jsp-fusion, kukielkarz, goniec, jaskier-guardian, ...

## NPM Package Policy

- Scope: `@eps-ai-solutions/*` exclusively
- Registry: GitHub Packages (`https://npm.pkg.github.com`)
- Visibility: PRIVATE by default
- 2FA: WebAuthn (FIDO) MANDATORY for all owners
- Publishing: OIDC trusted publishing only
