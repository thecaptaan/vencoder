if grep -q "Ubuntu" /etc/os-release; then
    echo "This is an Ubuntu system."
else
    echo "This is not an Ubuntu system."
fi
