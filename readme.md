
#网站脚本运行
```bash
apt update -y 
apt install git -y
mkdir -p $HOME/nimble && cd $HOME/nimble
git clone https://github.com/victor-vb/clore.git
cd clore && chmod +x ./env.sh ./run.sh ./rust.sh && bash ./env.sh >> log.txt 2>&1
```