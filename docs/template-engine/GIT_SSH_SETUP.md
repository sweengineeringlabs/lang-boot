# Git SSH Authentication Setup Guide

**Audience**: Developers, Contributors, DevOps Engineers

Quick guide to set up SSH authentication for Git operations (push, pull, clone with SSH URLs).

## Quick Start

### Linux/macOS/WSL

```bash
# 1. Generate SSH key
ssh-keygen -t ed25519 -C "your_email@example.com"

# 2. Start SSH agent and add key
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519

# 3. Copy public key
cat ~/.ssh/id_ed25519.pub
# Copy the output

# 4. Add to GitHub: Settings > SSH and GPG keys > New SSH key
# Paste the public key

# 5. Test connection
ssh -T git@github.com
```

### Windows (PowerShell)

```powershell
# 1. Generate SSH key
ssh-keygen -t ed25519 -C "your_email@example.com"

# 2. Start SSH agent (as Administrator)
Get-Service ssh-agent | Set-Service -StartupType Automatic
Start-Service ssh-agent
ssh-add $env:USERPROFILE\.ssh\id_ed25519

# 3. Copy public key
Get-Content $env:USERPROFILE\.ssh\id_ed25519.pub | clip

# 4. Add to GitHub: Settings > SSH and GPG keys > New SSH key
# Paste from clipboard

# 5. Test connection
ssh -T git@github.com
```

## Detailed Setup (Linux/macOS/WSL)

### Step 1: Check for Existing SSH Keys

```bash
ls -al ~/.ssh
```

**If you see** `id_ed25519` or `id_rsa`:
- ✅ You already have SSH keys
- Skip to Step 2

**If directory is empty or missing**:
- ❌ No SSH keys found
- Proceed to generate new keys

### Step 2: Generate SSH Key (if needed)

```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
```

**Prompts and recommendations**:

1. **Enter file to save the key**: 
   - Press `Enter` to accept default location (`~/.ssh/id_ed25519`)
   
