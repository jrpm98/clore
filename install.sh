apt-get update -y && apt-get upgrade -y && apt install build-essential

curl -sL https://deb.nodesource.com/setup_18.x | sudo -E bash - && sudo apt-get install -y nodejs && sudo npm install pm2 -g 

sudo rm -rf /usr/local/go
curl https://dl.google.com/go/go1.22.1.linux-amd64.tar.gz | sudo tar -C/usr/local -zxvf - ;
cat <<'EOF' >>$HOME/.bashrc
export GOROOT=/usr/local/go
export GOPATH=$HOME/go
export GO111MODULE=on
export PATH=$PATH:/usr/local/go/bin:$HOME/go/bin
EOF
source $HOME/.bashrc

cd
mkdir -p ~/miniconda3
wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh -O ~/miniconda3/miniconda.sh
bash ~/miniconda3/miniconda.sh -b -u -p ~/miniconda3
rm -rf ~/miniconda3/miniconda.sh
~/miniconda3/bin/conda init bash
source $HOME/.bashrc



conda create -n nimble python=3.11 -y
conda activate nimble

mkdir $HOME/nimble && cd $HOME/nimble

git clone https://github.com/nimble-technology/wallet-public.git

cd wallet-public

make install

cd  $HOME/nimble
git clone https://github.com/b5prolh/nimble-miner-public.git
cd nimble-miner-public
make install
source ./nimenv_localminers/bin/activate


cd && cd $HOME/nimble && cd wallet-public
./nimble-networkd keys add nimenv_localminers
