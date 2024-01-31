# _prelude.sh
# Commonly used shell utilities and functions, shared across scripts.

# Deal with errors
set -euo pipefail

# Color definitions
DEFAULT='\033[0m'
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
GREY='\033[0;90m'

# NOTE: This is quite noisy so turning off by default
# Coloring the -x output (commands)
#DEBUG_COLOR="${DEFAULT}"
#trap 'echo -e ${DEBUG_COLOR}${BASH_COMMAND}${DEFAULT}' DEBUG

# Function to handle exit
handle_exit() {
    # $? is a special variable that holds the exit code of the last command executed
    if [ $? -ne 0 ]; then
        echo -e "\n${RED}Script did not finish successfully!${DEFAULT}"
    fi
}

# Set the trap
trap handle_exit EXIT

print_action() {
    printf "\n${GREEN}$1${DEFAULT}\n"
}

print_warning() {
    printf "\n${YELLOW}$1${DEFAULT}\n"
}

# Check if toml-cli is installed
if ! command -v toml &> /dev/null; then
    echo -e "${RED}toml (toml-cli) is not installed. Please install it to continue.${DEFAULT}"
    exit 1
fi

# Function to read value from TOML file and remove quotes
read_toml() {
    toml get "$1" "$2" | tr -d '"'
}