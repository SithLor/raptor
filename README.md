git clone https://github.com/tpn/winsdk-10.git


https://github.com/immunant/c2rust

install rust from the website

  sudo apt update
  sudo apt upgrade  
  sudo apt install build-essential llvm clang libclang-dev cmake libssl-dev pkg-config python3 git
  cargo install c2rust
c2rust transpile winsdk10/Include/10.0.16299.0/*/* --translate-const-macros --reduce-type-annotations --translate-fn-macros -o ./r