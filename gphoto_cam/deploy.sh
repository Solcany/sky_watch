PI_USER='m'
PI_IP=192.168.1.198 # Be sure to change this!
PI_PW='brum35'
#TARGET=armv7-unknown-linux-gnueabihf # Pi 2/3/4
#TARGET=arm-unknown-linux-gnueabihf # Pi 0/1

# build binary
#cargo build --target $TARGET

# upload binary
sshpass -p $PI_PW scp -r ../gphoto_cam $PI_USER@$PI_IP:~/Documents/sky_watch/gphoto_cam

# execute binary
#sshpass -p $PI_PW ssh PI_USER@$PI_IP './pi_project'
