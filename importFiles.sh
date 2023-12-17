#!/bin/bash

# Define colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Read list of file paths and names from files.txt
while IFS=';' read -r file file_folder; do
    file_all=$(eval echo "$file")
    folder_all=$(eval echo "$file_folder")
    file_name=$(basename "$file_all")
  
  # Check if file exists in specified path
  if [ -f "$file_all" ]; then
    # Create backupFiles directory if it doesn't exist
    mkdir -p ./backupFiles
    # Create directory structure in backupFiles directory
    mkdir -p "./backupFiles/$folder_all" 
    # copy file to backupFiles directory if it has been modified
    if rsync --itemize-changes --update "$file_all" "./backupFiles/$folder_all" | grep -q ">f"; then
      echo -e " [${GREEN}OK${NC}] Copied $file_name to ./backupFiles/$folder_all"
    else 
      echo -e " [${GREEN}OK${NC}] $file_name is up to date"
    fi
  else
    echo -e " [${RED}ERROR${NC}] $file_name does not exist"
  fi
done < ./backupFiles/files.txt

