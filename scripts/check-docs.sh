#!/bin/bash
set -uo pipefail

DOCS_DIR="docs/"

if [ "${NO_DOCS_LABEL}" = "true" ]; then
    # 'no docs' label set, so finish successfully
    echo "\"no docs\" label has been set"
    exit 0
else
    # a docs check is required
    # fail if the diff is empty (no changes in docs directory)
    if git diff --quiet "origin/${BASE_REF}" -- "${DOCS_DIR}"; then
        >&2 echo "Changes should be accompanied by documentation updates in the \"docs/\" directory.
This behavior can be overridden by using the \"no docs\" label, which is used for changes
that don't require documentation updates or are purely maintenance-related."
        exit 1
    fi

    echo "The \"docs/\" directory has been updated."
fi
