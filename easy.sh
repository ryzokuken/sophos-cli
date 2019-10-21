#! /bin/bash

read -p 'Username: ' username 
read -sp 'Password: ' password

echo

if [[ -z "$username" || -z "$password" ]]; then
	echo "Please enter details correctly :)"
else
	cargo run $username $password

fi

