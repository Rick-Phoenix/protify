#!/bin/bash

set -eo pipefail

EXEC_RELEASE=false
if [[ "${2:-}" == "--execute" ]]; then
	EXEC_RELEASE=true
fi
VERSION="$1"

if [[ -z "$VERSION" ]]; then
	echo "Missing new version"
	exit 1
fi

if [[ "$EXEC_RELEASE" = true ]]; then
	echo "Starting pre-release process for version ${VERSION}..."

	if [[ -n $(git status --porcelain README.md) ]]; then
		echo "Committing the updated readme..."
		git add "README.md"
		git commit -m "chore(release): update README"
	fi

	if [[ "$VERSION" != "0.1.0" ]]; then
		echo "Generating changelog..."
		git cliff --tag "$VERSION" -o "CHANGELOG.md"

		if [[ -n $(git status --porcelain "CHANGELOG.md") ]]; then
			echo "Committing the new changelog..."
			git add "CHANGELOG.md"
			git commit -m "chore(release): update changelog for ${VERSION}"
		fi
	fi
fi
