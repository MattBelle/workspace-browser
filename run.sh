#!/usr/bin/env bash
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
script=$(mktemp $parent_path/script.XXXXXXXXXXX)
$parent_path/target/release/workspace -o $script
source $script
rm -f $script
