
#! /bin/bash

# This script is used to create a new project directory

# # Take the first argument and make it the project name
function make_project_name() {
  echo $1 \
  | sed 's/-/ /g' \
  | sed 's/_/ /g' \
  | sed 's/\.//g' \
  | sed 's/\///g' \
  `# Capitalize the first letter of each word` \
  | awk '{\
    for(i=1;i<=NF;i++) \
    sub(/./,toupper(substr($i,1,1)),$i) \
  }1'
}

# ./hello cargo/
# The first argument is the path to the project directory
function parse_args() {
  if [ $# -ne 1 ]; then
      echo "Usage: $0 <project directory>"
      exit 1
  fi
  PROJECT_DIR=$1
}

# If the current directory is not the root of the repo
# print an error message and exit
function check_repo_root() {
  if [ ! -d .git ]; then
    echo -e "\e[1;31mError: This script must be run from the root of the repo\e[0m"
    exit 1
  fi
}

# Create the cargo project
function create_cargo_project() {
  cargo new $PROJECT_DIR --bin && \
  echo -e "\e[1;32mSuccessfully created cargo project: $PROJECT_DIR\e[0m" || \
  ( \
    echo -e "\e[1;31mFailed to create cargo project: $PROJECT_DIR\e[0m" && \
    exit 1 \
  )
} 

### Insert a ldd 


# Construct vscode run configuration
function construct_run_config(){
  echo -e "    {
      \"name\": \"Launch $(make_project_name $PROJECT_DIR)\",
      \"type\": \"lldb\",
      \"request\": \"launch\",
      \"program\": \"\${workspaceRoot}/$PROJECT_DIR/target/debug/$PROJECT_DIR\",
      \"args\": [],
      \"cwd\": \"\${workspaceRoot}/$PROJECT_DIR\"
    }"
}

# Add the run configuration to the .vscode/launch.json file
# under the configurations array
function add_run_config() {
  RUN_CONFIG="$(construct_run_config)"
  # Split the launch.json file into two vars based on where
  # the configurations array ends
  EXISTING_CONFIG_END="$(cat .vscode/launch.json | sed -n '/"configurations": \[/,$p' | tail -n +2)"
  # Everything up to and including the configurations array
  EXISTING_CONFIG_START="$(cat .vscode/launch.json | sed -n '1,/"configurations": \[/p')"

  # Write the new launch.json file
  echo -e "$EXISTING_CONFIG_START
$RUN_CONFIG,
$EXISTING_CONFIG_END
" > .vscode/launch.json && \
  echo -e "\e[1;32mSuccessfully added run configuration to .vscode/launch.json\e[0m" || \
  ( \
    echo -e "\e[1;31mFailed to add run configuration to .vscode/launch.json\e[0m" && \
    exit 1 \
  )
}

# Add the project dir to the members array in the Cargo.toml file at the root of the repo
function add_project_to_cargo_toml() {
  # Split the Cargo.toml file into two vars based on where
  # the members array ends
  MEMBERS_END="$(cat Cargo.toml | sed -n '/members = \[/,$p' | tail -n +2)"
  # Everything up to and including the members array
  MEMBERS_START="$(cat Cargo.toml | sed -n '1,/"members": \[/p')"

  # Write the new Cargo.toml file
  echo -e "$MEMBERS_START
  \"$PROJECT_DIR\",
$MEMBERS_END" > Cargo.toml && \
  echo -e "\e[1;32mSuccessfully added project to Cargo.toml\e[0m" || \
  ( \
    echo -e "\e[1;31mFailed to add project to Cargo.toml\e[0m" && \
    exit 1 \
  )
}

parse_args $@ && \
check_repo_root && \
create_cargo_project && \
add_run_config && \
add_project_to_cargo_toml && \
git checkout -b $(sed 's/_/-/g' <<< $PROJECT_DIR) && \
cd $PROJECT_DIR && \
cargo build && \
cd - 
