#!/bin/bash

# 项目依赖
apt install pkg-config gcc libssl-dev python3.8-venv -y

#rust安装
chmod +x ./rust.sh && ./rust.sh -y
source $HOME/.cargo/env


if [ ! -d "nimble-miner-public" ]; then
    git clone https://github.com/nimble-technology/nimble-miner-public.git;
    sed -ir 's/print\((.*)\)/print(\1,flush=True)/' nimble-miner-public/execute.py
    sed -ir 's/"\\0.*"/""/' nimble-miner-public/execute.py
fi

cd nimble-miner-public

python3 -m venv ./venv
source ./venv/bin/activate

python3 -m pip install --upgrade pip
python3 -m pip install requests==2.31.0 
python3 -m pip install torch==2.2.1
python3 -m pip install accelerate==0.27.0 
python3 -m pip install transformers==4.38.1 
python3 -m pip install datasets==2.17.1 
python3 -m pip install numpy
python3 -m pip install gitpython==3.1.42 
python3 -m pip install prettytable==3.10.0

# python3 execute.py nimble19ds02xkxwfw9l2k8jdlx9ns7s5p0aguxd0v75c

