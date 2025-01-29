# run in terminal

INSTALL_PATH="/usr/bin"
git clone https://github.com/novnc/noVNC.git

sudo cp ./noVNC /usr/bin/noVNC -r

# add to system path
nvm install --lts 
nvm use --lts


# move 
/usr/bin/noVNC/utils/novnc_proxy --vnc localhost:443