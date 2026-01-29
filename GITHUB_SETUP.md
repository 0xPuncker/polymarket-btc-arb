# GitHub Setup via Telegram

## Option 1: Personal Access Token (Recommended)

Since GitHub CLI authentication typically requires a browser, the easiest way via Telegram is to use a Personal Access Token.

### Steps:

1. **Create a Personal Access Token:**
   - Go to: https://github.com/settings/tokens
   - Click "Generate new token (classic)"
   - Give it a name like "Clawdbot-Polymarket"
   - Select scopes: `repo`, `read:org`, `gist`
   - Click "Generate token"
   - **Copy the token** (you won't see it again)

2. **Share the token with me:**
   - Send it in a message like: `GH_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxxx`

3. **I'll configure and push:**
   ```bash
   export GH_TOKEN="your-token-here"
   gh auth status
   gh repo create polymarket-btc-arb --public --source=. --push
   ```

## Option 2: OAuth Device Flow (Interactive)

If you prefer OAuth:

1. Run this command on your local machine:
   ```bash
   gh auth login --web
   ```

2. Follow the browser prompts

3. Share your GitHub username with me, and I'll set up the repository using your account via environment variables

## Option 3: Manual GitHub Setup

1. Go to https://github.com/new
2. Repository name: `polymarket-btc-arb`
3. Public repository
4. Click "Create repository"

5. Then share the repository URL with me, and I'll add it as remote and push

---

**Recommendation:** Option 1 is simplest. Just generate a token and share it in Telegram.

Note: The token will only be stored in the environment for the current session and won't persist.