2. **Enter passphrase**: 
   - For automation/convenience: Press `Enter` twice (no passphrase)
   - For security: Enter a strong passphrase (you'll type it when using the key)

**Expected output**:
```
Generating public/private ed25519 key pair.
Your identification has been saved in /home/user/.ssh/id_ed25519
Your public key has been saved in /home/user/.ssh/id_ed25519.pub
The key fingerprint is:
SHA256:xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx your_email@example.com
```

**Alternative key types**:
- Modern (recommended): `ssh-keygen -t ed25519 -C "your_email@example.com"`
- Legacy compatibility: `ssh-keygen -t rsa -b 4096 -C "your_email@example.com"`

### Step 3: Add Key to SSH Agent

The SSH agent manages your keys and saves you from typing the passphrase repeatedly.

```bash
# Start SSH agent
eval "$(ssh-agent -s)"
```

**Expected output**: `Agent pid 12345`

```bash
# Add your SSH private key to the agent
ssh-add ~/.ssh/id_ed25519
```

**If you used RSA instead**: `ssh-add ~/.ssh/id_rsa`

**Expected output**: `Identity added: /home/user/.ssh/id_ed25519`

### Step 4: Copy Public Key

Display your public key:

```bash
cat ~/.ssh/id_ed25519.pub
```

**Copy the entire output**, which looks like:
```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx your_email@example.com
```

**Quick copy to clipboard**:
- **Linux (with xclip)**: `cat ~/.ssh/id_ed25519.pub | xclip -selection clipboard`
- **macOS**: `cat ~/.ssh/id_ed25519.pub | pbcopy`
- **WSL**: `cat ~/.ssh/id_ed25519.pub | clip.exe`

### Step 5: Add to GitHub

1. Go to **GitHub.com** and sign in
2. Click your profile picture → **Settings**
3. In the sidebar, click **SSH and GPG keys**
4. Click **New SSH key** (green button)
5. **Title**: Give it a descriptive name (e.g., "Work Laptop", "Home PC")
6. **Key**: Paste the public key you copied
7. Click **Add SSH key**
8. Confirm with your GitHub password if prompted

### Step 6: Test Connection

```bash
ssh -T git@github.com
```

**Expected output** (first time):
```
The authenticity of host 'github.com (140.82.121.4)' can't be established.
ED25519 key fingerprint is SHA256:xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.
Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
```
Type `yes` and press Enter.

**Successful output**:
```
Hi username! You've successfully authenticated, but GitHub does not provide shell access.
```

✅ **Success!** You're now authenticated with GitHub via SSH.

## Detailed Setup (Windows)

### Step 1: Check for Existing SSH Keys

```powershell
ls $env:USERPROFILE\.ssh
```

### Step 2: Generate SSH Key (if needed)

```powershell
ssh-keygen -t ed25519 -C "your_email@example.com"
```

Press `Enter` to accept defaults.

### Step 3: Start SSH Agent

**Open PowerShell as Administrator**:

```powershell
# Set SSH agent to start automatically
Get-Service ssh-agent | Set-Service -StartupType Automatic

# Start the SSH agent
Start-Service ssh-agent

# Verify it's running
Get-Service ssh-agent
```

**Add your key**:
```powershell
ssh-add $env:USERPROFILE\.ssh\id_ed25519
```

### Step 4: Copy Public Key

```powershell
# Display and copy to clipboard
Get-Content $env:USERPROFILE\.ssh\id_ed25519.pub | clip
```

The public key is now in your clipboard, ready to paste.

### Step 5 & 6: Add to GitHub and Test

Same as Linux/macOS steps above.

## GitLab, Bitbucket, and Other Platforms

### GitLab

1. Go to **GitLab.com** → **Preferences** → **SSH Keys**
2. Paste your public key
3. Test: `ssh -T git@gitlab.com`

**Expected**: `Welcome to GitLab, @username!`

### Bitbucket

1. Go to **Bitbucket.org** → **Personal settings** → **SSH keys**
2. Click **Add key**
3. Paste your public key
4. Test: `ssh -T git@bitbucket.org`

**Expected**: `authenticated via ssh key`

### Azure DevOps

1. Go to **User settings** (icon in top right) → **SSH public keys**
2. Click **+ New Key**
3. Paste your public key
4. Test: `ssh -T git@ssh.dev.azure.com`

## Using SSH with Git

### Clone with SSH

```bash
# Instead of HTTPS:
git clone https://github.com/user/repo.git

# Use SSH:
git clone git@github.com:user/repo.git
```

### Convert Existing Repo from HTTPS to SSH

```bash
# Check current remote URL
git remote -v

# Change to SSH
git remote set-url origin git@github.com:user/repo.git

# Verify
git remote -v
```

### Push/Pull Without Password

With SSH authentication set up:

```bash
# Push changes
git push origin main

# Pull changes  
git pull origin main
```

No username/password prompt! 🎉

## Troubleshooting

### Issue: "Permission denied (publickey)"

**Symptoms**:
```
git@github.com: Permission denied (publickey).
fatal: Could not read from remote repository.
```

**Solutions**:

1. **Verify SSH key is added to GitHub**
   - Check GitHub.com → Settings → SSH and GPG keys
   - Your key should be listed

2. **Verify SSH agent has the key**
   ```bash
   ssh-add -l
   ```
   Should show your key fingerprint

3. **Test SSH connection**
   ```bash
   ssh -T git@github.com
   ```

4. **Add key to agent if missing**
   ```bash
   ssh-add ~/.ssh/id_ed25519
   ```

5. **Check SSH config** (if using custom config)
   ```bash
   cat ~/.ssh/config
   ```

### Issue: "Could not open a connection to your authentication agent"

**Linux/macOS**:
```bash
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519
```

**Windows**:
```powershell
Start-Service ssh-agent
ssh-add $env:USERPROFILE\.ssh\id_ed25519
```

### Issue: SSH key not working after reboot

**Linux/macOS - Add to shell startup**:

Add to `~/.bashrc` or `~/.zshrc`:
```bash
# Start SSH agent if not running
if [ -z "$SSH_AUTH_SOCK" ]; then
   eval "$(ssh-agent -s)"
   ssh-add ~/.ssh/id_ed25519
fi
```

**Windows - Ensure ssh-agent starts automatically**:
```powershell
Get-Service ssh-agent | Set-Service -StartupType Automatic
```

### Issue: "Bad passphrase" or wrong key

```bash
# Remove all keys from agent
ssh-add -D

# Add the correct key
ssh-add ~/.ssh/id_ed25519

# List loaded keys to verify
ssh-add -l
```

### Issue: Multiple GitHub accounts

Create SSH config file (`~/.ssh/config`):

```
# Personal account
Host github.com
  HostName github.com
  User git
  IdentityFile ~/.ssh/id_ed25519_personal

# Work account
Host github-work
  HostName github.com
  User git
  IdentityFile ~/.ssh/id_ed25519_work
```

Clone with:
```bash
# Personal
git clone git@github.com:user/repo.git

# Work
git clone git@github-work:company/repo.git
```

## Best Practices

### ✅ DO:

- **Use ed25519 keys** - Modern, secure, faster
- **Add passphrase for security** - Especially on shared machines
- **Use descriptive key titles** - Know which key is which on GitHub
- **Generate new keys per device** - Don't copy private keys between machines
- **Keep private keys private** - Never commit or share `id_ed25519`
- **Use SSH agent** - Avoid typing passphrase repeatedly
- **Test after setup** - Verify connection works

### ❌ DON'T:

- **Don't share private keys** - Only share public keys (`.pub`)
- **Don't commit keys to Git** - Add `*.pem`, `id_*` to `.gitignore`
- **Don't use weak passphrases** - If using one, make it strong
- **Don't reuse keys** - Generate new key if compromised
- **Don't skip testing** - Always verify with `ssh -T`

## Security Notes

### Public vs Private Keys

- **Private key** (`id_ed25519`): Keep secret, never share
- **Public key** (`id_ed25519.pub`): Safe to share, add to GitHub

### Key Permissions

Ensure correct permissions:

```bash
chmod 700 ~/.ssh
chmod 600 ~/.ssh/id_ed25519
chmod 644 ~/.ssh/id_ed25519.pub
```

### Revoking Access

If a key is compromised:

1. **Remove from GitHub**: Settings → SSH keys → Delete
2. **Generate new key**: Follow steps 1-6 again
3. **Update all repos**: Change remote URLs if needed

## Quick Reference

### Common Commands

```bash
# Generate new key
ssh-keygen -t ed25519 -C "email@example.com"

# Start SSH agent
eval "$(ssh-agent -s)"

# Add key to agent
ssh-add ~/.ssh/id_ed25519

# List loaded keys
ssh-add -l

# Display public key
cat ~/.ssh/id_ed25519.pub

# Test GitHub connection
ssh -T git@github.com

# Remove all keys from agent
ssh-add -D

# Change remote to SSH
git remote set-url origin git@github.com:user/repo.git
```

### Key Locations

- **Linux/macOS**: `~/.ssh/`
- **Windows**: `C:\Users\YourName\.ssh\`

### File Names

- **Private key**: `id_ed25519` or `id_rsa`
- **Public key**: `id_ed25519.pub` or `id_rsa.pub`

## Related Documentation

- [GitHub SSH Documentation](https://docs.github.com/en/authentication/connecting-to-github-with-ssh)
- [GitLab SSH Documentation](https://docs.gitlab.com/ee/user/ssh.html)
- [SSH Keygen Man Page](https://man.openbsd.org/ssh-keygen)

---

**Last Updated**: 2025-12-22  
**Maintained By**: SEA Documentation Template Engine Team

