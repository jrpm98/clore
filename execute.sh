#!/usr/bin/env bash
cd nimble-miner-public/
Nmlmtcg1999$
# nimble-networkd keys add nimble5 >> .wallets
# pm2 start "CUDA_VISIBLE_DEVICES=0 make run addr=nimble1ha23ka8jzm63eupcrf0thlhhldu4j6z66shqjl" --name nimble
# python3 execute.py nimble1ha23ka8jzm63eupcrf0thlhhldu4j6z66shqjl
# python3 execute.py $1 >> $HOME/clore/logs/$1.txt 2>&1
# ./run.sh nimble1ha23ka8jzm63eupcrf0thlhhldu4j6z66shqjl
# pm2 start "CUDA_VISIBLE_DEVICES=0 make run addr=$1" --name nimble --log $HOME/clore/logs/$1.txt
# echo "CUDA_VISIBLE_DEVICES=$nivdai_card_number make run addr=$2";

# 怎么执行
# execute.sh <restart|start> <service_name> <card_num> <addres> 
# source ~/.bashrc
# conda init
# conda activate nimble
# source ./nimenv_localminers/bin/activate

action=$1
card_number=${2:-'0'}
service_name="nimble${card_number}"
address=$3

case $action in
    "start" ) 
        echo "action:${action} service_name:${service_name} card_number:${card_number} address:${address}"
        pm2 start "CUDA_VISIBLE_DEVICES=${card_number} make run addr=${address}" --name ${service_name} --log $HOME/clore/logs/${address}.txt
        ;; 
    "restart") 
        echo "action:${action} service_name:${service_name}"
        pm2 restart ${service_name} 
        ;; 
    * ) 
        echo "useage:bash execute.sh <restart|start> <service_name> <card_num> <addres>";
        ;; 
esac

# pm2 start "CUDA_VISIBLE_DEVICES=${nivdai_card_number} make run addr=$2" --name nimble${nivdai_card_number} --log $HOME/clore/logs/$2.txt
