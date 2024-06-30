#!/bin/bash
cd $HOME

apt install build-essential  pkg-config gcc libssl-dev python3.8-venv -y
curl -sL https://deb.nodesource.com/setup_18.x | sudo -E bash - && sudo apt-get install -y nodejs && sudo npm install pm2 -g 

if [ ! -d ~/miniconda3 ];then
    mkdir -p ~/miniconda3
    wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh -O ~/miniconda3/miniconda.sh
    bash ~/miniconda3/miniconda.sh -b -u -p ~/miniconda3
    echo "export PATH=$HOME/miniconda3/bin:$PATH" >> $HOME/.bashrc
    # rm -rf ~/miniconda3/miniconda.sh
fi

source $HOME/.bashrc
case ":${PATH}:" in
    *:"$HOME/miniconda3/bin":*)
        ;;
    *)
    export PATH="$HOME/miniconda3/bin:$PATH"
    ;;
esac

conda init
nimble=`conda info -e | grep nimble`;
if [[ "$nimble" == "" ]];then
    conda create -n nimble python=3.11 -y    
fi

conda activate nimble

# clore目录相关的操作
cd $HOME/clore

# 安装rust
if [ ! -f $HOME/.cargo/env ] ;then
    chmod +x ./erust.sh && ./erust.sh -y
fi


#安装挖矿程序
if [ ! -d nimble-miner-public ];then
    git clone https://github.com/nimble-technology/nimble-miner-public.git
    cd nimble-miner-public
    # sed -ir 's/numpy==1.26.4/numpy/' requirements.txt
    make install
fi


cd $HOME/clore
source $HOME/.cargo/env
cargo  build -r --bin monitor
# 系统初始化时，会运行以下此命令
# 下面是安装时就会自动运行
#!/bin/bash
# cd $HOME
# apt update -y 
# apt install git -y
# # 设置时区
# ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
# echo "export SERVER_ID={server_id}" >> $HOME/.bashrc
# echo "export CARD_NUMBER={card_number}" >> $HOME/.bashrc
echo "export ADDRESS=nimble1ha23ka8jzm63eupcrf0thlhhldu4j6z66shqjl" >> $HOME/.bashrc
source $HOME/.bashrc
# git clone  https://github.com/zlseqx/clore.git >> $HOME/server.txt 2>&1

# cd $HOME/clore
# chmod +x env.sh erust.sh execute.sh && ./env.sh >> $HOME/server.txt 2>&1
# mkdir -p $HOME/clore/logs
# mv $HOME/server.txt $HOME/clore/logs
# # 防止内部被cd,需要切换到clore目录操作
# cd $HOME/clore
# source $HOME/.cargo/env
# source $HOME/.bashrc

# # 激活pm2环境,用pm2管理cargo 监控进程
# conda init
# conda activate nimble
# pm2 start "cargo run -r --bin monitor" --name monitor --log $HOME/clore/monitor.txt