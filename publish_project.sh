#! /bin/bash

# This script is used to push a new project to the repo

# The first argument is the path to the project directory
function parse_args() {
  # Infer the kebab-case branch name from the current branch
  CURRENT_BRANCH="$(git branch --show-current)"
  # The project directory name i
  PROJECT_DIR=$(echo -e "./$CURRENT_BRANCH/" | sed "s/-/_/g")
  # The project name is the branch name capitalized with spaces
  PROJECT_NAME=$(echo -e "$CURRENT_BRANCH" \
    | sed 's/-/ /g' \
    | sed 's/\.//g' \
    | sed 's/\///g' \
    `# Capitalize the first letter of each word` \
    | awk '{\
      for(i=1;i<=NF;i++) \
      sub(/./,toupper(substr($i,1,1)),$i) \
    }1')

  echo -e "\e[1;32mProject name: $PROJECT_NAME\e[0m"
  echo -e "\e[1;32mProject directory: $PROJECT_DIR\e[0m"
  echo -e "\e[1;32mBranch name: $CURRENT_BRANCH\e[0m"
}

# This function checks out the local branch and pushes it to
# the remote
function checkout_and_commit() {
  `# Add the project directory to the repo` \
  git add $PROJECT_DIR && \
  git add Cargo.toml && \
  git add Cargo.lock && \
  git add .vscode/launch.json && \
  `# Commit the changes` \
  git commit -m "Added example: $PROJECT_NAME" && \
  echo -e "\e[1;32mSuccessfully committed $PROJECT_DIR on branch: $CURRENT_BRANCH \e[0m" && \
  `# Push the branch to the remote` \
  git push origin $CURRENT_BRANCH && \
  echo -e "\e[1;32mSuccessfully pushed $PROJECT_DIR on branch: $CURRENT_BRANCH \e[0m" || \
  `# Output an error message if the push fails and exit` && \
  ( \
    echo -e "\e[1;31mFailed to push $PROJECT_DIR on branch: $CURRENT_BRANCH \e[0m" && \
    exit 1 \
  )
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
    --head $CURRENT_BRANCH && \
  echo -e "\e[1;32mSuccessfully published $PROJECT_NAME\e[0m" && \
  `# Merge the pr and delete the branch` \
  gh pr merge $CURRENT_BRANCH \
    --delete-branch \
    --rebase && \
  echo -e "\e[1;32mSuccessfully merged $PROJECT_NAME\e[0m" || \
  ( \
    echo -e "\e[1;31mFailed to merge $PROJECT_NAME\e[0m" && \
    `# Exit the script if the merge fails` \
    exit 1 \
  )
}

# Call the functions
parse_args $@ && \
checkout_and_commit && \
create_and_merge_pr
