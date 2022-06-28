#!/bin/sh

# Decrypt the file
mkdir -p ~/.config/dfx/identity/admin/
# --batch to prevent interactive command
# --yes to assume "yes" for questions
gpg --quiet --batch --yes --decrypt --passphrase="$app_key" --output ~/.config/dfx/identity/admin/identity.pem .github/key.pem.gpg
