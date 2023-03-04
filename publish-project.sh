#! /bin/bash

# This script is used to push a new project to the repo

# The first argument is the path to the project directory
function parse_args() {
    if [ $# -ne 1 ]; then
        echo "Usage: $0 <project directory>"
        exit 1
    fi
    PROJECT_DIR=$1
    # The project directory name is kebab-case
    # The project name should be caps on first letter of
    # each word with spaces
    PROJECT_NAME=$(
      echo $PROJECT_DIR \
      | sed 's/-/ /g' \
      | sed 's/\b\(.\)/\u\1/g'
    )
}

# This function checks out the local branch and pushes it to
# the remote
function checkout_and_commit() {
    git checkout -b $PROJECT_DIR
    # Add the project directory to the repo
    git add $PROJECT_DIR
    git commit -m "Added example: $PROJECT_NAME"
    git push origin $PROJECT_DIR
}

# This function uses gh to create a pull request for the
# project and merges it to origin/main by rebasing.
# It uses the gh option for deleting the branch after
# merging.
function create_and_merge_pr() {
  # Create the pr and capture the pr id in a variable
  # The last line in the output will be something like:
  #  https://github.com/spigelli/rust-practice/pull/1
  # We want to capture the number at the end of the line
  gh pr create \
    --title "Added example: $PROJECT_NAME" \
    --body "Added example: $PROJECT_NAME" \
    --base main \
    --head $PROJECT_DIR

  # Merge the pr and delete the branch
  gh pr merge $PROJECT_DIR \
    --delete-branch \
    --rebase
}

# Call the functions
parse_args $@
checkout_and_commit
create_and_merge_pr
